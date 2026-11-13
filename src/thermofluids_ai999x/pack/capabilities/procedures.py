"""Thermofluids procedures: algorithms and correlations that a closed-form relation cannot hold (an implicit equation,
an iteration, a case split, a property correlation the reader cannot invert). Enabled only once
tools/certify_procedures.py has certified them.

Conventions (fixed, so run and reference agree exactly):
- Darcy friction factor (not Fanning); relative roughness eD = absolute roughness / diameter.
- Saturation pressure of water: the ASHRAE (Hyland-Wexler) correlation, over ice at or below the triple point (0.01 °C)
  and over liquid water above it, valid from -100 °C to 200 °C; moist air as an ideal-gas mixture with the ratio of
  molar masses 0.621945. Relative humidity is a fraction (0.6) or a percentage written with % (60%); a bare number
  above 1 is refused.
- Atomic weights H 1.008, C 12.011, O 15.999, N 14.007 (those of the cantera gri30 mechanism); air is O2 + 3.76 N2.
- Heat-exchanger LMTD from the two terminal temperature differences; equal differences give that difference.

Question form (one per procedure): the capability phrase, then `name = value` pairs (`Re = 100000, eD = 0.0002`;
temperatures and pressures with a unit, `T = 25 °C`, `P = 101325 Pa`). Any other number or an unknown name is refused.
"""
from __future__ import annotations

import math
import re

_NUM = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?"
_PAIR = re.compile(r"\b([A-Za-z][A-Za-z_0-9]*)\s*=\s*(" + _NUM + r")\s*(kPa|MPa|bar|Pa|atm|W/\(m\^2\*K\)|W/\(m\*K\)|W/K|°\s*C|degC|K|mm|m|%|\b)?",
                   re.I)


def _pairs(question: str, phrase: str, names: dict, units: dict | None = None):
    """{name: value} from 'phrase ... name = value, ...'; None unless the phrase is present and every number belongs to a
    known name. `names` maps accepted spellings (lower case) to input symbols; `units` lists the symbols that carry a unit
    (returned as 'value unit' strings, read by the runtime)."""
    q = question.strip()
    if not re.search(phrase, q, re.I):
        return None
    out, rest = {}, q
    for m in _PAIR.finditer(q):
        key = names.get(m.group(1).lower())
        if key is None or key in out:
            return None
        unit = (m.group(3) or "").replace(" ", "")
        if units and key in units:
            if not unit or unit == "%":
                return None
            out[key] = f"{m.group(2)} {'°C' if unit.lower() in ('°c', 'degc') else unit}"
        elif unit == "%":
            out[key] = float(m.group(2)) / 100.0
        elif unit:
            return None
        else:
            out[key] = float(m.group(2))
        rest = rest.replace(m.group(0), " ", 1)
    if re.search(r"\d", re.sub(phrase, " ", rest, flags=re.I)):
        return None                                   # a number that is not a 'name = value' pair: never guessed
    return out


# ------------------------------------------------------------------------------------------------ pipe friction
def colebrook(Re, eD) -> dict:
    if not (math.isfinite(Re) and Re >= 4000):
        raise ValueError("the Colebrook equation is for turbulent flow: Re must be at least 4000")
    if not (0 <= eD <= 0.05):
        raise ValueError("relative roughness eD must lie between 0 and 0.05")
    x = 1 / math.sqrt(0.25 / math.log10(eD / 3.7 + 5.74 / Re ** 0.9) ** 2)      # start from Swamee-Jain
    for _ in range(100):                                                         # Newton on g(x) = x + 2 log10(a + b x)
        a, b = eD / 3.7, 2.51 / Re
        g = x + 2 * math.log10(a + b * x)
        dg = 1 + 2 * b / ((a + b * x) * math.log(10))
        step = g / dg
        x -= step
        if abs(step) <= 1e-15 * abs(x):
            break
    return {"f": 1 / (x * x)}


def verify_colebrook(i, o) -> float:
    f = o["f"]
    if not (isinstance(f, float) and f > 0):
        return math.inf
    lhs = 1 / math.sqrt(f)
    return abs(lhs + 2 * math.log10(i["eD"] / 3.7 + 2.51 / (i["Re"] * math.sqrt(f)))) / lhs


def parse_colebrook(q):
    if not re.search(r"colebrook", q, re.I):
        return None
    return _pairs(q, r"colebrook(?: friction factor| equation)?", {"re": "Re", "ed": "eD"})


# ------------------------------------------------------------------------------------------------ heat exchangers
def _lm(d1, d2):
    if d1 <= 0 or d2 <= 0:
        raise ValueError("each terminal temperature difference must be positive (the streams' temperatures cross)")
    return d1 if abs(d1 - d2) <= 1e-12 * max(d1, d2) else (d1 - d2) / math.log(d1 / d2)


def lmtd(Th_in, Th_out, Tc_in, Tc_out, arrangement) -> dict:
    if Th_out > Th_in or Tc_out < Tc_in:
        raise ValueError("the hot stream must cool and the cold stream must warm")
    if arrangement == "counterflow":
        d1, d2 = Th_in - Tc_out, Th_out - Tc_in
    else:
        d1, d2 = Th_in - Tc_in, Th_out - Tc_out
    return {"lmtd": _lm(d1, d2), "dT1": d1, "dT2": d2}


def verify_lmtd(i, o) -> float:
    d1, d2, L = o["dT1"], o["dT2"], o["lmtd"]
    if i["arrangement"] == "counterflow":
        e1, e2 = i["Th_in"] - i["Tc_out"], i["Th_out"] - i["Tc_in"]
    else:
        e1, e2 = i["Th_in"] - i["Tc_in"], i["Th_out"] - i["Tc_out"]
    r = abs(d1 - e1) / abs(e1) + abs(d2 - e2) / abs(e2)
    if abs(d1 - d2) <= 1e-12 * max(d1, d2):
        return r + abs(L - d1) / d1
    return r + abs(L * math.log(d1 / d2) - (d1 - d2)) / abs(d1 - d2)


_T_UNITS = {"th_in": "Th_in", "th_out": "Th_out", "tc_in": "Tc_in", "tc_out": "Tc_out"}


def parse_lmtd(q):
    m = re.search(r"\b(counterflow|counter-flow|parallel[- ]flow|parallel)\b", q, re.I)
    if not re.search(r"\blmtd\b|log[- ]mean temperature difference", q, re.I) or not m:
        return None
    phrase = r"(?:lmtd|log[- ]mean temperature difference)|(?:counterflow|counter-flow|parallel[- ]flow|parallel)"
    out = _pairs(q, phrase, _T_UNITS, units=set(_T_UNITS.values()))
    if out is None:
        return None
    out["arrangement"] = "counterflow" if m.group(1).lower().startswith("counter") else "parallel"
    return out


def eps_ntu(NTU, Cr, arrangement):
    if arrangement == "counterflow":
        if abs(1 - Cr) < 1e-12:
            return NTU / (1 + NTU)
        e = math.exp(-NTU * (1 - Cr))
        return (1 - e) / (1 - Cr * e)
    return (1 - math.exp(-NTU * (1 + Cr))) / (1 + Cr)


def hx_rating(C_hot, C_cold, UA, Th_in, Tc_in, arrangement) -> dict:
    if min(C_hot, C_cold, UA) <= 0:
        raise ValueError("heat capacity rates and UA must be positive")
    if Th_in <= Tc_in:
        raise ValueError("the hot inlet must be hotter than the cold inlet")
    Cmin, Cmax = min(C_hot, C_cold), max(C_hot, C_cold)
    NTU, Cr = UA / Cmin, Cmin / Cmax
    eps = eps_ntu(NTU, Cr, arrangement)
    Q = eps * Cmin * (Th_in - Tc_in)
    return {"Q": Q, "Th_out": Th_in - Q / C_hot, "Tc_out": Tc_in + Q / C_cold, "effectiveness": eps, "NTU": NTU}


def verify_hx(i, o) -> float:
    Cmin, Cmax = min(i["C_hot"], i["C_cold"]), max(i["C_hot"], i["C_cold"])
    eps = eps_ntu(i["UA"] / Cmin, Cmin / Cmax, i["arrangement"])
    Qmax = Cmin * (i["Th_in"] - i["Tc_in"])
    return (abs(o["effectiveness"] - eps) / eps + abs(o["Q"] - eps * Qmax) / Qmax
            + abs(i["C_hot"] * (i["Th_in"] - o["Th_out"]) - o["Q"]) / Qmax + abs(i["C_cold"] * (o["Tc_out"] - i["Tc_in"]) - o["Q"]) / Qmax
            + abs(o["NTU"] - i["UA"] / Cmin) / o["NTU"])


def parse_hx(q):
    m = re.search(r"\b(counterflow|counter-flow|parallel[- ]flow)\b", q, re.I)
    if not re.search(r"effectiveness[- ]ntu|\bntu method\b", q, re.I) or not m:
        return None
    names = {"c_hot": "C_hot", "c_cold": "C_cold", "ua": "UA", "th_in": "Th_in", "tc_in": "Tc_in"}
    out = _pairs(q, r"effectiveness[- ]ntu(?: rating| method)?|ntu method|counterflow|counter-flow|parallel[- ]flow", names,
                 units={"C_hot", "C_cold", "UA", "Th_in", "Tc_in"})
    if out is None:
        return None
    out["arrangement"] = "counterflow" if m.group(1).lower().startswith("counter") else "parallel"
    return out


# ------------------------------------------------------------------------------------------------ psychrometrics
def pws(T) -> float:
    """Saturation pressure of water vapour (Pa) at T (K): ASHRAE Handbook (Hyland-Wexler) correlation."""
    t = T - 273.15
    if not (-100 <= t <= 200):
        raise ValueError("temperature must lie between -100 °C and 200 °C")
    if t <= 0.01:
        ln = (-5.6745359e3 / T + 6.3925247 - 9.677843e-3 * T + 6.2215701e-7 * T ** 2 + 2.0747825e-9 * T ** 3
              - 9.484024e-13 * T ** 4 + 4.1635019 * math.log(T))
    else:
        ln = -5.8002206e3 / T + 1.3914993 - 4.8640239e-2 * T + 4.1764768e-5 * T ** 2 - 1.4452093e-8 * T ** 3 + 6.5459673 * math.log(T)
    return math.exp(ln)


def sat_pressure(T) -> dict:
    return {"p_sat": pws(T)}


def _rh(RH):
    if not (0 < RH <= 1):
        raise ValueError("relative humidity must be a fraction between 0 and 1 (or a percentage written with %)")


def _solve_T(fun, lo, hi):
    """Bisection to the last representable digit: fun increasing, fun(lo) < 0 < fun(hi)."""
    flo = fun(lo)
    for _ in range(200):
        mid = 0.5 * (lo + hi)
        if mid in (lo, hi):
            break
        fm = fun(mid)
        if (fm < 0) == (flo < 0):
            lo, flo = mid, fm
        else:
            hi = mid
    return 0.5 * (lo + hi)


def dew_point(T, RH) -> dict:
    _rh(RH)
    pv = RH * pws(T)
    Td = T if RH == 1 else _solve_T(lambda x: pws(x) - pv, 173.15, T)
    return {"T_dew": Td, "p_v": pv}


def verify_dew(i, o) -> float:
    pv = i["RH"] * pws(i["T"])
    return abs(pws(o["T_dew"]) - pv) / pv + abs(o["p_v"] - pv) / pv


def hum_ratio_rh(T, RH, P) -> dict:
    _rh(RH)
    pv = RH * pws(T)
    if not P > pv:
        raise ValueError("the total pressure must exceed the vapour pressure")
    return {"W": 0.621945 * pv / (P - pv), "p_v": pv}


def verify_hum(i, o) -> float:
    pv = i["RH"] * pws(i["T"])
    return abs(o["W"] * (i["P"] - pv) - 0.621945 * pv) / (0.621945 * pv) + abs(o["p_v"] - pv) / pv


def _w_from_wb(T, Twb, P):
    """ASHRAE psychrometric equation: humidity ratio from the dry-bulb and wet-bulb temperatures (°C forms)."""
    t, tw = T - 273.15, Twb - 273.15
    ps = pws(Twb)
    ws = 0.621945 * ps / (P - ps)
    if tw >= 0:
        return ((2501 - 2.326 * tw) * ws - 1.006 * (t - tw)) / (2501 + 1.86 * t - 4.186 * tw)
    return ((2830 - 0.24 * tw) * ws - 1.006 * (t - tw)) / (2830 + 1.86 * t - 2.1 * tw)


