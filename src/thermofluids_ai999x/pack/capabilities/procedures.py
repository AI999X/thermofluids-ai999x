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