def wet_bulb(T, RH, P) -> dict:
    W = hum_ratio_rh(T, RH, P)["W"]
    Td = dew_point(T, RH)["T_dew"]
    Twb = T if RH == 1 else _solve_T(lambda x: _w_from_wb(T, x, P) - W, Td - 1e-9, T)
    return {"T_wb": Twb, "W": W}


def verify_wb(i, o) -> float:
    W = hum_ratio_rh(i["T"], i["RH"], i["P"])["W"]
    return abs(_w_from_wb(i["T"], o["T_wb"], i["P"]) - W) / W + abs(o["W"] - W) / W


_PSY_T = {"t": "T", "tdb": "T", "t_db": "T"}
_PSY = {**_PSY_T, "rh": "RH", "phi": "RH", "p": "P"}


def parse_psy(phrase, needs_p):
    def parse(q):
        if not re.search(phrase, q, re.I):
            return None
        names = _PSY if needs_p else {**_PSY_T, "rh": "RH", "phi": "RH"}
        return _pairs(q, phrase, names, units={"T", "P"} if needs_p else {"T"})
    return parse


def parse_sat(q):
    phrase = r"saturation (?:vapou?r )?pressure(?: of water)?"
    if not re.search(phrase, q, re.I):
        return None
    return _pairs(q, phrase, _PSY_T, units={"T"})


# ------------------------------------------------------------------------------------------------ combustion
AW = {"H": 1.008, "C": 12.011, "O": 15.999, "N": 14.007}
FUELS = {"methane": ("CH4", 1, 4), "ethane": ("C2H6", 2, 6), "propane": ("C3H8", 3, 8), "ethylene": ("C2H4", 2, 4),
         "acetylene": ("C2H2", 2, 2), "hydrogen": ("H2", 0, 2)}
AIR_PER_O2 = 2 * AW["O"] + 3.76 * 2 * AW["N"]


def stoich(fuel) -> dict:
    _sp, x, y = FUELS[fuel]
    o2 = x + y / 4
    return {"afr_stoich": o2 * AIR_PER_O2 / (x * AW["C"] + y * AW["H"]), "o2_moles": o2, "air_moles": 4.76 * o2}


def verify_stoich(i, o) -> float:
    _sp, x, y = FUELS[i["fuel"]]
    o2 = o["o2_moles"]
    # atom balances of CxHy + o2 (O2 + 3.76 N2) -> x CO2 + y/2 H2O + 3.76 o2 N2, then the mass ratio
    r = abs(2 * o2 - (2 * x + y / 2)) / (2 * o2) + abs(o["air_moles"] - 4.76 * o2) / o["air_moles"]
    mf = x * AW["C"] + y * AW["H"]
    return r + abs(o["afr_stoich"] * mf - o2 * AIR_PER_O2) / (o2 * AIR_PER_O2)


def equivalence(fuel, afr) -> dict:
    if not afr > 0:
        raise ValueError("the air-fuel ratio must be positive")
    s = stoich(fuel)["afr_stoich"]
    return {"phi": s / afr, "afr_stoich": s, "excess_air": afr / s - 1}


def verify_equiv(i, o) -> float:
    s = stoich(i["fuel"])["afr_stoich"]
    return abs(o["phi"] * i["afr"] - s) / s + abs(o["afr_stoich"] - s) / s + abs((o["excess_air"] + 1) * s - i["afr"]) / i["afr"]


def _fuel_of(q):
    hits = [f for f in FUELS if re.search(r"\b" + f + r"\b", q, re.I)]
    return hits[0] if len(hits) == 1 else None


def parse_stoich(q):
    if not re.search(r"stoichiometric air[- ]fuel ratio", q, re.I) or re.search(r"equivalence ratio", q, re.I):
        return None
    f = _fuel_of(q)
    if f is None or re.search(r"\d", re.sub(r"\b[CH]\d+H?\d*\b", " ", q)):
        return None
    return {"fuel": f}


def parse_equiv(q):
    if not re.search(r"equivalence ratio", q, re.I):
        return None
    f = _fuel_of(q)
    out = _pairs(q, r"equivalence ratio", {"afr": "afr"})
    if f is None or out is None:
        return None
    return {"fuel": f, **out}


# ------------------------------------------------------------------------------------------------ correlations
def gnielinski(Re, Pr, f) -> dict:
    if not (3000 <= Re <= 5e6 and 0.5 <= Pr <= 2000 and 0 < f < 0.2):
        raise ValueError("Gnielinski holds for 3000 <= Re <= 5e6, 0.5 <= Pr <= 2000 and a Darcy friction factor 0 < f < 0.2")
    return {"Nu": (f / 8) * (Re - 1000) * Pr / (1 + 12.7 * math.sqrt(f / 8) * (Pr ** (2 / 3) - 1))}


def verify_gn(i, o) -> float:
    f, Re, Pr = i["f"], i["Re"], i["Pr"]
    return abs(o["Nu"] * (1 + 12.7 * math.sqrt(f / 8) * (Pr ** (2 / 3) - 1)) - (f / 8) * (Re - 1000) * Pr) / ((f / 8) * (Re - 1000) * Pr)


def churchill_bernstein(Re, Pr) -> dict:
    if not (Re > 0 and Pr > 0 and Re * Pr >= 0.2):
        raise ValueError("Churchill-Bernstein holds for Re * Pr >= 0.2")
    return {"Nu": 0.3 + 0.62 * Re ** 0.5 * Pr ** (1 / 3) / (1 + (0.4 / Pr) ** (2 / 3)) ** 0.25 * (1 + (Re / 282000) ** 0.625) ** 0.8}


def verify_cb(i, o) -> float:
    Re, Pr = i["Re"], i["Pr"]
    core = 0.62 * Re ** 0.5 * Pr ** (1 / 3) / (1 + (0.4 / Pr) ** (2 / 3)) ** 0.25 * (1 + (Re / 282000) ** 0.625) ** 0.8
    return abs(o["Nu"] - 0.3 - core) / (0.3 + core)


def churchill_chu(Ra, Pr) -> dict:
    if not (0 < Ra <= 1e13 and Pr > 0):
        raise ValueError("Churchill-Chu holds for 0 < Ra <= 1e13 and Pr > 0")
    s = 0.825 + 0.387 * Ra ** (1 / 6) / (1 + (0.492 / Pr) ** (9 / 16)) ** (8 / 27)
    return {"Nu": s * s}


def verify_cc(i, o) -> float:
    s = 0.825 + 0.387 * i["Ra"] ** (1 / 6) / (1 + (0.492 / i["Pr"]) ** (9 / 16)) ** (8 / 27)
    return abs(math.sqrt(o["Nu"]) - s) / s if o["Nu"] > 0 else math.inf


def pin_fin(h, k, D, L) -> dict:
    if min(h, k, D, L) <= 0:
        raise ValueError("h, k, D and L must be positive")
    m = math.sqrt(4 * h / (k * D))
    mL = m * L
    eta = math.tanh(mL) / mL
    return {"efficiency": eta, "mL": mL}


def verify_fin(i, o) -> float:
    mL = math.sqrt(4 * i["h"] / (i["k"] * i["D"])) * i["L"]
    return abs(o["efficiency"] * mL - math.tanh(mL)) / math.tanh(mL) + abs(o["mL"] - mL) / mL


# ------------------------------------------------------------------------------------------------ references (oracles)
def ref_colebrook(Re, eD):
    import fluids.friction
    return {"f": fluids.friction.Colebrook(Re, eD)}


def ref_lmtd(Th_in, Th_out, Tc_in, Tc_out, arrangement):
    import ht
    return {"lmtd": ht.LMTD(Th_in, Th_out, Tc_in, Tc_out, counterflow=arrangement == "counterflow")}


def ref_hx(C_hot, C_cold, UA, Th_in, Tc_in, arrangement):
    import ht
    r = ht.hx.effectiveness_NTU_method(mh=1.0, mc=1.0, Cph=C_hot, Cpc=C_cold, subtype=arrangement, Thi=Th_in, Tci=Tc_in, UA=UA)
    return {"Q": r["Q"], "Th_out": r["Tho"], "Tc_out": r["Tco"], "effectiveness": r["effectiveness"], "NTU": r["NTU"]}


def _psy():
    import psychrolib
    psychrolib.SetUnitSystem(psychrolib.SI)
    return psychrolib


def ref_sat(T):
    return {"p_sat": _psy().GetSatVapPres(T - 273.15)}


def ref_dew(T, RH):
    from scipy.optimize import brentq
    p = _psy()
    pv = RH * p.GetSatVapPres(T - 273.15)
    if RH == 1:
        return {"T_dew": T, "p_v": pv}
    td = brentq(lambda t: p.GetSatVapPres(t) - pv, -100.0, T - 273.15, xtol=1e-13, rtol=8.9e-16)
    return {"T_dew": td + 273.15, "p_v": pv}


def ref_hum(T, RH, P):
    p = _psy()
    return {"W": p.GetHumRatioFromRelHum(T - 273.15, RH, P), "p_v": p.GetVapPresFromRelHum(T - 273.15, RH)}


def ref_wb(T, RH, P):
    from scipy.optimize import brentq
    p = _psy()
    W = p.GetHumRatioFromRelHum(T - 273.15, RH, P)
    if RH == 1:
        return {"T_wb": T, "W": W}
    td = p.GetTDewPointFromRelHum(T - 273.15, RH)
    tw = brentq(lambda x: p.GetHumRatioFromTWetBulb(T - 273.15, x, P) - W, td - 0.5, T - 273.15, xtol=1e-13, rtol=8.9e-16)
    return {"T_wb": tw + 273.15, "W": W}


def _cantera_afr(fuel, phi=1.0):
    import cantera as ct
    g = ct.Solution("gri30.yaml")
    g.set_equivalence_ratio(phi, FUELS[fuel][0], "O2:1.0, N2:3.76")
    yf = g.Y[g.species_index(FUELS[fuel][0])]
    return g, (1 - yf) / yf


def ref_stoich(fuel):
    _g, afr = _cantera_afr(fuel)
    _sp, x, y = FUELS[fuel]
    return {"afr_stoich": afr, "o2_moles": x + y / 4}


def ref_equiv(fuel, afr):
    import cantera as ct
    g = ct.Solution("gri30.yaml")
    sp = FUELS[fuel][0]
    mw = {s: g.molecular_weights[g.species_index(s)] for s in (sp, "O2", "N2")}
    air_mw = (mw["O2"] + 3.76 * mw["N2"]) / 4.76                   # mixture mass afr kg air per kg fuel, as moles
    n_air = afr / air_mw
    g.TPX = 300, 101325, {sp: 1.0 / mw[sp], "O2": n_air / 4.76, "N2": n_air * 3.76 / 4.76}
    phi = g.equivalence_ratio(sp, "O2:1.0, N2:3.76")
    return {"phi": phi}


def ref_gn(Re, Pr, f):
    import ht
    return {"Nu": ht.conv_internal.turbulent_Gnielinski(Re, Pr, f)}


def ref_cc(Ra, Pr):
    import ht
    return {"Nu": ht.conv_free_immersed.Nu_vertical_plate_Churchill(Pr, Ra / Pr)}


def ref_fin(h, k, D, L):
    """Fin equation theta'' = m^2 theta with an insulated tip, shot with scipy.linalg.expm; efficiency q / q_max."""
    import numpy as np
    from scipy.linalg import expm
    m2 = 4 * h / (k * D)
    E = expm(np.array([[0.0, 1.0], [m2, 0.0]]) * L)
    s = -E[1, 0] / E[1, 1]                                            # base slope with theta_b = 1
    Ac, P = math.pi * D * D / 4, math.pi * D
    return {"efficiency": -k * Ac * s / (h * P * L)}


# ------------------------------------------------------------------------------------------------ the procedures
def _num_in(sym, meaning, **extra):
    return {"symbol": sym, "kind": "number", "meaning": meaning, "required": True, **extra}


def _phys(sym, kind, meaning):
    return {"symbol": sym, "kind": kind, "meaning": meaning, "required": True}


ARR = {"symbol": "arrangement", "type": "choice", "choices": ["counterflow", "parallel"], "meaning": "flow arrangement", "required": True}


def _log_uniform(rng, a, b):
    return math.exp(rng.uniform(math.log(a), math.log(b)))


PROCEDURES = {
    "colebrook_friction_factor": {
        "name": "Darcy friction factor from the Colebrook equation", "topic": "pipe flow / friction factor",
        "purpose": "Solve the implicit Colebrook-White equation for the Darcy friction factor of turbulent pipe flow.",
        "inputs": [_num_in("Re", "Reynolds number (turbulent, at least 4000)"), _num_in("eD", "relative roughness (roughness / diameter)")],
        "outputs": [{"symbol": "f", "kind": "darcy_friction_factor", "meaning": "Darcy friction factor"}],
        "primary": "f", "conditions": ["Re >= 4000", "0 <= eD <= 0.05"],
        "method": "Newton iteration on 1/sqrt(f) from a Swamee-Jain start, to machine precision",
        "latex": r"\frac{1}{\sqrt{f}} = -2\log_{10}\left(\frac{\varepsilon/D}{3.7} + \frac{2.51}{Re\sqrt{f}}\right)",
        "run": colebrook, "verify": verify_colebrook, "parse": parse_colebrook, "reference": ref_colebrook,
        "wrong": lambda Re, eD: {"f": 0.25 / math.log10(eD / 3.7 + 5.74 / Re ** 0.9) ** 2 * 1.02},
        "random_inputs": lambda rng: {"Re": _log_uniform(rng, 4000, 1e8), "eD": rng.choice([0.0, _log_uniform(rng, 1e-6, 0.05)])},
        "templates": ["Colebrook friction factor: Re = 100000, eD = 0.0002"],
        "examples": [{"inputs": {"Re": 100000, "eD": 0.0002}, "outputs": {"f": 0.01900543522195957}},
                     {"inputs": {"Re": 1e6, "eD": 0.0}, "outputs": {"f": 0.011645040997991622}}],
        "questions": [{"q": "Colebrook friction factor: Re = 100000, eD = 0.0002", "outputs": {}},
                      {"q": "Colebrook friction factor: Re = 1500, eD = 0.001", "outcome": "INVALID_INPUT"},
                      {"q": "Colebrook friction factor: Re = 100000", "outcome": "NEEDS_INPUT"}]},
    "lmtd": {
        "name": "Log-mean temperature difference of a heat exchanger", "topic": "heat exchangers / LMTD method",
        "purpose": "The LMTD of a counterflow or parallel-flow exchanger from the four terminal temperatures.",
        "inputs": [_phys("Th_in", "temperature", "hot stream inlet temperature"), _phys("Th_out", "temperature", "hot stream outlet temperature"),
                   _phys("Tc_in", "temperature", "cold stream inlet temperature"), _phys("Tc_out", "temperature", "cold stream outlet temperature"), ARR],
        "outputs": [{"symbol": "lmtd", "kind": "temperature_difference", "meaning": "log-mean temperature difference"},
                    {"symbol": "dT1", "kind": "temperature_difference", "meaning": "temperature difference at the hot-inlet end"},
                    {"symbol": "dT2", "kind": "temperature_difference", "meaning": "temperature difference at the other end"}],
        "primary": "lmtd", "conditions": ["both terminal temperature differences positive", "the hot stream cools and the cold stream warms"],
        "method": "(dT1 - dT2) / ln(dT1 / dT2) from the terminal differences of the chosen arrangement",
        "latex": r"\Delta T_{lm} = \frac{\Delta T_1 - \Delta T_2}{\ln(\Delta T_1/\Delta T_2)}",
        "run": lmtd, "verify": verify_lmtd, "parse": parse_lmtd, "reference": ref_lmtd,
        "wrong": lambda Th_in, Th_out, Tc_in, Tc_out, arrangement: (lambda o: {**o, "lmtd": (o["dT1"] + o["dT2"]) / 2})(
            lmtd(Th_in, Th_out, Tc_in, Tc_out, arrangement)),
        "random_inputs": lambda rng: (lambda tci, th: {"Th_in": th, "Th_out": th - rng.uniform(5, 60), "Tc_in": tci,
                                                       "Tc_out": tci + rng.uniform(5, 30), "arrangement": rng.choice(["counterflow", "parallel"])})(
            rng.uniform(280, 320), rng.uniform(400, 500)),
        "templates": ["Counterflow LMTD: Th_in = 150 °C, Th_out = 90 °C, Tc_in = 30 °C, Tc_out = 70 °C"],
        "examples": [{"inputs": {"Th_in": 423.15, "Th_out": 363.15, "Tc_in": 303.15, "Tc_out": 343.15, "arrangement": "counterflow"},
                      "outputs": {"lmtd": 20 / math.log(80 / 60)}},
                     {"inputs": {"Th_in": 423.15, "Th_out": 363.15, "Tc_in": 303.15, "Tc_out": 343.15, "arrangement": "parallel"},
                      "outputs": {"lmtd": 100 / math.log(120 / 20)}}],
        "questions": [{"q": "Counterflow LMTD: Th_in = 150 °C, Th_out = 90 °C, Tc_in = 30 °C, Tc_out = 70 °C", "outputs": {"lmtd": 20 / math.log(80 / 60)}},
                      {"q": "Parallel flow LMTD: Th_in = 150 °C, Th_out = 90 °C, Tc_in = 30 °C, Tc_out = 70 °C", "outputs": {"lmtd": 100 / math.log(6)}},
                      {"q": "Parallel flow LMTD: Th_in = 150 °C, Th_out = 60 °C, Tc_in = 30 °C, Tc_out = 70 °C", "outcome": "INVALID_INPUT"},
                      {"q": "Counterflow LMTD: Th_in = 150 °C, Th_out = 90 °C, Tc_in = 30 °C", "outcome": "NEEDS_INPUT"}]},
    "hx_effectiveness_ntu": {
        "name": "Heat exchanger rating by the effectiveness-NTU method", "topic": "heat exchangers / effectiveness-NTU",
        "purpose": "Heat duty, outlet temperatures, effectiveness and NTU of a counterflow or parallel-flow exchanger.",
        "inputs": [_phys("C_hot", "heat_capacity_rate", "heat capacity rate of the hot stream (m cp)"),
                   _phys("C_cold", "heat_capacity_rate", "heat capacity rate of the cold stream (m cp)"),
                   _phys("UA", "heat_capacity_rate", "overall conductance U A"), _phys("Th_in", "temperature", "hot inlet temperature"),
                   _phys("Tc_in", "temperature", "cold inlet temperature"), ARR],
        "outputs": [{"symbol": "Q", "kind": "power", "meaning": "heat transfer rate"},
                    {"symbol": "Th_out", "kind": "temperature", "meaning": "hot outlet temperature"},
                    {"symbol": "Tc_out", "kind": "temperature", "meaning": "cold outlet temperature"},
                    {"symbol": "effectiveness", "kind": "heat_exchanger_effectiveness", "meaning": "effectiveness"},
                    {"symbol": "NTU", "kind": "number_of_transfer_units", "meaning": "number of transfer units"}],
        "primary": "Q", "conditions": ["positive capacity rates and UA", "hot inlet hotter than cold inlet"],
        "method": "NTU = UA/Cmin, Cr = Cmin/Cmax, effectiveness of the arrangement, Q = eps Cmin (Th,in - Tc,in), energy balances",
        "latex": r"Q = \varepsilon(NTU, C_r)\,C_{min}(T_{h,i} - T_{c,i})",
        "run": hx_rating, "verify": verify_hx, "parse": parse_hx, "reference": ref_hx,
        "wrong": lambda C_hot, C_cold, UA, Th_in, Tc_in, arrangement: hx_rating(C_hot, C_cold, UA, Th_in, Tc_in,
                                                                                "parallel" if arrangement == "counterflow" else "counterflow"),
        "random_inputs": lambda rng: {"C_hot": rng.uniform(500, 5000), "C_cold": rng.uniform(500, 5000), "UA": rng.uniform(500, 8000),
                                      "Th_in": rng.uniform(350, 500), "Tc_in": rng.uniform(280, 330), "arrangement": rng.choice(["counterflow", "parallel"])},
        "templates": ["Effectiveness-NTU rating, counterflow: C_hot = 2000 W/K, C_cold = 4000 W/K, UA = 3000 W/K, Th_in = 150 °C, Tc_in = 30 °C"],
        "examples": [{"inputs": {"C_hot": 2000, "C_cold": 4000, "UA": 3000, "Th_in": 423.15, "Tc_in": 303.15, "arrangement": "counterflow"},
                      "outputs": {"effectiveness": (1 - math.exp(-0.75)) / (1 - 0.5 * math.exp(-0.75))}}],
        "questions": [{"q": "Effectiveness-NTU rating, counterflow: C_hot = 2000 W/K, C_cold = 4000 W/K, UA = 3000 W/K, Th_in = 150 °C, Tc_in = 30 °C",
                       "outputs": {"NTU": 1.5}},
                      {"q": "Effectiveness-NTU rating, parallel flow: C_hot = 2000 W/K, C_cold = 4000 W/K, UA = 3000 W/K, Th_in = 20 °C, Tc_in = 30 °C",
                       "outcome": "INVALID_INPUT"}]},
    "saturation_vapour_pressure": {
        "name": "Saturation pressure of water vapour", "topic": "psychrometrics / saturation",
        "purpose": "The saturation pressure of water vapour over liquid water (above 0.01 °C) or ice, -100 °C to 200 °C.",
        "inputs": [_phys("T", "temperature", "temperature")],
        "outputs": [{"symbol": "p_sat", "kind": "pressure", "meaning": "saturation pressure of water vapour"}],
        "primary": "p_sat", "conditions": ["-100 °C <= T <= 200 °C"],
        "method": "ASHRAE (Hyland-Wexler) correlation for ln p_ws", "latex": r"\ln p_{ws} = C_8/T + C_9 + C_{10}T + C_{11}T^2 + C_{12}T^3 + C_{13}\ln T",
        "run": sat_pressure, "verify": lambda i, o: abs(o["p_sat"] - pws(i["T"])) / pws(i["T"]), "parse": parse_sat, "reference": ref_sat,
        "wrong": lambda T: {"p_sat": 610.94 * math.exp(17.625 * (T - 273.15) / (T - 273.15 + 243.04))},   # Magnus approximation
        "random_inputs": lambda rng: {"T": rng.uniform(173.15, 473.15)},
        "templates": ["Saturation vapour pressure of water: T = 25 °C"],
        "examples": [{"inputs": {"T": 298.15}, "outputs": {"p_sat": pws(298.15)}}, {"inputs": {"T": 373.15}, "outputs": {"p_sat": pws(373.15)}}],
        "questions": [{"q": "Saturation vapour pressure of water: T = 25 °C", "outputs": {"p_sat": pws(298.15)}},
                      {"q": "Saturation vapour pressure of water: T = 300 °C", "outcome": "INVALID_INPUT"},
                      {"q": "Saturation vapour pressure of water at 25 °C and 50 kPa", "outcome": "UNRECOGNIZED_FORMAT"}]},
    "dew_point_temperature": {
        "name": "Dew-point temperature from dry-bulb temperature and relative humidity", "topic": "psychrometrics / dew point",
        "purpose": "The temperature at which moist air of a given relative humidity becomes saturated on cooling.",
        "inputs": [_phys("T", "temperature", "dry-bulb temperature"), _num_in("RH", "relative humidity (fraction, or % written)")],
        "outputs": [{"symbol": "T_dew", "kind": "temperature", "meaning": "dew-point temperature"},
                    {"symbol": "p_v", "kind": "pressure", "meaning": "partial pressure of the water vapour"}],
        "primary": "T_dew", "conditions": ["0 < RH <= 1", "-100 °C <= T <= 200 °C"],
        "method": "p_v = RH p_ws(T), then p_ws(T_dew) = p_v solved by bisection to the last digit",
        "latex": r"p_{ws}(T_{dp}) = \phi\,p_{ws}(T)",
        "run": dew_point, "verify": verify_dew, "parse": parse_psy(r"dew[- ]point(?: temperature)?", False), "reference": ref_dew,
        "wrong": lambda T, RH: (lambda t, g: {"T_dew": 243.04 * g / (17.625 - g) + 273.15, "p_v": RH * pws(T)})(
            T - 273.15, math.log(RH) + 17.625 * (T - 273.15) / (T - 273.15 + 243.04)),
        "random_inputs": lambda rng: {"T": rng.uniform(263.15, 323.15), "RH": rng.uniform(0.1, 0.95)},
        "templates": ["Dew point temperature: T = 25 °C, RH = 60%"],
        "examples": [{"inputs": {"T": 298.15, "RH": 0.6}, "outputs": {"p_v": 0.6 * pws(298.15)}}],
        "questions": [{"q": "Dew point temperature: T = 25 °C, RH = 60%", "outputs": {"p_v": 0.6 * pws(298.15)}},
                      {"q": "Dew point temperature: T = 25 °C, RH = 0.6", "outputs": {"p_v": 0.6 * pws(298.15)}},
                      {"q": "Dew point temperature: T = 25 °C, RH = 60", "outcome": "INVALID_INPUT"},
                      {"q": "Dew point temperature: T = 25 °C", "outcome": "NEEDS_INPUT"}]},
    "humidity_ratio_from_relative_humidity": {
        "name": "Humidity ratio from temperature, relative humidity and pressure", "topic": "psychrometrics / humidity ratio",
        "purpose": "Mass of water vapour per kilogram of dry air in moist air of a given relative humidity.",
        "inputs": [_phys("T", "temperature", "dry-bulb temperature"), _num_in("RH", "relative humidity (fraction, or % written)"),
                   _phys("P", "pressure", "total (barometric) pressure")],
        "outputs": [{"symbol": "W", "kind": "humidity_ratio", "meaning": "humidity ratio (kg/kg dry air)"},
                    {"symbol": "p_v", "kind": "pressure", "meaning": "partial pressure of the water vapour"}],
        "primary": "W", "conditions": ["0 < RH <= 1", "P greater than the vapour pressure"],
        "method": "p_v = RH p_ws(T); W = 0.621945 p_v / (P - p_v)", "latex": r"W = 0.621945\,\frac{\phi p_{ws}}{P - \phi p_{ws}}",
        "run": hum_ratio_rh, "verify": verify_hum, "parse": parse_psy(r"humidity ratio", True), "reference": ref_hum,
        "wrong": lambda T, RH, P: {"W": 0.621945 * RH * pws(T) / P, "p_v": RH * pws(T)},
        "random_inputs": lambda rng: {"T": rng.uniform(273.15, 323.15), "RH": rng.uniform(0.1, 1.0), "P": rng.uniform(7e4, 1.1e5)},
        "templates": ["Humidity ratio: T = 30 °C, RH = 50%, P = 101325 Pa"],
        "examples": [{"inputs": {"T": 303.15, "RH": 0.5, "P": 101325}, "outputs": {"p_v": 0.5 * pws(303.15)}}],
        "questions": [{"q": "Humidity ratio: T = 30 °C, RH = 50%, P = 101325 Pa", "outputs": {"p_v": 0.5 * pws(303.15)}},
                      {"q": "Humidity ratio: T = 30 °C, RH = 50%", "outcome": "NEEDS_INPUT"}]},
    "wet_bulb_temperature": {
        "name": "Thermodynamic wet-bulb temperature", "topic": "psychrometrics / wet bulb",
        "purpose": "The wet-bulb temperature of moist air from its dry-bulb temperature, relative humidity and pressure.",
        "inputs": [_phys("T", "temperature", "dry-bulb temperature"), _num_in("RH", "relative humidity (fraction, or % written)"),
                   _phys("P", "pressure", "total (barometric) pressure")],
        "outputs": [{"symbol": "T_wb", "kind": "temperature", "meaning": "thermodynamic wet-bulb temperature"},
                    {"symbol": "W", "kind": "humidity_ratio", "meaning": "humidity ratio"}],
        "primary": "T_wb", "conditions": ["0 < RH <= 1", "0 °C <= T <= 50 °C for the liquid-water branch"],
        "method": "the ASHRAE psychrometric equation W(T, T_wb, P) = W solved for T_wb by bisection",
        "latex": r"W = \frac{(2501 - 2.326\,t^*)W_s^* - 1.006(t - t^*)}{2501 + 1.86\,t - 4.186\,t^*}",
        "run": wet_bulb, "verify": verify_wb, "parse": parse_psy(r"wet[- ]bulb(?: temperature)?", True), "reference": ref_wb,
        "wrong": lambda T, RH, P: (lambda t: {"T_wb": 273.15 + t * math.atan(0.151977 * math.sqrt(RH * 100 + 8.313659)) + math.atan(t + RH * 100)
                                               - math.atan(RH * 100 - 1.676331) + 0.00391838 * (RH * 100) ** 1.5 * math.atan(0.023101 * RH * 100)
                                               - 4.686035, "W": hum_ratio_rh(T, RH, P)["W"]})(T - 273.15),  # Stull's fit
        "random_inputs": lambda rng: {"T": rng.uniform(283.15, 318.15), "RH": rng.uniform(0.15, 0.95), "P": rng.uniform(8e4, 1.05e5)},
        "templates": ["Wet bulb temperature: T = 30 °C, RH = 40%, P = 101325 Pa"],
        "examples": [{"inputs": {"T": 303.15, "RH": 0.4, "P": 101325}, "outputs": {"W": hum_ratio_rh(303.15, 0.4, 101325)["W"]}}],
        "questions": [{"q": "Wet bulb temperature: T = 30 °C, RH = 40%, P = 101325 Pa", "outputs": {"W": hum_ratio_rh(303.15, 0.4, 101325)["W"]}},
                      {"q": "Wet bulb temperature: T = 30 °C, RH = 140%, P = 101325 Pa", "outcome": "INVALID_INPUT"}]},
    "stoichiometric_air_fuel_ratio": {
        "name": "Stoichiometric air-fuel ratio of a gaseous fuel", "topic": "combustion / stoichiometry",
        "purpose": "Mass of air per mass of fuel for complete combustion, with the moles of O2 and air per mole of fuel.",
        "inputs": [{"symbol": "fuel", "type": "choice", "choices": list(FUELS), "meaning": "the fuel", "required": True}],
        "outputs": [{"symbol": "afr_stoich", "kind": "air_fuel_ratio", "meaning": "stoichiometric air-fuel ratio (mass)"},
                    {"symbol": "o2_moles", "kind": "number", "meaning": "moles of O2 per mole of fuel"},
                    {"symbol": "air_moles", "kind": "number", "meaning": "moles of air per mole of fuel"}],
        "primary": "afr_stoich", "conditions": ["complete combustion to CO2 and H2O", "air = O2 + 3.76 N2"],
        "method": "atom balance of CxHy + (x + y/4)(O2 + 3.76 N2), then the mass ratio with standard atomic weights",
        "latex": r"AFR_s = \frac{(x + y/4)(M_{O_2} + 3.76 M_{N_2})}{x M_C + y M_H}",
        "run": stoich, "verify": verify_stoich, "parse": parse_stoich, "reference": ref_stoich,
        "wrong": lambda fuel: (lambda s: {**s, "afr_stoich": s["afr_stoich"] / 4.76 * 4.29})(stoich(fuel)),
        "random_inputs": lambda rng: {"fuel": rng.choice(list(FUELS))},
        "templates": ["What is the stoichiometric air-fuel ratio of methane?"],
        "examples": [{"inputs": {"fuel": "methane"}, "outputs": {"o2_moles": 2.0, "air_moles": 9.52}},
                     {"inputs": {"fuel": "propane"}, "outputs": {"o2_moles": 5.0}}],
        "questions": [{"q": "What is the stoichiometric air-fuel ratio of methane?", "outputs": {"o2_moles": 2.0}},
                      {"q": "What is the stoichiometric air-fuel ratio of propane C3H8?", "outputs": {"o2_moles": 5.0}},
                      {"q": "What is the stoichiometric air-fuel ratio of methane at 300 K?", "outcome": "UNRECOGNIZED_FORMAT"}]},
    "equivalence_ratio": {
        "name": "Equivalence ratio of a fuel-air mixture", "topic": "combustion / mixture strength",
        "purpose": "Equivalence ratio and excess air of a gaseous fuel burned at a given air-fuel ratio by mass.",
        "inputs": [{"symbol": "fuel", "type": "choice", "choices": list(FUELS), "meaning": "the fuel", "required": True},
                   _num_in("afr", "actual air-fuel ratio by mass")],
        "outputs": [{"symbol": "phi", "kind": "number", "meaning": "equivalence ratio (fuel-air ratio over stoichiometric)"},
                    {"symbol": "afr_stoich", "kind": "air_fuel_ratio", "meaning": "stoichiometric air-fuel ratio"},
                    {"symbol": "excess_air", "kind": "number", "meaning": "excess air as a fraction"}],
        "primary": "phi", "conditions": ["afr > 0"], "method": "phi = AFR_s / AFR with the stoichiometric ratio from the atom balance",
        "latex": r"\phi = \frac{AFR_s}{AFR}",
        "run": equivalence, "verify": verify_equiv, "parse": parse_equiv, "reference": ref_equiv,
        "wrong": lambda fuel, afr: (lambda e: {**e, "phi": afr / e["afr_stoich"]})(equivalence(fuel, afr)),
        "random_inputs": lambda rng: (lambda f: {"fuel": f, "afr": stoich(f)["afr_stoich"] * rng.choice([rng.uniform(0.4, 0.8), rng.uniform(1.3, 2.5)])})(
            rng.choice(list(FUELS))),
        "templates": ["Equivalence ratio of methane: afr = 20"],
        "examples": [{"inputs": {"fuel": "methane", "afr": 20}, "outputs": {"phi": stoich("methane")["afr_stoich"] / 20}}],
        "questions": [{"q": "Equivalence ratio of methane: afr = 20", "outputs": {"phi": stoich("methane")["afr_stoich"] / 20}},
                      {"q": "Equivalence ratio of propane: afr = 0", "outcome": "INVALID_INPUT"},
                      {"q": "Equivalence ratio of methane", "outcome": "NEEDS_INPUT"}]},
    "gnielinski_nusselt": {
        "name": "Gnielinski correlation for turbulent pipe flow", "topic": "convection / internal flow",
        "purpose": "Nusselt number of fully developed turbulent or transitional flow in a tube.",
        "inputs": [_num_in("Re", "Reynolds number (3000 to 5e6)"), _num_in("Pr", "Prandtl number (0.5 to 2000)"),
                   _num_in("f", "Darcy friction factor")],
        "outputs": [{"symbol": "Nu", "kind": "nusselt_number", "meaning": "Nusselt number based on the diameter"}],
        "primary": "Nu", "conditions": ["3000 <= Re <= 5e6", "0.5 <= Pr <= 2000"],
        "method": "Nu = (f/8)(Re - 1000)Pr / (1 + 12.7 sqrt(f/8)(Pr^(2/3) - 1))",
        "latex": r"Nu = \frac{(f/8)(Re - 1000)Pr}{1 + 12.7\sqrt{f/8}\,(Pr^{2/3} - 1)}",
        "run": gnielinski, "verify": verify_gn, "reference": ref_gn,
        "parse": lambda q: _pairs(q, r"gnielinski(?: correlation| nusselt number)?", {"re": "Re", "pr": "Pr", "f": "f"}),
        "wrong": lambda Re, Pr, f: {"Nu": (f / 8) * Re * Pr / (1 + 12.7 * math.sqrt(f / 8) * (Pr ** (2 / 3) - 1))},
        "random_inputs": lambda rng: {"Re": _log_uniform(rng, 3000, 2e4), "Pr": _log_uniform(rng, 0.5, 200), "f": rng.uniform(0.02, 0.05)},
        "templates": ["Gnielinski correlation: Re = 50000, Pr = 5, f = 0.021"],
        "examples": [{"inputs": {"Re": 50000, "Pr": 5, "f": 0.021}, "outputs": {"Nu": (0.021 / 8) * 49000 * 5 / (1 + 12.7 * math.sqrt(0.021 / 8) * (5 ** (2 / 3) - 1))}}],
        "questions": [{"q": "Gnielinski correlation: Re = 50000, Pr = 5, f = 0.021", "outputs": {}},
                      {"q": "Gnielinski correlation: Re = 1000, Pr = 5, f = 0.06", "outcome": "INVALID_INPUT"}]},
    "churchill_bernstein_cylinder": {
        "name": "Churchill-Bernstein correlation for a cylinder in cross flow", "topic": "convection / external flow",
        "purpose": "Average Nusselt number of a circular cylinder in cross flow over the whole Reynolds range.",
        "inputs": [_num_in("Re", "Reynolds number based on the diameter"), _num_in("Pr", "Prandtl number")],
        "outputs": [{"symbol": "Nu", "kind": "nusselt_number", "meaning": "average Nusselt number based on the diameter"}],
        "primary": "Nu", "conditions": ["Re * Pr >= 0.2"],
        "method": "Nu = 0.3 + 0.62 Re^(1/2) Pr^(1/3) / (1 + (0.4/Pr)^(2/3))^(1/4) (1 + (Re/282000)^(5/8))^(4/5)",
        "latex": r"Nu = 0.3 + \frac{0.62\,Re^{1/2}Pr^{1/3}}{[1 + (0.4/Pr)^{2/3}]^{1/4}}\left[1 + \left(\frac{Re}{282000}\right)^{5/8}\right]^{4/5}",
        "run": churchill_bernstein, "verify": verify_cb,
        "reference": lambda Re, Pr: {"Nu": __import__("ht").conv_external.Nu_cylinder_Churchill_Bernstein(Re, Pr)},
        "parse": lambda q: _pairs(q, r"churchill[- ]bernstein(?: correlation)?", {"re": "Re", "pr": "Pr"}),
        "wrong": lambda Re, Pr: {"Nu": 0.62 * Re ** 0.5 * Pr ** (1 / 3) / (1 + (0.4 / Pr) ** (2 / 3)) ** 0.25 * (1 + (Re / 282000) ** 0.625) ** 0.8},
        "random_inputs": lambda rng: {"Re": _log_uniform(rng, 1, 1e6), "Pr": _log_uniform(rng, 0.6, 100)},
        "templates": ["Churchill-Bernstein correlation: Re = 10000, Pr = 0.7"],
        "examples": [{"inputs": {"Re": 10000, "Pr": 0.7}, "outputs": churchill_bernstein(10000, 0.7)}],
        "questions": [{"q": "Churchill-Bernstein correlation: Re = 10000, Pr = 0.7", "outputs": churchill_bernstein(10000, 0.7)},
                      {"q": "Churchill-Bernstein correlation: Re = 0.1, Pr = 0.7", "outcome": "INVALID_INPUT"},
                      {"q": "Churchill-Bernstein correlation: Re = 10000", "outcome": "NEEDS_INPUT"}]},
    "churchill_chu_vertical_plate": {
        "name": "Churchill-Chu correlation for natural convection on a vertical plate", "topic": "convection / natural convection",
        "purpose": "Average Nusselt number of free convection from an isothermal vertical plate, laminar and turbulent.",
        "inputs": [_num_in("Ra", "Rayleigh number based on the plate height"), _num_in("Pr", "Prandtl number")],
        "outputs": [{"symbol": "Nu", "kind": "nusselt_number", "meaning": "average Nusselt number"}],
        "primary": "Nu", "conditions": ["0 < Ra <= 1e13"],
        "method": "Nu = (0.825 + 0.387 Ra^(1/6) / (1 + (0.492/Pr)^(9/16))^(8/27))^2",
        "latex": r"Nu = \left(0.825 + \frac{0.387\,Ra^{1/6}}{[1 + (0.492/Pr)^{9/16}]^{8/27}}\right)^2",
        "run": churchill_chu, "verify": verify_cc, "reference": ref_cc,
        "parse": lambda q: _pairs(q, r"churchill[- ]chu(?: correlation)?", {"ra": "Ra", "pr": "Pr"}),
        "wrong": lambda Ra, Pr: {"Nu": (0.825 + 0.387 * Ra ** (1 / 6) / (1 + (0.492 / Pr) ** (9 / 16)) ** (8 / 27))},
        "random_inputs": lambda rng: {"Ra": _log_uniform(rng, 1e3, 1e12), "Pr": _log_uniform(rng, 0.6, 100)},
        "templates": ["Churchill-Chu correlation: Ra = 1000000000, Pr = 0.71"],
        "examples": [{"inputs": {"Ra": 1e9, "Pr": 0.71}, "outputs": {"Nu": (0.825 + 0.387 * 1e9 ** (1 / 6) / (1 + (0.492 / 0.71) ** (9 / 16)) ** (8 / 27)) ** 2}}],
        "questions": [{"q": "Churchill-Chu correlation: Ra = 1000000000, Pr = 0.71", "outputs": {}},
                      {"q": "Churchill-Chu correlation: Ra = 1000000000", "outcome": "NEEDS_INPUT"}]},
    "pin_fin_efficiency": {
        "name": "Efficiency of a pin fin with an insulated tip", "topic": "fins / extended surfaces",
        "purpose": "Fin efficiency tanh(mL)/(mL) of a cylindrical pin fin with m = sqrt(4h/(kD)).",
        "inputs": [_phys("h", "heat_transfer_coefficient", "heat transfer coefficient"), _phys("k", "thermal_conductivity", "fin thermal conductivity"),
                   _phys("D", "length", "pin diameter"), _phys("L", "length", "fin length")],
        "outputs": [{"symbol": "efficiency", "kind": "number", "meaning": "fin efficiency"}, {"symbol": "mL", "kind": "number", "meaning": "fin parameter m L"}],
        "primary": "efficiency", "conditions": ["h, k, D, L > 0", "one-dimensional conduction, uniform h, insulated tip"],
        "method": "eta = tanh(mL)/(mL), m = sqrt(4h/(kD))", "latex": r"\eta_f = \frac{\tanh(mL)}{mL},\quad m = \sqrt{\frac{4h}{kD}}",
        "run": pin_fin, "verify": verify_fin, "reference": ref_fin,
        "parse": lambda q: _pairs(q, r"pin[- ]fin efficiency", {"h": "h", "k": "k", "d": "D", "l": "L"}, units={"h", "k", "D", "L"}),
        "wrong": lambda h, k, D, L: (lambda m: {"efficiency": math.tanh(m * L) / (m * L), "mL": m * L})(math.sqrt(2 * h / (k * D))),
        "random_inputs": lambda rng: {"h": rng.uniform(10, 300), "k": rng.uniform(15, 400), "D": rng.uniform(0.002, 0.02), "L": rng.uniform(0.01, 0.1)},
        "templates": ["Pin fin efficiency: h = 50 W/(m^2*K), k = 200 W/(m*K), D = 5 mm, L = 50 mm"],
        "examples": [{"inputs": {"h": 50, "k": 200, "D": 0.005, "L": 0.05}, "outputs": {"mL": math.sqrt(4 * 50 / (200 * 0.005)) * 0.05}}],
        "questions": [{"q": "Pin fin efficiency: h = 50 W/(m^2*K), k = 200 W/(m*K), D = 5 mm, L = 50 mm",
                       "outputs": {"mL": math.sqrt(4 * 50 / (200 * 0.005)) * 0.05}},
                      {"q": "Pin fin efficiency: h = 50 W/(m^2*K), k = 200 W/(m*K), D = 5 mm", "outcome": "NEEDS_INPUT"}]},
}


# ================================================================================================ core additions (2026-11-12)
# The elementary first-course calculations the curriculum gap report found missing: the first law, boundary work at
# constant pressure, the entropy of heat exchanged at a fixed temperature (one reservoir, and heat flowing between two),
# Pascal's hydraulic press and Bernoulli's equation with a change of height.
# Conventions: W is the work done BY the system (W > 0 when the system expands); Q is the heat added TO the system
# (Q < 0 when heat leaves it). Pressures in Bernoulli's equation are absolute. g is an input (it is never assumed).
# Their documented form is the same 'phrase: name = value, ...' with EVERY value carrying a unit; units the pair reader
# above lacks (J, N, m^2, m^3, L, m/s, kg/m^3, m/s^2) are read by the reader below and converted by the runtime.
_U2 = (r"kg/m\^3|m/s\^2|m/s|J/K|kJ/K|cm\^2|mm\^2|m\^2|cm\^3|m\^3|kJ|MJ|mJ|J|kN|MN|N|kPa|MPa|bar|Pa|atm|°\s*C|degC|K|L|"
       r"km|cm|mm|m")
_PAIR2 = re.compile(r"\b([A-Za-z][A-Za-z_0-9]*)\s*=\s*(" + _NUM + r")\s*(?:(" + _U2 + r")(?![A-Za-z0-9^/*]))?")


def _upairs(question: str, phrase: str, names: dict):
    """{symbol: 'value unit'} from 'phrase ... name = value unit, ...'; None unless the phrase is present, every number
    belongs to a known name and every value carries a unit (temperatures in K or °C)."""
    q = question.strip()
    if not re.search(phrase, q, re.I):
        return None
    out, rest = {}, q
    for m in _PAIR2.finditer(q):
        key = names.get(m.group(1).lower())
        unit = (m.group(3) or "").replace(" ", "")
        if key is None or key in out or not unit:
            return None
        out[key] = f"{m.group(2)} {'°C' if unit.lower() in ('°c', 'degc') else unit}"
        rest = rest.replace(m.group(0), " ", 1)
    if re.search(r"\d", re.sub(phrase, " ", rest, flags=re.I)):
        return None                                   # a number that is not a 'name = value unit' pair: never guessed
    return out


def _quad(f, a, b):
    from scipy.integrate import quad
    return quad(f, a, b, epsabs=0.0, epsrel=1e-13, limit=200)[0]


# ------------------------------------------------------------------------------------------------ first law
def first_law(Q, W) -> dict:
    if not (math.isfinite(Q) and math.isfinite(W)):
        raise ValueError("the heat and the work must be finite numbers")
    return {"dU": Q - W}


def verify_first_law(i, o) -> float:
    s = max(abs(i["Q"]), abs(i["W"]))
    if s == 0:
        return abs(o["dU"])
    return abs(o["dU"] + i["W"] - i["Q"]) / s


def ref_first_law(Q, W):
    """The first law in rate form, dU/dt = Qdot - Wdot, integrated over a process that adds Q and does W uniformly
    (scipy quad). A restatement of the law: a weaker reference, documented."""
    return {"dU": _quad(lambda t: Q - W, 0.0, 1.0)}


# ------------------------------------------------------------------------------------------------ isobaric work
def isobaric_work(P, V1, V2) -> dict:
    if not (math.isfinite(P) and P > 0):
        raise ValueError("the pressure must be positive (an absolute pressure)")
    if not (math.isfinite(V1) and math.isfinite(V2) and V1 > 0 and V2 > 0):
        raise ValueError("the initial and final volumes must be positive")
    return {"W": P * (V2 - V1), "dV": V2 - V1}


def verify_isobaric_work(i, o) -> float:
    P, V1, V2 = i["P"], i["V1"], i["V2"]
    s = max(V1, V2)
    return abs(o["W"] - P * (V2 - V1)) / (P * s) + abs(o["dV"] - (V2 - V1)) / s


def ref_isobaric_work(P, V1, V2):
    """Boundary work as the integral of P dV from V1 to V2 (scipy quad), the definition rather than the closed form."""
    return {"W": _quad(lambda V: P, V1, V2), "dV": V2 - V1}


# ------------------------------------------------------------------------------------------------ entropy and heat
def entropy_reservoir(Q, T) -> dict:
    if not (math.isfinite(T) and T > 0):
        raise ValueError("the absolute temperature must be positive")
    if not math.isfinite(Q):
        raise ValueError("the heat must be a finite number")
    return {"dS": Q / T}


def verify_entropy_reservoir(i, o) -> float:
    Q, T = i["Q"], i["T"]
    if Q == 0:
        return abs(o["dS"])
    return abs(o["dS"] * T - Q) / abs(Q)


def ref_entropy_reservoir(Q, T):
    """dS = integral of dQ_rev / T over the heat added at the fixed temperature (scipy quad). Weaker reference, documented."""
    return {"dS": _quad(lambda q: 1.0 / T, 0.0, Q)}


def entropy_heat_transfer(Q, T_hot, T_cold) -> dict:
    if not (math.isfinite(T_hot) and math.isfinite(T_cold) and T_hot > 0 and T_cold > 0):
        raise ValueError("the absolute temperatures must be positive")
    if not (math.isfinite(Q) and Q > 0):
        raise ValueError("Q is the heat that flows from the hot to the cold reservoir and must be positive")
    if T_hot < T_cold:
        raise ValueError("heat cannot flow by itself from the colder to the hotter reservoir (the total entropy would fall)")
    return {"dS_total": Q * (T_hot - T_cold) / (T_hot * T_cold), "dS_hot": -Q / T_hot, "dS_cold": Q / T_cold}


def verify_entropy_heat_transfer(i, o) -> float:
    Q, Th, Tc = i["Q"], i["T_hot"], i["T_cold"]
    return (abs(o["dS_hot"] * Th + Q) / Q + abs(o["dS_cold"] * Tc - Q) / Q
            + abs(o["dS_total"] * Th * Tc - Q * (Th - Tc)) / (Q * Th))


def ref_entropy_heat_transfer(Q, T_hot, T_cold):
    """Each reservoir's entropy change as the integral of dQ/T (scipy quad), summed. Weaker reference, documented."""
    s_hot = -_quad(lambda q: 1.0 / T_hot, 0.0, Q)
    s_cold = _quad(lambda q: 1.0 / T_cold, 0.0, Q)
    return {"dS_total": s_hot + s_cold, "dS_hot": s_hot, "dS_cold": s_cold}


# ------------------------------------------------------------------------------------------------ hydraulic press
def hydraulic_press(F1, A1, A2) -> dict:
    if not (math.isfinite(F1) and F1 > 0):
        raise ValueError("the input force must be positive")
    if not (math.isfinite(A1) and math.isfinite(A2) and A1 > 0 and A2 > 0):
        raise ValueError("the piston areas must be positive")
    P = F1 / A1
    return {"F2": P * A2, "P": P, "MA": A2 / A1, "d_ratio": A1 / A2}


def verify_hydraulic_press(i, o) -> float:
    F1, A1, A2 = i["F1"], i["A1"], i["A2"]
    return (abs(o["F2"] * A1 - F1 * A2) / (F1 * A2) + abs(o["P"] * A1 - F1) / F1
            + abs(o["MA"] * A1 - A2) / A2 + abs(o["d_ratio"] * A2 - A1) / A1)


def ref_hydraulic_press(F1, A1, A2):
    """Energy route: an input stroke d1 displaces A1 d1 of liquid, which lifts the output piston d2 = A1 d1 / A2; a
    lossless press does the same work on the load as on the input piston (work by scipy quad), so F2 = W / d2."""
    d1 = 0.01
    d2 = A1 * d1 / A2
    W_in = _quad(lambda x: F1, 0.0, d1)
    F2 = W_in / d2
    return {"F2": F2, "P": F2 / A2, "MA": F2 / F1, "d_ratio": d2 / d1}


# ------------------------------------------------------------------------------------------------ Bernoulli with height
def bernoulli_height(P1, v1, h1, v2, h2, rho, g) -> dict:
    if not (math.isfinite(rho) and rho > 0 and math.isfinite(g) and g > 0):
        raise ValueError("the density and g must be positive")
    if not (math.isfinite(P1) and P1 > 0):
        raise ValueError("P1 is an absolute pressure and must be positive")
    if not (math.isfinite(v1) and math.isfinite(v2) and v1 >= 0 and v2 >= 0):
        raise ValueError("give the speeds as magnitudes (v >= 0)")
    if not (math.isfinite(h1) and math.isfinite(h2)):
        raise ValueError("the heights must be finite numbers")
    P2 = P1 + 0.5 * rho * (v1 * v1 - v2 * v2) + rho * g * (h1 - h2)
    if not P2 > 0:
        raise ValueError("the computed absolute pressure P2 is not positive: the flow cannot exist so (the liquid would cavitate)")
    return {"P2": P2}


def verify_bernoulli_height(i, o) -> float:
    rho, g = i["rho"], i["g"]
    lhs = o["P2"] + 0.5 * rho * i["v2"] * i["v2"] + rho * g * i["h2"]
    rhs = i["P1"] + 0.5 * rho * i["v1"] * i["v1"] + rho * g * i["h1"]
    s = i["P1"] + 0.5 * rho * (i["v1"] * i["v1"] + i["v2"] * i["v2"]) + rho * g * (abs(i["h1"]) + abs(i["h2"]))
    return abs(lhs - rhs) / s


def ref_bernoulli_height(P1, v1, h1, v2, h2, rho, g):
    """Euler's equation along the streamline, dp/ds = -rho (v dv/ds + g dz/ds), integrated (scipy quad) on a path from
    point 1 to point 2 on which the speed and the height vary linearly (the result depends only on the end points)."""
    dv, dh = v2 - v1, h2 - h1
    dp = _quad(lambda s: -rho * ((v1 + dv * s) * dv + g * dh), 0.0, 1.0)
    return {"P2": P1 + dp}


_TF_FIRST = r"first law(?: of thermodynamics)?"
_TF_ISOB = r"isobaric (?:boundary )?work|constant[- ]pressure work|work at constant pressure"
_TF_SRES = r"entropy change (?:of a (?:thermal )?reservoir|at (?:a )?constant temperature)"
_TF_SGEN = r"entropy (?:generated|generation|produced) (?:by|in) heat transfer"
_TF_PRESS = r"hydraulic (?:press|lift|jack)|pascal'?s principle"
_TF_BERN = r"bernoulli(?:'s)?(?: equation)? with (?:a )?(?:change (?:of|in) )?(?:height|elevation)"


def _sgn(rng):
    return rng.choice([-1.0, 1.0])


PROCEDURES.update({
    "first_law_internal_energy_change": {
        "name": "First law of thermodynamics: change of internal energy", "topic": "thermodynamics / first law",
        "purpose": "The change of internal energy of a closed system from the heat added to it and the work it does.",
        "inputs": [_phys("Q", "energy", "heat added to the system (negative if heat leaves it)"),
                   _phys("W", "energy", "work done BY the system (negative if work is done on it)")],
        "outputs": [{"symbol": "dU", "kind": "energy", "meaning": "change of internal energy"}],
        "primary": "dU", "conditions": ["closed system", "Q added to the system, W done by the system"],
        "method": "dU = Q - W",
        "latex": r"\Delta U = Q - W",
        "run": first_law, "verify": verify_first_law, "reference": ref_first_law,
        "parse": lambda q: _upairs(q, _TF_FIRST, {"q": "Q", "w": "W"}),
        "wrong": lambda Q, W: {"dU": Q + W},                                     # the work-on-the-system sign
        "random_inputs": lambda rng: (lambda q: {"Q": q, "W": _sgn(rng) * abs(q) * rng.uniform(0.1, 2.0)})(_sgn(rng) * _log_uniform(rng, 10, 1e6)),
        "templates": ["First law of thermodynamics: Q = 500 J, W = 200 J"],
        "examples": [{"inputs": {"Q": 500.0, "W": 200.0}, "outputs": {"dU": 300.0}},
                     {"inputs": {"Q": -300.0, "W": -100.0}, "outputs": {"dU": -200.0}},
                     {"inputs": {"Q": 1200.0, "W": 1500.0}, "outputs": {"dU": -300.0}}],
        "questions": [{"q": "First law of thermodynamics: Q = 500 J, W = 200 J", "outputs": {"dU": 300.0}},
                      {"q": "First law of thermodynamics: Q = -300 J, W = -100 J", "outputs": {"dU": -200.0}},
                      {"q": "First law: Q = 1.2 kJ, W = 1500 J", "outputs": {"dU": -300.0}},
                      {"q": "First law of thermodynamics: Q = 1e400 J, W = 200 J", "outcome": "INVALID_INPUT"},
                      {"q": "First law of thermodynamics: Q = 500 J", "outcome": "NEEDS_INPUT"}]},
    "isobaric_boundary_work": {
        "name": "Boundary work of a constant-pressure (isobaric) process", "topic": "thermodynamics / boundary work",
        "purpose": "The work done by a gas that expands or is compressed at constant pressure from V1 to V2.",
        "inputs": [_phys("P", "pressure", "constant (absolute) pressure"), _phys("V1", "volume", "initial volume"),
                   _phys("V2", "volume", "final volume")],
        "outputs": [{"symbol": "W", "kind": "energy", "meaning": "work done by the system (negative for a compression)"},
                    {"symbol": "dV", "kind": "volume", "meaning": "change of volume V2 - V1"}],
        "primary": "W", "conditions": ["P > 0 and constant", "V1, V2 > 0", "quasi-static process"],
        "method": "W = P (V2 - V1)",
        "latex": r"W = \int_{V_1}^{V_2} P\,dV = P\,(V_2 - V_1)",
        "run": isobaric_work, "verify": verify_isobaric_work, "reference": ref_isobaric_work,
        "parse": lambda q: _upairs(q, _TF_ISOB, {"p": "P", "v1": "V1", "v2": "V2"}),
        "wrong": lambda P, V1, V2: {"W": P * (V1 - V2), "dV": V2 - V1},          # the sign of the work on the gas
        "random_inputs": lambda rng: (lambda v1: {"P": _log_uniform(rng, 5e4, 5e6), "V1": v1,
                                                  "V2": v1 * rng.choice([rng.uniform(0.3, 0.8), rng.uniform(1.25, 3.0)])})(rng.uniform(1e-3, 1.0)),
        "templates": ["Isobaric work: P = 200 kPa, V1 = 0.010 m^3, V2 = 0.025 m^3"],
        "examples": [{"inputs": {"P": 200000.0, "V1": 0.010, "V2": 0.025}, "outputs": {"W": 3000.0}},
                     {"inputs": {"P": 101325.0, "V1": 0.002, "V2": 0.001}, "outputs": {"W": -101.325}}],
        "questions": [{"q": "Isobaric work: P = 200 kPa, V1 = 0.010 m^3, V2 = 0.025 m^3", "outputs": {"W": 3000.0}},
                      {"q": "Isobaric work: P = 2 bar, V1 = 10 L, V2 = 25 L", "outputs": {"W": 3000.0}},
                      {"q": "Isobaric work: P = 200 kPa, V1 = -0.010 m^3, V2 = 0.025 m^3", "outcome": "INVALID_INPUT"},
                      {"q": "Isobaric work: P = 200 kPa, V1 = 0.010 m^3", "outcome": "NEEDS_INPUT"}]},
    "entropy_change_reservoir": {
        "name": "Entropy change for heat exchanged at a constant temperature", "topic": "thermodynamics / entropy",
        "purpose": "The entropy change dS = Q/T of a reservoir (or any system) that takes in heat Q reversibly at a fixed absolute temperature T.",
        "inputs": [_phys("Q", "energy", "heat added to the reservoir (negative if heat leaves it)"),
                   _phys("T", "temperature", "absolute temperature of the reservoir")],
        "outputs": [{"symbol": "dS", "kind": "entropy", "meaning": "entropy change of the reservoir"}],
        "primary": "dS", "conditions": ["T > 0 (absolute)", "the temperature stays constant while Q is exchanged"],
        "method": "dS = Q / T",
        "latex": r"\Delta S = \frac{Q}{T}",
        "run": entropy_reservoir, "verify": verify_entropy_reservoir, "reference": ref_entropy_reservoir,
        "parse": lambda q: _upairs(q, _TF_SRES, {"q": "Q", "t": "T"}),
        "wrong": lambda Q, T: {"dS": Q / (T - 273.15)},                          # the temperature in °C
        "random_inputs": lambda rng: {"Q": _sgn(rng) * _log_uniform(rng, 10, 1e7), "T": rng.uniform(320, 1500)},
        "templates": ["Entropy change of a reservoir: Q = 1000 J, T = 300 K"],
        "examples": [{"inputs": {"Q": 1000.0, "T": 300.0}, "outputs": {"dS": 1000 / 300}},
                     {"inputs": {"Q": -5000.0, "T": 500.0}, "outputs": {"dS": -10.0}}],
        "questions": [{"q": "Entropy change of a reservoir: Q = 1000 J, T = 300 K", "outputs": {"dS": 1000 / 300}},
                      {"q": "Entropy change of a reservoir: Q = 2 kJ, T = 27 °C", "outputs": {"dS": 2000 / 300.15}},
                      {"q": "Entropy change of a reservoir: Q = 1000 J, T = 0 K", "outcome": "INVALID_INPUT"},
                      {"q": "Entropy change of a reservoir: Q = 1000 J", "outcome": "NEEDS_INPUT"}]},
    "entropy_generation_heat_transfer": {
        "name": "Entropy generated when heat flows between two reservoirs", "topic": "thermodynamics / entropy and the second law",
        "purpose": "The entropy changes of a hot and a cold reservoir when heat Q flows from the hot one to the cold one, and the total (generated) entropy.",
        "inputs": [_phys("Q", "energy", "heat that flows from the hot to the cold reservoir"),
                   _phys("T_hot", "temperature", "absolute temperature of the hot reservoir"),
                   _phys("T_cold", "temperature", "absolute temperature of the cold reservoir")],
        "outputs": [{"symbol": "dS_total", "kind": "entropy", "meaning": "total entropy change (entropy generated)"},
                    {"symbol": "dS_hot", "kind": "entropy", "meaning": "entropy change of the hot reservoir"},
                    {"symbol": "dS_cold", "kind": "entropy", "meaning": "entropy change of the cold reservoir"}],
        "primary": "dS_total", "conditions": ["Q > 0", "T_hot >= T_cold > 0 (absolute)", "both temperatures stay constant"],
        "method": "dS_hot = -Q/T_hot, dS_cold = Q/T_cold, dS_total = Q (T_hot - T_cold)/(T_hot T_cold) >= 0",
        "latex": r"\Delta S_{tot} = -\frac{Q}{T_h} + \frac{Q}{T_c} = Q\,\frac{T_h - T_c}{T_h T_c}",
        "run": entropy_heat_transfer, "verify": verify_entropy_heat_transfer, "reference": ref_entropy_heat_transfer,
        "parse": lambda q: _upairs(q, _TF_SGEN, {"q": "Q", "t_hot": "T_hot", "th": "T_hot", "t_cold": "T_cold", "tc": "T_cold"}),
        "wrong": lambda Q, T_hot, T_cold: {"dS_total": Q / T_hot - Q / T_cold, "dS_hot": Q / T_hot, "dS_cold": -Q / T_cold},
        "random_inputs": lambda rng: (lambda tc: {"Q": _log_uniform(rng, 10, 1e7), "T_cold": tc, "T_hot": tc * rng.uniform(1.2, 4.0)})(rng.uniform(200, 600)),
        "templates": ["Entropy generated by heat transfer: Q = 1000 J, T_hot = 500 K, T_cold = 300 K"],
        "examples": [{"inputs": {"Q": 1000.0, "T_hot": 500.0, "T_cold": 300.0},
                      "outputs": {"dS_total": 1000 * 200 / (500 * 300), "dS_hot": -2.0, "dS_cold": 1000 / 300}},
                     {"inputs": {"Q": 1200.0, "T_hot": 600.0, "T_cold": 400.0}, "outputs": {"dS_total": 1.0, "dS_hot": -2.0, "dS_cold": 3.0}}],
        "questions": [{"q": "Entropy generated by heat transfer: Q = 1000 J, T_hot = 500 K, T_cold = 300 K",
                       "outputs": {"dS_total": 1000 * 200 / (500 * 300)}},
                      {"q": "Entropy generated by heat transfer: Q = 1.2 kJ, T_hot = 327 °C, T_cold = 127 °C",
                       "outputs": {"dS_total": 1200 * 200 / (600.15 * 400.15)}},
                      {"q": "Entropy generated by heat transfer: Q = 1000 J, T_hot = 300 K, T_cold = 500 K", "outcome": "INVALID_INPUT"},
                      {"q": "Entropy generated by heat transfer: Q = 1000 J, T_hot = 500 K", "outcome": "NEEDS_INPUT"}]},
    "hydraulic_press_pascal": {
        "name": "Hydraulic press (Pascal's principle)", "topic": "fluid statics / Pascal's principle",
        "purpose": "Output force, pressure, mechanical advantage and piston-travel ratio of an ideal hydraulic press or lift.",
        "inputs": [_phys("F1", "force", "force on the input (small) piston"), _phys("A1", "area", "area of the input piston"),
                   _phys("A2", "area", "area of the output piston")],
        "outputs": [{"symbol": "F2", "kind": "force", "meaning": "force on the output piston"},
                    {"symbol": "P", "kind": "pressure", "meaning": "pressure transmitted through the liquid (above ambient)"},
                    {"symbol": "MA", "kind": "number", "meaning": "mechanical advantage F2/F1 = A2/A1"},
                    {"symbol": "d_ratio", "kind": "number", "meaning": "output piston travel / input piston travel (d2/d1 = A1/A2)"}],
        "primary": "F2", "conditions": ["F1, A1, A2 > 0", "incompressible liquid, pistons at the same level, no friction"],
        "method": "P = F1/A1 is transmitted undiminished, F2 = P A2; the displaced volume is shared, d2/d1 = A1/A2",
        "latex": r"\frac{F_1}{A_1} = \frac{F_2}{A_2},\quad \frac{d_2}{d_1} = \frac{A_1}{A_2}",
        "run": hydraulic_press, "verify": verify_hydraulic_press, "reference": ref_hydraulic_press,
        "parse": lambda q: _upairs(q, _TF_PRESS, {"f1": "F1", "a1": "A1", "a2": "A2"}),
        "wrong": lambda F1, A1, A2: {"F2": F1 * A1 / A2, "P": F1 / A1, "MA": A1 / A2, "d_ratio": A2 / A1},   # the ratio inverted
        "random_inputs": lambda rng: (lambda a1: {"F1": _log_uniform(rng, 1, 1e4), "A1": a1, "A2": a1 * _log_uniform(rng, 4, 400)})(_log_uniform(rng, 1e-5, 1e-2)),
        "templates": ["Hydraulic press: F1 = 100 N, A1 = 0.01 m^2, A2 = 0.5 m^2"],
        "examples": [{"inputs": {"F1": 100.0, "A1": 0.01, "A2": 0.5}, "outputs": {"F2": 5000.0, "P": 10000.0, "MA": 50.0, "d_ratio": 0.02}},
                     {"inputs": {"F1": 50.0, "A1": 0.001, "A2": 0.02}, "outputs": {"F2": 1000.0, "MA": 20.0}}],
        "questions": [{"q": "Hydraulic press: F1 = 100 N, A1 = 0.01 m^2, A2 = 0.5 m^2", "outputs": {"F2": 5000.0}},
                      {"q": "Hydraulic press: F1 = 50 N, A1 = 10 cm^2, A2 = 200 cm^2", "outputs": {"F2": 1000.0}},
                      {"q": "Hydraulic press: F1 = 100 N, A1 = 0 m^2, A2 = 0.5 m^2", "outcome": "INVALID_INPUT"},
                      {"q": "Hydraulic press: F1 = 100 N, A1 = 0.01 m^2", "outcome": "NEEDS_INPUT"}]},
    "bernoulli_with_height": {
        "name": "Bernoulli's equation with a change of height", "topic": "fluid flow / Bernoulli's equation",
        "purpose": "The absolute pressure at a second point of a streamline of steady, inviscid, incompressible flow from the pressure, speed and height at the first and the speed and height at the second.",
        "inputs": [_phys("P1", "pressure", "absolute pressure at point 1"), _phys("v1", "speed", "flow speed at point 1"),
                   _phys("h1", "length", "height of point 1 above a datum"), _phys("v2", "speed", "flow speed at point 2"),
                   _phys("h2", "length", "height of point 2 above the same datum"), _phys("rho", "density", "density of the fluid"),
                   _phys("g", "acceleration", "gravitational acceleration")],
        "outputs": [{"symbol": "P2", "kind": "pressure", "meaning": "absolute pressure at point 2"}],
        "primary": "P2", "conditions": ["steady, inviscid, incompressible flow along one streamline", "rho, g > 0", "P1 and P2 absolute and positive"],
        "method": "P2 = P1 + rho (v1^2 - v2^2)/2 + rho g (h1 - h2)",
        "latex": r"P_1 + \tfrac{1}{2}\rho v_1^2 + \rho g h_1 = P_2 + \tfrac{1}{2}\rho v_2^2 + \rho g h_2",
        "run": bernoulli_height, "verify": verify_bernoulli_height, "reference": ref_bernoulli_height,
        "parse": lambda q: _upairs(q, _TF_BERN, {"p1": "P1", "v1": "v1", "h1": "h1", "v2": "v2", "h2": "h2", "rho": "rho", "g": "g"}),
        "wrong": lambda P1, v1, h1, v2, h2, rho, g: {"P2": P1 + 0.5 * rho * (v1 * v1 - v2 * v2) + rho * g * (h2 - h1)},  # height sign
        "random_inputs": lambda rng: (lambda h1: {"P1": rng.uniform(1.5e5, 4e5), "v1": rng.uniform(0.5, 5.0), "h1": h1, "v2": rng.uniform(0.5, 8.0),
                                                  "h2": h1 + _sgn(rng) * rng.uniform(1.0, 8.0), "rho": rng.uniform(700, 1100),
                                                  "g": rng.choice([9.80665, 9.81, 9.8])})(rng.uniform(0.0, 5.0)),
        "templates": ["Bernoulli equation with height: P1 = 200 kPa, v1 = 2 m/s, h1 = 0 m, v2 = 6 m/s, h2 = 5 m, rho = 1000 kg/m^3, g = 9.81 m/s^2"],
        "examples": [{"inputs": {"P1": 200000.0, "v1": 2.0, "h1": 0.0, "v2": 6.0, "h2": 5.0, "rho": 1000.0, "g": 9.81}, "outputs": {"P2": 134950.0}},
                     {"inputs": {"P1": 150000.0, "v1": 3.0, "h1": 10.0, "v2": 3.0, "h2": 0.0, "rho": 1000.0, "g": 9.80665}, "outputs": {"P2": 248066.5}}],
        "questions": [{"q": "Bernoulli equation with height: P1 = 200 kPa, v1 = 2 m/s, h1 = 0 m, v2 = 6 m/s, h2 = 5 m, rho = 1000 kg/m^3, g = 9.81 m/s^2",
                       "outputs": {"P2": 134950.0}},
                      {"q": "Bernoulli equation with height: P1 = 150 kPa, v1 = 3 m/s, h1 = 10 m, v2 = 3 m/s, h2 = 0 m, rho = 1000 kg/m^3, g = 9.80665 m/s^2",
                       "outputs": {"P2": 248066.5}},
                      {"q": "Bernoulli equation with height: P1 = 100 kPa, v1 = 1 m/s, h1 = 0 m, v2 = 1 m/s, h2 = 20 m, rho = 1000 kg/m^3, g = 9.81 m/s^2",
                       "outcome": "INVALID_INPUT"},
                      {"q": "Bernoulli equation with height: P1 = 200 kPa, v1 = 2 m/s, h1 = 0 m, v2 = 6 m/s, h2 = 5 m, rho = 1000 kg/m^3",
                       "outcome": "NEEDS_INPUT"}]},
})


# ================================================================================================ core addition, part 3 (2026-11-12)
# Wet steam (a saturated liquid-vapour mixture): the quality and the mixture's specific volume, enthalpy and entropy by
# the lever rule from saturated-liquid and saturated-vapour values THE USER SUPPLIES (vf, vg; hf, hfg; sf, sfg read from
# their own table at the mixture's pressure or temperature). No steam table is built in, so the result is only as good
# as the values given. Conventions: x is the vapour mass fraction (0 saturated liquid, 1 saturated vapour); each pair is
# given whole or not at all; exactly one of x, v, h, s states the mixture; a quality computed within 1e-12 of 0 or 1 is
# written as 0 or 1 (a value typed as hf + hfg is saturated vapour, not "outside the dome"). Values in SI internally
# (J/kg, J/(kg*K), m^3/kg); the question form accepts kJ/kg and kJ/(kg*K).
_U_WET = r"kJ/\(kg\*K\)|J/\(kg\*K\)|m\^3/kg|kJ/kg|MJ/kg|J/kg|%"
_PAIR_WET = re.compile(r"\b([A-Za-z][A-Za-z_0-9]*)\s*=\s*(" + _NUM + r")\s*(?:(" + _U_WET + r")(?![A-Za-z0-9^/*(]))?")
_WET_NAMES = {"vf": "vf", "vg": "vg", "hf": "hf", "hfg": "hfg", "sf": "sf", "sfg": "sfg", "x": "x", "v": "v", "h": "h", "s": "s"}
_WET_UNITS = {"vf": ("m^3/kg",), "vg": ("m^3/kg",), "v": ("m^3/kg",), "hf": ("kJ/kg", "MJ/kg", "J/kg"),
              "hfg": ("kJ/kg", "MJ/kg", "J/kg"), "h": ("kJ/kg", "MJ/kg", "J/kg"), "sf": ("kJ/(kg*K)", "J/(kg*K)"),
              "sfg": ("kJ/(kg*K)", "J/(kg*K)"), "s": ("kJ/(kg*K)", "J/(kg*K)")}
_TF_WET = r"wet steam|two[- ]phase (?:liquid[- ]vapou?r )?mixture|steam quality|quality of (?:a )?wet steam"


def parse_wet_steam(q):
    """'Wet steam: name = value unit, ...' (x a fraction or %); None for any other number, unknown name or unit."""
    if not isinstance(q, str) or not re.search(_TF_WET, q, re.I):
        return None
    out, rest = {}, q
    for m in _PAIR_WET.finditer(q):
        key = _WET_NAMES.get(m.group(1).lower())
        unit = m.group(3) or ""
        if key is None or key in out:
            return None
        if key == "x":
            if unit == "%":
                out[key] = float(m.group(2)) / 100.0
            elif unit:
                return None
            else:
                out[key] = float(m.group(2))
        else:
            if unit not in _WET_UNITS[key]:
                return None
            out[key] = f"{m.group(2)} {unit}"
        rest = rest.replace(m.group(0), " ", 1)
    if re.search(r"\d", re.sub(_TF_WET, " ", rest, flags=re.I)):
        return None
    return out


def wet_steam(vf=None, vg=None, hf=None, hfg=None, sf=None, sfg=None, x=None, v=None, h=None, s=None) -> dict:
    if (vf is None) != (vg is None) or (hf is None) != (hfg is None) or (sf is None) != (sfg is None):
        raise ValueError("give each saturated pair whole: vf with vg, hf with hfg, sf with sfg")
    if vf is None and hf is None and sf is None:
        raise ValueError("give at least one saturated pair (vf and vg, hf and hfg, or sf and sfg)")
    n = 0
    for val in (x, v, h, s):
        if val is not None:
            n += 1
    if n != 1:
        raise ValueError("state the mixture by exactly one of the quality x, or its v, h or s")
    if vf is not None:
        if not (math.isfinite(vf) and math.isfinite(vg) and 0 < vf < vg):
            raise ValueError("the specific volumes must satisfy 0 < vf < vg")
    if hf is not None:
        if not (math.isfinite(hf) and math.isfinite(hfg) and hfg > 0):
            raise ValueError("the enthalpy of vaporization hfg must be positive")
    if sf is not None:
        if not (math.isfinite(sf) and math.isfinite(sfg) and sfg > 0):
            raise ValueError("the entropy of vaporization sfg must be positive")
    if x is not None:
        if not (math.isfinite(x) and 0 <= x <= 1):
            raise ValueError("the quality x must be a fraction from 0 to 1")
        q = x
    else:
        if v is not None:
            if vf is None:
                raise ValueError("a mixture specific volume needs vf and vg")
            q = (v - vf) / (vg - vf)
        elif h is not None:
            if hf is None:
                raise ValueError("a mixture enthalpy needs hf and hfg")
            q = (h - hf) / hfg
        else:
            if sf is None:
                raise ValueError("a mixture entropy needs sf and sfg")
            q = (s - sf) / sfg
        if not math.isfinite(q):
            raise ValueError("the mixture property must be a finite number")
        if -1e-12 < q < 0:
            q = 0.0
        if 1 < q < 1 + 1e-12:
            q = 1.0
        if q < 0 or q > 1:
            raise ValueError("the state is not a wet mixture: the property lies outside the saturated-liquid to saturated-vapour range "
                             "(compressed liquid or superheated vapour)")
    out = {"x": q, "moisture": 1.0 - q}
    if vf is not None:
        out["v"] = vf + q * (vg - vf)
    if hf is not None:
        out["h"] = hf + q * hfg
        out["hg"] = hf + hfg
    if sf is not None:
        out["s"] = sf + q * sfg
        out["sg"] = sf + sfg
    return out


def verify_wet_steam(i, o) -> float:
    if "x" not in o or "moisture" not in o:
        return math.inf
    q = o["x"]
    if not (math.isfinite(q) and 0 <= q <= 1):
        return math.inf
    res = abs(o["moisture"] + q - 1.0)
    vf, vg, hf, hfg, sf, sfg = i.get("vf"), i.get("vg"), i.get("hf"), i.get("hfg"), i.get("sf"), i.get("sfg")
    if vf is not None:
        if "v" not in o:
            return math.inf
        res += abs(o["v"] - vf - q * (vg - vf)) / vg
        if i.get("v") is not None:
            res += abs(o["v"] - i["v"]) / vg
    if hf is not None:
        if "h" not in o or "hg" not in o:
            return math.inf
        sc = abs(hf) + hfg
        res += abs(o["h"] - hf - q * hfg) / sc + abs(o["hg"] - hf - hfg) / sc
        if i.get("h") is not None:
            res += abs(o["h"] - i["h"]) / sc
    if sf is not None:
        if "s" not in o or "sg" not in o:
            return math.inf
        sc = abs(sf) + sfg
        res += abs(o["s"] - sf - q * sfg) / sc + abs(o["sg"] - sf - sfg) / sc
        if i.get("s") is not None:
            res += abs(o["s"] - i["s"]) / sc
    if i.get("x") is not None:
        res += abs(q - i["x"])
    return res


def ref_wet_steam(vf=None, vg=None, hf=None, hfg=None, sf=None, sfg=None, x=None, v=None, h=None, s=None):
    """The lever rule as linear interpolation between the saturated end states (numpy interp), the quality of a stated
    mixture property found by root-finding on that interpolation (scipy brentq). The same definition by another route:
    a weaker reference, documented (no steam table is involved, so there is nothing else to compare with)."""
    import numpy as np
    from scipy.optimize import brentq
    ends = {}
    if vf is not None:
        ends["v"] = (vf, vg)
    if hf is not None:
        ends["h"] = (hf, hf + hfg)
    if sf is not None:
        ends["s"] = (sf, sf + sfg)
    if x is not None:
        q = x
    else:
        name, val = [(k, w) for k, w in (("v", v), ("h", h), ("s", s)) if w is not None][0]
        lo, hi = ends[name]
        q = brentq(lambda t: float(np.interp(t, [0.0, 1.0], [lo, hi])) - val, 0.0, 1.0, xtol=1e-16, rtol=8.9e-16, maxiter=500)
    out = {"x": q, "moisture": 1.0 - q}
    for k, (lo, hi) in ends.items():
        out[k] = float(np.interp(q, [0.0, 1.0], [lo, hi]))
    if hf is not None:
        out["hg"] = hf + hfg
    if sf is not None:
        out["sg"] = sf + sfg
    return out


def _wrong_wet_steam(vf=None, vg=None, hf=None, hfg=None, sf=None, sfg=None, x=None, v=None, h=None, s=None):
    """The liquid part forgotten: mixture property = x times the saturated-vapour value (v = x vg, h = x hg, s = x sg)."""
    gs = {}
    if vf is not None:
        gs["v"] = vg
    if hf is not None:
        gs["h"] = hf + hfg
    if sf is not None:
        gs["s"] = sf + sfg
    if x is not None:
        q = x
    elif v is not None:
        q = v / vg
    elif h is not None:
        q = h / (hf + hfg)
    else:
        q = s / (sf + sfg)
    out = {"x": q, "moisture": 1.0 - q}
    for k, g in gs.items():
        out[k] = q * g
    if hf is not None:
        out["hg"] = hf + hfg
    if sf is not None:
        out["sg"] = sf + sfg
    return out


def _rand_wet_steam(rng):
    d = {"vf": round(rng.uniform(0.001, 0.0016), 6), "vg": round(_log_uniform(rng, 0.01, 50.0), 5),
         "hf": round(rng.uniform(100e3, 1500e3), -2), "hfg": round(rng.uniform(800e3, 2400e3), -2),
         "sf": round(rng.uniform(300.0, 3500.0), 1), "sfg": round(rng.uniform(1500.0, 8000.0), 1)}
    mode = rng.choice(["x", "v", "h", "s"])
    q = rng.uniform(0.05, 0.98)
    keep = {"v": ("vf", "vg"), "h": ("hf", "hfg"), "s": ("sf", "sfg")}
    for k, pair in keep.items():
        if k != mode and rng.random() < 0.4:
            for p in pair:
                del d[p]
    if "vf" not in d and "hf" not in d and "sf" not in d:
        d.update(hf=round(rng.uniform(100e3, 1500e3), -2), hfg=round(rng.uniform(800e3, 2400e3), -2))
    if mode == "x":
        d["x"] = round(q, 4)
    elif mode == "v":
        d["v"] = float(f"{d['vf'] + q * (d['vg'] - d['vf']):.6g}")
    elif mode == "h":
        d["h"] = float(f"{d['hf'] + q * d['hfg']:.6g}")
    else:
        d["s"] = float(f"{d['sf'] + q * d['sfg']:.6g}")
    return d


def _wet_in(sym, kind, meaning):
    return {"symbol": sym, "kind": kind, "meaning": meaning, "required": False}


_X_WET = (0.5 - 0.001043) / (1.694 - 0.001043)

PROCEDURES.update({
    "wet_steam_mixture": {
        "name": "Wet steam: quality and mixture properties from saturated values", "topic": "thermodynamics / properties of pure substances",
        "purpose": "The quality x of a saturated liquid-vapour mixture and its specific volume, enthalpy and entropy by the lever "
                   "rule, from the saturated-liquid and saturated-vapour values the user supplies (vf, vg; hf, hfg; sf, sfg) and "
                   "either the quality or one mixture property (v, h or s).",
        "inputs": [_wet_in("vf", "specific_volume", "specific volume of the saturated liquid"),
                   _wet_in("vg", "specific_volume", "specific volume of the saturated vapour"),
                   _wet_in("hf", "specific_enthalpy", "specific enthalpy of the saturated liquid"),
                   _wet_in("hfg", "latent_heat", "enthalpy of vaporization hg - hf"),
                   _wet_in("sf", "specific_entropy", "specific entropy of the saturated liquid"),
                   _wet_in("sfg", "specific_entropy", "entropy of vaporization sg - sf"),
                   {"symbol": "x", "kind": "number", "meaning": "quality (vapour mass fraction), a fraction or %", "required": False},
                   _wet_in("v", "specific_volume", "specific volume of the mixture (instead of x)"),
                   _wet_in("h", "specific_enthalpy", "specific enthalpy of the mixture (instead of x)"),
                   _wet_in("s", "specific_entropy", "specific entropy of the mixture (instead of x)")],
        "outputs": [{"symbol": "x", "kind": "number", "meaning": "quality (vapour mass fraction)"},
                    {"symbol": "moisture", "kind": "number", "meaning": "moisture content 1 - x (liquid mass fraction)"},
                    {"symbol": "v", "kind": "specific_volume", "meaning": "specific volume of the mixture"},
                    {"symbol": "h", "kind": "specific_enthalpy", "meaning": "specific enthalpy of the mixture"},
                    {"symbol": "s", "kind": "specific_entropy", "meaning": "specific entropy of the mixture"},
                    {"symbol": "hg", "kind": "specific_enthalpy", "meaning": "specific enthalpy of the saturated vapour hf + hfg"},
                    {"symbol": "sg", "kind": "specific_entropy", "meaning": "specific entropy of the saturated vapour sf + sfg"}],
        "primary": "x", "conditions": ["saturated values at the mixture's own pressure (or temperature), supplied by the user",
                                       "each pair whole; 0 < vf < vg, hfg > 0, sfg > 0",
                                       "exactly one of x, v, h, s; 0 <= x <= 1 (otherwise not a wet mixture)"],
        "method": "lever rule: y = yf + x (yg - yf) for y = v, h, s; x = (y - yf)/(yg - yf) from a stated mixture property",
        "latex": r"x = \frac{h - h_f}{h_{fg}},\quad v = v_f + x\,(v_g - v_f),\quad h = h_f + x\,h_{fg},\quad s = s_f + x\,s_{fg}",
        "run": wet_steam, "verify": verify_wet_steam, "reference": ref_wet_steam, "parse": parse_wet_steam,
        "wrong": _wrong_wet_steam,
        "random_inputs": _rand_wet_steam,
        "templates": ["Wet steam: vf = 0.001043 m^3/kg, vg = 1.694 m^3/kg, hf = 417.5 kJ/kg, hfg = 2257.5 kJ/kg, "
                      "sf = 1.3028 kJ/(kg*K), sfg = 6.0562 kJ/(kg*K), x = 0.9",
                      "Wet steam: hf = 417.5 kJ/kg, hfg = 2257.5 kJ/kg, h = 2000 kJ/kg"],
        "examples": [{"inputs": {"vf": 0.001043, "vg": 1.694, "hf": 417500.0, "hfg": 2257500.0, "sf": 1302.8, "sfg": 6056.2, "x": 0.9},
                      "outputs": {"x": 0.9, "moisture": 0.1, "v": 1.5247043, "h": 2449250.0, "s": 6753.38, "hg": 2675000.0, "sg": 7359.0}},
                     {"inputs": {"hf": 417500.0, "hfg": 2257500.0, "h": 2000000.0},
                      "outputs": {"x": 1582500.0 / 2257500.0, "h": 2000000.0, "hg": 2675000.0}},
                     {"inputs": {"vf": 0.001043, "vg": 1.694, "sf": 1302.8, "sfg": 6056.2, "v": 0.5},
                      "outputs": {"x": _X_WET, "v": 0.5, "s": 1302.8 + _X_WET * 6056.2}}],
        "questions": [{"q": "Wet steam: vf = 0.001043 m^3/kg, vg = 1.694 m^3/kg, hf = 417.5 kJ/kg, hfg = 2257.5 kJ/kg, "
                            "sf = 1.3028 kJ/(kg*K), sfg = 6.0562 kJ/(kg*K), x = 90%",
                       "outputs": {"h": 2449250.0, "s": 6753.38, "v": 1.5247043}},
                      {"q": "Wet steam: hf = 417.5 kJ/kg, hfg = 2257.5 kJ/kg, h = 2000 kJ/kg", "outputs": {"x": 1582500.0 / 2257500.0}},
                      {"q": "Steam quality: vf = 0.001043 m^3/kg, vg = 1.694 m^3/kg, sf = 1.3028 kJ/(kg*K), sfg = 6.0562 kJ/(kg*K), v = 0.5 m^3/kg",
                       "outputs": {"x": _X_WET}},
                      {"q": "Wet steam: vf = 0.001043 m^3/kg, vg = 1.694 m^3/kg, v = 2 m^3/kg", "outcome": "INVALID_INPUT"},
                      {"q": "Wet steam: hf = 417.5 kJ/kg, hfg = 2257.5 kJ/kg, x = 1.2", "outcome": "INVALID_INPUT"},
                      {"q": "Wet steam: hf = 417.5 kJ/kg, hfg = 2257.5 kJ/kg, x = 0.9, h = 2449.25 kJ/kg", "outcome": "INVALID_INPUT"}]},
})
