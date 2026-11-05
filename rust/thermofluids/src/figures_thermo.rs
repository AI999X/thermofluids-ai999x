//! Thermodynamics and moist-air figures: air-standard cycles on P-V and T-s axes, processes on T-s and P-V axes, the
//! psychrometric chart. Every state is a run of the model's own relations (ideal-gas law, isentropic relations,
//! entropy change, humidity ratio); the stated starting states are said to be stated.

use crate::figures::{calc, own, qv, run};
use crate::figures_plot::{last, range, Plot};
use ax_engine::figkit::{self as k, num, q, Canvas, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;

/// cp and R of air (J/(kg K)) stated for the entropy of the cycles' states.
const CP: f64 = 1005.0;
const RA: f64 = 287.0;

fn gas_p(s: &Solved, n: f64, t: f64, v: f64) -> Option<f64> {
    calc(s, "ideal_gas_law", &[("n", n), ("T", t), ("V", v)])
}
fn gas_v(s: &Solved, n: f64, t: f64, p: f64) -> Option<f64> {
    calc(s, "ideal_gas_law", &[("n", n), ("T", t), ("P", p)])
}
fn gas_t(s: &Solved, n: f64, p: f64, v: f64) -> Option<f64> {
    calc(s, "ideal_gas_law", &[("n", n), ("P", p), ("V", v)])
}
fn gas_n(s: &Solved, p: f64, t: f64, v: f64) -> Option<f64> {
    calc(s, "ideal_gas_law", &[("P", p), ("T", t), ("V", v)])
}
fn isen_tv(s: &Solved, t1: f64, v1: f64, v2: f64) -> Option<f64> {
    calc(s, "isentropic_temperature_volume_air", &[("T1", t1), ("V1", v1), ("V2", v2)])
}
fn isen_vt(s: &Solved, t1: f64, v1: f64, t2: f64) -> Option<f64> {
    calc(s, "isentropic_temperature_volume_air", &[("T1", t1), ("V1", v1), ("T2", t2)])
}
fn isen_tp(s: &Solved, t1: f64, p1: f64, p2: f64) -> Option<f64> {
    calc(s, "isentropic_temperature_pressure_air", &[("T1", t1), ("P1", p1), ("P2", p2)])
}
/// The model's specific entropy change of air from (T0, P0) to (T, P).
fn ent(s: &Solved, t0: f64, p0: f64, t: f64, p: f64) -> Option<f64> {
    if (t - t0).abs() < 1e-12 * t0 && (p - p0).abs() < 1e-12 * p0 {
        return Some(0.0);
    }
    calc(s, "entropy_change_ideal_gas", &[("cp", CP), ("R", RA), ("T1", t0), ("T2", t), ("P1", p0), ("P2", p)])
}

/// A state of a cycle: volume (m³), pressure (Pa), temperature (K), specific entropy from state 1 (J/(kg K)).
type St = (f64, f64, f64, f64);

/// The states along a leg from model runs: `f` gives (V, P, T) at a fraction of the leg; entropy is added.
fn leg(s: &Solved, n: usize, s0: (f64, f64), f: impl Fn(f64) -> Option<(f64, f64, f64)>) -> Option<Vec<St>> {
    let mut v = vec![];
    for i in 0..n {
        let (vv, p, t) = f(i as f64 / (n - 1) as f64)?;
        v.push((vv, p, t, ent(s, s0.0, s0.1, t, p)?));
    }
    Some(v)
}

/// Both diagrams of a cycle (P-V over T-s when `pv`), the enclosed area shaded, each leg's direction marked, the
/// states numbered.
fn cycle_body(legs: &[Vec<St>], pv: bool, header: &str) -> Option<(String, f64)> {
    let h = if pv { 584.0 } else { 320.0 };
    let mut c = Canvas::new(h);
    c.text(96.0, 20.0, "start", "axp-mt", header);
    let all: Vec<St> = legs.iter().flatten().copied().collect();
    let panels: Vec<(&str, &str, (f64, f64, f64, f64), fn(&St) -> (f64, f64))> = if pv {
        vec![("Volume V (L)", "Pressure P (kPa)", (96.0, 44.0, 470.0, 236.0), pv_map as fn(&St) -> (f64, f64)),
             ("Specific entropy from state 1, s (J/(kg·K))", "Temperature T (K)", (96.0, 336.0, 470.0, 528.0),
              ts_map as fn(&St) -> (f64, f64))]
    } else {
        vec![("Specific entropy from state 1, s (J/(kg·K))", "Temperature T (K)", (96.0, 44.0, 470.0, 250.0),
              ts_map as fn(&St) -> (f64, f64))]
    };
    for (i, (xl, yl, r, map)) in panels.iter().enumerate() {
        let pts: Vec<(f64, f64)> = all.iter().map(|x| map(x)).collect();
        let xr = range(&pts.iter().map(|p| p.0).collect::<Vec<_>>(), false, false)?;
        let yr = range(&pts.iter().map(|p| p.1).collect::<Vec<_>>(), false, false)?;
        let pl = Plot::new(*r, (xr.0, xr.1, false), (yr.0, yr.1, false), &format!("c{i}"))?;
        if pv && i == 1 {
            c.bold(96.0, 318.0, "start", "T-s diagram");
        }
        pl.area(&mut c, &pts, "axp-s");
        pl.frame(&mut c, xl, yl);
        for lg in legs {
            let lp: Vec<(f64, f64)> = lg.iter().map(|x| map(x)).collect();
            pl.path(&mut c, &lp, "axp-c");
            if lp.len() >= 3 {
                let m = lp.len() / 2;
                pl.head(&mut c, lp[m - 1], lp[m]);
            }
        }
        // the states, numbered away from the middle of the cycle
        let cx = pts.iter().map(|p| pl.px(p.0)).sum::<f64>() / pts.len() as f64;
        let cy = pts.iter().map(|p| pl.py(p.1)).sum::<f64>() / pts.len() as f64;
        for (j, lg) in legs.iter().enumerate() {
            let p = map(&lg[0]);
            pl.dot(&mut c, p);
            let (x, y) = pl.p(p);
            let (dx, dy) = (x - cx, y - cy);
            let d = (dx * dx + dy * dy).sqrt().max(1.0);
            c.bold(x + dx / d * 16.0, y + dy / d * 16.0 + 5.0, "middle", &format!("{}", j + 1));
        }
    }
    Some((c.s, h))
}

fn pv_map(x: &St) -> (f64, f64) {
    (x.0 * 1000.0, x.1 / 1000.0)
}

fn ts_map(x: &St) -> (f64, f64) {
    (x.3, x.2)
}

fn state_list(sts: &[St]) -> String {
    sts.iter().enumerate().map(|(i, x)| format!("{}: {} K, {}", i + 1, num(x.2), q(x.1, "Pa"))).collect::<Vec<_>>().join("; ")
}

/// The Carnot cycle between the visitor's temperatures for air: a stated start of 10 bar in 1 L at Th (the amount
/// from the model's ideal-gas law), isothermal expansion to 2 L, the isentropes' ends from the model's isentropic
/// relation solved for the volume.
pub(crate) fn carnot(s: &Solved) -> Option<Figure> {
    let (th, tc, eta) = (s.get("Th")?, s.get("Tc")?, s.get("eta")?);
    if !(th > tc && tc > 0.0) {
        return None;
    }
    let (p1, v1) = (1.0e6, 1.0e-3);
    let n = gas_n(s, p1, th, v1)?;
    let v2 = 2.0 * v1;
    let v3 = isen_vt(s, th, v2, tc)?;
    let v4 = isen_vt(s, th, v1, tc)?;
    let s0 = (th, p1);
    let l1 = leg(s, 14, s0, |f| { let v = v1 * (v2 / v1).powf(f); Some((v, gas_p(s, n, th, v)?, th)) })?;
    let l2 = leg(s, 14, s0, |f| { let v = v2 * (v3 / v2).powf(f); let t = isen_tv(s, th, v2, v)?; Some((v, gas_p(s, n, t, v)?, t)) })?;
    let l3 = leg(s, 14, s0, |f| { let v = v3 * (v4 / v3).powf(f); Some((v, gas_p(s, n, tc, v)?, tc)) })?;
    let l4 = leg(s, 14, s0, |f| { let v = v4 * (v1 / v4).powf(f); let t = isen_tv(s, tc, v4, v)?; Some((v, gas_p(s, n, t, v)?, t)) })?;
    let (b, h) = cycle_body(&[l1, l2, l3, l4], true, &format!("η = {} between {} K and {} K", num(eta), num(th), num(tc)))?;
    Some(s.figure("cycle", "Carnot cycle on P-V and T-s diagrams",
                  &format!("Carnot cycle between Th = {} K and Tc = {} K for air: a stated start of 10 bar in 1 L ({} mol, \
                            from the model's ideal-gas law), isothermal expansion to 2 L. The isentropes end at V3 = {} L and \
                            V4 = {} L (the model's isentropic relation solved for the volume); pressures from its ideal-gas \
                            law, entropy from its entropy-change relation (cp = 1005, R = 287 J/(kg·K)). Marked: the model's \
                            efficiency η = {}.", num(th), num(tc), num(n), num(v3 * 1000.0), num(v4 * 1000.0), num(eta)),
                  &b, h))
}

/// The reversed Carnot cycle (refrigerator or heat pump) on T-s axes: its width the model's entropy change of air at
/// Tc for a stated pressure ratio of 2.
pub(crate) fn reversed_carnot(s: &Solved) -> Option<Figure> {
    let (th, tc, cop) = (s.get("Th")?, s.get("Tc")?, s.get("cop")?);
    if !(th > tc && tc > 0.0) {
        return None;
    }
    let p0 = 1.0e5;
    let ds = ent(s, tc, 2.0 * p0, tc, p0)?;
    if !(ds > 0.0) {
        return None;
    }
    let st = |x: f64, t: f64| -> St { (0.0, 0.0, t, x) };
    let line = |a: St, b: St| -> Vec<St> { (0..12).map(|i| { let f = i as f64 / 11.0; (0.0, 0.0, a.2 + (b.2 - a.2) * f, a.3 + (b.3 - a.3) * f) }).collect() };
    let (s1, s2, s3, s4) = (st(0.0, tc), st(ds, tc), st(ds, th), st(0.0, th));
    let legs = vec![line(s1, s2), line(s2, s3), line(s3, s4), line(s4, s1)];
    let fridge = s.key == "carnot_cop_refrigerator";
    let (b, h) = cycle_body(&legs, false, &format!("COP = {} ({})", num(cop), if fridge { "refrigerator" } else { "heat pump" }))?;
    let what = if fridge { "refrigerator" } else { "heat pump" };
    Some(s.figure("cycle", &format!("Reversed Carnot cycle: {what}"),
                  &format!("Reversed Carnot cycle ({what}) between Tc = {} K and Th = {} K on T-s axes, running \
                            counter-clockwise: heat taken in at Tc (1 to 2), isentropic compression (2 to 3), heat rejected at \
                            Th (3 to 4), isentropic expansion (4 to 1). The width is the model's entropy change of air at Tc \
                            for a stated pressure ratio of 2, {} J/(kg·K). Marked: the model's COP = {}.", num(tc), num(th),
                           num(ds), num(cop)), &b, h))
}

/// The air-standard Otto cycle: stated intake 1 bar, 300 K in 0.5 L and a peak of 1800 K; every state a run of the
/// model's isentropic and ideal-gas relations.
pub(crate) fn otto(s: &Solved) -> Option<Figure> {
    let (r, eta) = (s.get("r")?, s.get("eta")?);
    if !(r > 1.0) {
        return None;
    }
    let (p1, t1, v1, t3) = (1.0e5, 300.0, 5.0e-4, 1800.0);
    let n = gas_n(s, p1, t1, v1)?;
    let v2 = v1 / r;
    let t2 = isen_tv(s, t1, v1, v2)?;
    if !(t3 > t2) {
        return None;
    }
    let t4 = isen_tv(s, t3, v2, v1)?;
    let s0 = (t1, p1);
    let l1 = leg(s, 14, s0, |f| { let v = v1 * (v2 / v1).powf(f); let t = isen_tv(s, t1, v1, v)?; Some((v, gas_p(s, n, t, v)?, t)) })?;
    let l2 = leg(s, 10, s0, |f| { let t = t2 + (t3 - t2) * f; Some((v2, gas_p(s, n, t, v2)?, t)) })?;
    let l3 = leg(s, 14, s0, |f| { let v = v2 * (v1 / v2).powf(f); let t = isen_tv(s, t3, v2, v)?; Some((v, gas_p(s, n, t, v)?, t)) })?;
    let l4 = leg(s, 10, s0, |f| { let t = t4 + (t1 - t4) * f; Some((v1, gas_p(s, n, t, v1)?, t)) })?;
    let sts = [l1[0], l2[0], l3[0], l4[0]];
    let (b, h) = cycle_body(&[l1, l2, l3, l4], true, &format!("η = {} at r = {}", num(eta), num(r)))?;
    Some(s.figure("cycle", "Otto cycle on P-V and T-s diagrams",
                  &format!("Air-standard Otto cycle, compression ratio r = {} with a stated intake of 1 bar, 300 K in 0.5 L \
                            ({} mol, from the model's ideal-gas law) and a stated peak temperature of 1800 K, on P-V (top) \
                            and T-s (bottom) axes. Every point is a run of the model: temperatures on the isentropes from its \
                            isentropic temperature-volume relation, pressures from its ideal-gas law, entropy from its \
                            entropy-change relation. States {}. Marked: the model's efficiency η = {}.", num(r), num(n),
                           state_list(&sts), num(eta)), &b, h))
}

/// The air-standard Diesel cycle: stated intake 1 bar, 300 K in 0.5 L; heat added at constant pressure to rc times
/// the volume.
pub(crate) fn diesel(s: &Solved) -> Option<Figure> {
    let (r, rc, eta) = (s.get("r")?, s.get("rc")?, s.get("eta")?);
    if !(r > 1.0 && rc > 1.0 && rc < r) {
        return None;
    }
    let (p1, t1, v1) = (1.0e5, 300.0, 5.0e-4);
    let n = gas_n(s, p1, t1, v1)?;
    let v2 = v1 / r;
    let t2 = isen_tv(s, t1, v1, v2)?;
    let p2 = gas_p(s, n, t2, v2)?;
    let v3 = rc * v2;
    let t3 = gas_t(s, n, p2, v3)?;
    let t4 = isen_tv(s, t3, v3, v1)?;
    let s0 = (t1, p1);
    let l1 = leg(s, 14, s0, |f| { let v = v1 * (v2 / v1).powf(f); let t = isen_tv(s, t1, v1, v)?; Some((v, gas_p(s, n, t, v)?, t)) })?;
    let l2 = leg(s, 10, s0, |f| { let v = v2 * (v3 / v2).powf(f); Some((v, p2, gas_t(s, n, p2, v)?)) })?;
    let l3 = leg(s, 14, s0, |f| { let v = v3 * (v1 / v3).powf(f); let t = isen_tv(s, t3, v3, v)?; Some((v, gas_p(s, n, t, v)?, t)) })?;
    let l4 = leg(s, 10, s0, |f| { let t = t4 + (t1 - t4) * f; Some((v1, gas_p(s, n, t, v1)?, t)) })?;
    let sts = [l1[0], l2[0], l3[0], l4[0]];
    let (b, h) = cycle_body(&[l1, l2, l3, l4], true, &format!("η = {} at r = {}, rc = {}", num(eta), num(r), num(rc)))?;
    Some(s.figure("cycle", "Diesel cycle on P-V and T-s diagrams",
                  &format!("Air-standard Diesel cycle, compression ratio r = {}, cut-off ratio rc = {}, stated intake 1 bar, \
                            300 K in 0.5 L; P-V (top) and T-s (bottom). Isentropes from the model's isentropic relation, the \
                            constant-pressure heating from its ideal-gas law ({} mol), entropy from its entropy-change \
                            relation. States {}. Marked: the model's efficiency η = {}.", num(r), num(rc), num(n),
                           state_list(&sts), num(eta)), &b, h))
}

/// The air-standard Brayton cycle: stated compressor inlet 1 bar and 300 K, turbine inlet 1400 K, 1 mol of air.
pub(crate) fn brayton(s: &Solved) -> Option<Figure> {
    let (rp, eta) = (s.get("rp")?, s.get("eta")?);
    if !(rp > 1.0) {
        return None;
    }
    let (p1, t1, t3, n) = (1.0e5, 300.0, 1400.0, 1.0);
    let p2 = rp * p1;
    let t2 = isen_tp(s, t1, p1, p2)?;
    if !(t3 > t2) {
        return None;
    }
    let t4 = isen_tp(s, t3, p2, p1)?;
    let s0 = (t1, p1);
    let l1 = leg(s, 14, s0, |f| { let p = p1 * (p2 / p1).powf(f); let t = isen_tp(s, t1, p1, p)?; Some((gas_v(s, n, t, p)?, p, t)) })?;
    let l2 = leg(s, 10, s0, |f| { let t = t2 + (t3 - t2) * f; Some((gas_v(s, n, t, p2)?, p2, t)) })?;
    let l3 = leg(s, 14, s0, |f| { let p = p2 * (p1 / p2).powf(f); let t = isen_tp(s, t3, p2, p)?; Some((gas_v(s, n, t, p)?, p, t)) })?;
    let l4 = leg(s, 10, s0, |f| { let t = t4 + (t1 - t4) * f; Some((gas_v(s, n, t, p1)?, p1, t)) })?;
    let sts = [l1[0], l2[0], l3[0], l4[0]];
    let (b, h) = cycle_body(&[l1, l2, l3, l4], true, &format!("η = {} at rp = {}", num(eta), num(rp)))?;
    Some(s.figure("cycle", "Brayton cycle on P-V and T-s diagrams",
                  &format!("Air-standard Brayton cycle, pressure ratio rp = {}, stated compressor inlet 1 bar and 300 K, \
                            turbine inlet 1400 K, for 1 mol of air; P-V (top) and T-s (bottom). Temperatures on the \
                            isentropes from the model's isentropic temperature-pressure relation, volumes from its ideal-gas \
                            law, entropy from its entropy-change relation. States {}. Marked: the model's efficiency η = {}.",
                           num(rp), state_list(&sts), num(eta)), &b, h))
}

/// An isobar on T-s axes: (s, T) at 14 temperatures over [ta, tb], the entropy from `ent_of`.
fn isobar(p: f64, ta: f64, tb: f64, ent_of: &dyn Fn(f64, f64) -> Option<f64>) -> Vec<(f64, f64)> {
    k::span(ta, tb, 14, false).into_iter().filter_map(|t| ent_of(t, p).map(|x| (x, t))).collect()
}

fn ts_body(isobars: &[(String, Vec<(f64, f64)>)], header: &str) -> Option<(Canvas, Plot)> {
    let all: Vec<(f64, f64)> = isobars.iter().flat_map(|x| x.1.iter().copied()).collect();
    let xr = range(&all.iter().map(|p| p.0).collect::<Vec<_>>(), false, false)?;
    let yr = range(&all.iter().map(|p| p.1).collect::<Vec<_>>(), false, false)?;
    let mut c = Canvas::new(330.0);
    let pl = Plot::new((96.0, 44.0, 440.0, 258.0), (xr.0, xr.1, false), (yr.0, yr.1, false), "t")?;
    pl.frame(&mut c, "Specific entropy from state 1, s (J/(kg·K))", "Temperature T (K)");
    for (n, pts) in isobars {
        pl.path(&mut c, pts, "axp-c4");
        if let Some(e) = last(pts) {
            pl.label(&mut c, e, 6.0, 4.0, "axp-lt", n);
        }
    }
    c.text(96.0, 20.0, "start", "axp-mt", header);
    Some((c, pl))
}

/// Real and ideal adiabatic compression of air on T-s axes: the two isobars from the model's entropy change (cp =
/// 1005, R = 287 stated), the ideal outlet 2s from its isentropic relation, the real outlet 2 the model's answer.
pub(crate) fn compressor(s: &Solved) -> Option<Figure> {
    let (t1, p1, p2, eta, t2) = (s.get("T1")?, s.get("P1")?, s.get("P2")?, s.get("eta")?, s.get("T2")?);
    if !(p2 > p1 && t2 > t1) {
        return None;
    }
    let t2s = isen_tp(s, t1, p1, p2)?;
    let e = |t: f64, p: f64| ent(s, t1, p1, t, p);
    let (ta, tb) = (t1 * 0.92, t2 * 1.06);
    let iso = vec![(format!("P1 = {}", q(p1, "Pa")), isobar(p1, ta, tb, &e)),
                   (format!("P2 = {}", q(p2, "Pa")), isobar(p2, ta, tb, &e))];
    let (mut c, pl) = ts_body(&iso, &format!("T2 = {} K (ideal {} K)", num(t2), num(t2s)))?;
    let s1 = (0.0, t1);
    let s2s = (e(t2s, p2)?, t2s);
    let s2 = (e(t2, p2)?, t2);
    pl.dashed(&mut c, &[s1, s2s]);
    pl.path(&mut c, &[s1, s2], "axp-c");
    pl.head(&mut c, s1, s2);
    for (p, t, dx) in [(s1, "1", -10.0), (s2s, "2s", -10.0), (s2, "2", 10.0)] {
        pl.dot(&mut c, p);
        pl.label(&mut c, p, dx, 5.0, "axp-bt", t);
    }
    pl.mark(&mut c, s2);
    Some(s.figure("process", "Real and ideal compression on a T-s diagram",
                  &format!("Adiabatic compression of air from {} K and {} to {} with an isentropic efficiency of {} on T-s \
                            axes, with the two isobars from the model's ideal-gas entropy change (cp = 1005, R = 287 J/(kg·K), \
                            stated). The ideal outlet 2s ({} K) comes from its isentropic temperature-pressure relation; the \
                            real outlet 2 is the model's T2 = {} K, at higher entropy.", num(t1), q(p1, "Pa"), q(p2, "Pa"),
                           num(eta), num(t2s), num(t2)), c.body(), c.h))
}

/// The entropy change between two states on T-s axes: both isobars are runs of the model's entropy relation from
/// state 1 with the visitor's cp and R.
pub(crate) fn entropy(s: &Solved) -> Option<Figure> {
    let (t1, t2, p1, p2, ds) = (s.get("T1")?, s.get("T2")?, s.get("P1")?, s.get("P2")?, s.get("ds")?);
    let e = |t: f64, p: f64| -> Option<f64> {
        if (t - t1).abs() < 1e-12 * t1 && (p - p1).abs() < 1e-12 * p1 { Some(0.0) } else { own(s, &[("T2", t), ("P2", p)]) }
    };
    let (ta, tb) = (t1.min(t2) * 0.9, t1.max(t2) * 1.08);
    let iso = vec![(format!("P1 = {}", q(p1, "Pa")), isobar(p1, ta, tb, &e)),
                   (format!("P2 = {}", q(p2, "Pa")), isobar(p2, ta, tb, &e))];
    let (mut c, pl) = ts_body(&iso, &format!("Δs = {} J/(kg·K)", num(ds)))?;
    let (a, b) = ((0.0, t1), (ds, t2));
    pl.dashed(&mut c, &[a, b]);
    pl.dot(&mut c, a);
    pl.mark(&mut c, b);
    pl.label(&mut c, a, -10.0, 5.0, "axp-bt", "1");
    pl.label(&mut c, b, 10.0, 5.0, "axp-bt", "2");
    Some(s.figure("process", "Entropy change between two states on a T-s diagram",
                  &format!("Entropy change of an ideal gas (cp = {}, R = {}) from {} K, {} to {} K, {} on T-s axes; both \
                            isobars are 14 runs each of the model's entropy relation from state 1. Marked: state 2, Δs = {} \
                            J/(kg·K).", qv(s, "cp", s.get("cp")?), qv(s, "R", s.get("R")?), num(t1), q(p1, "Pa"), num(t2),
                           q(p2, "Pa"), num(ds)), c.body(), c.h))
}

/// Isothermal expansion on P-V axes: the isotherm from the model's ideal-gas law at 41 volumes, the work shaded.
pub(crate) fn isothermal(s: &Solved) -> Option<Figure> {
    let (n, t, v1, v2, w) = (s.get("n")?, s.get("T")?, s.get("V1")?, s.get("V2")?, s.get("W")?);
    let (va, vb) = (v1.min(v2), v1.max(v2));
    if !(va > 0.0) || vb == va {
        return None;
    }
    let vs = k::span(va * 0.8, vb * 1.2, 41, false);
    let pts: Vec<(f64, f64)> = vs.iter().filter_map(|v| gas_p(s, n, t, *v).map(|p| (v * 1000.0, p / 1000.0))).collect();
    if pts.len() < 20 {
        return None;
    }
    let (pa, pb) = (gas_p(s, n, t, v1)?, gas_p(s, n, t, v2)?);
    let mut c = Canvas::new(320.0);
    let ys: Vec<f64> = pts.iter().map(|p| p.1).collect();
    let yr = range(&ys, false, true)?;
    let pl = Plot::new((96.0, 44.0, 470.0, 252.0), (va * 800.0, vb * 1200.0, false), (yr.0, yr.1, false), "i")?;
    let mut area: Vec<(f64, f64)> = pts.iter().copied().filter(|p| p.0 >= va * 1000.0 - 1e-12 && p.0 <= vb * 1000.0 + 1e-12).collect();
    area.insert(0, (va * 1000.0, 0.0));
    area.insert(1, (va * 1000.0, if v1 < v2 { pa } else { pb } / 1000.0));
    area.push((vb * 1000.0, if v1 < v2 { pb } else { pa } / 1000.0));
    area.push((vb * 1000.0, 0.0));
    pl.area(&mut c, &area, "axp-s");
    pl.frame(&mut c, "Volume V (L)", "Pressure P (kPa)");
    pl.path(&mut c, &pts, "axp-c");
    let (a, b) = ((v1 * 1000.0, pa / 1000.0), (v2 * 1000.0, pb / 1000.0));
    pl.dot(&mut c, a);
    pl.mark(&mut c, b);
    pl.label(&mut c, a, 8.0, -8.0, "axp-vt", &format!("P1 = {}", q(pa, "Pa")));
    pl.label(&mut c, b, 8.0, -8.0, "axp-vt", &format!("P2 = {}", q(pb, "Pa")));
    pl.label(&mut c, ((va + vb) * 500.0, pa.min(pb) / 2000.0), 0.0, 0.0, "axp-mt", &format!("W = {}", q(w, "J")));
    c.text(96.0, 20.0, "start", "axp-mt", &format!("T = {} K, n = {} mol", num(t), num(n)));
    Some(s.figure("process", "Isothermal expansion and its work on a P-V diagram",
                  &format!("Isothermal process of {} mol of ideal gas at {} K from {} L to {} L on P-V axes; pressures from the \
                            model's ideal-gas law at 41 volumes. The shaded area under the isotherm is the work; marked: the \
                            model's W = {}, P1 = {}, P2 = {}.", num(n), num(t), num(v1 * 1000.0), num(v2 * 1000.0), q(w, "J"),
                           q(pa, "Pa"), q(pb, "Pa")), c.body(), c.h))
}

// ------------------------------------------------------------------------------------------------ moist air
/// The model's humidity ratio (kg/kg) of air at T (K), relative humidity rh, pressure p.
fn hum(s: &Solved, t: f64, rh: f64, p: f64) -> Option<f64> {
    let a = run(s, "humidity_ratio_from_relative_humidity", &[("T", Input::Float(t)), ("RH", Input::Float(rh)),
                                                                ("P", Input::Float(p))])?;
    k::num_of(&a, "W")
}

/// Lines of constant relative humidity on (T in °C, W in g/kg): the model's humidity ratio every 2 °C.
fn rh_lines(s: &Solved, p: f64, rhs: &[f64], ta: f64, tb: f64) -> Vec<(f64, Vec<(f64, f64)>)> {
    let ts = k::span(ta, tb, 26, false);
    rhs.iter().map(|rh| (*rh, ts.iter().filter_map(|t| hum(s, t + 273.15, *rh, p).map(|w| (*t, w * 1000.0))).collect::<Vec<_>>()))
        .filter(|x| x.1.len() >= 2).collect()
}

/// A psychrometric chart (humidity ratio against dry-bulb temperature), the line of `hl` relative humidity drawn in
/// the accent colour; the caller adds its state points.
fn psy_chart(lines: &[(f64, Vec<(f64, f64)>)], hl: Option<f64>, extra: &[(f64, f64)], header: &str) -> Option<(Canvas, Plot)> {
    let xs: Vec<f64> = lines.iter().flat_map(|l| l.1.iter().map(|p| p.0)).chain(extra.iter().map(|p| p.0)).collect();
    let ys: Vec<f64> = lines.iter().flat_map(|l| l.1.iter().map(|p| p.1)).chain(extra.iter().map(|p| p.1)).collect();
    let xr = range(&xs, false, false)?;
    let yr = range(&ys, false, true)?;
    let mut c = Canvas::new(340.0);
    let pl = Plot::new((80.0, 44.0, 446.0, 278.0), (xr.0, xr.1, false), (0.0, yr.1, false), "y")?;
    pl.frame(&mut c, "Dry-bulb temperature T (°C)", "Humidity ratio W (g/kg)");
    let mut used: Vec<f64> = vec![];
    for (rh, pts) in lines {
        let mine = hl.map(|h| (h - rh).abs() < 1e-9).unwrap_or(false);
        let cls = if mine { "axp-c" } else if (*rh - 1.0).abs() < 1e-9 { "axp-c3" } else { "axp-c4" };
        pl.path(&mut c, pts, cls);
        // the label where the line leaves the chart (its top or its right edge)
        let inside: Vec<(f64, f64)> = pts.iter().copied().filter(|p| p.1 <= yr.1).collect();
        if let Some(e) = last(&inside) {
            let (x, mut y) = pl.p(e);
            y = y.max(52.0);
            while used.iter().any(|u| (u - y).abs() < 12.0 && x > 400.0) {
                y += 12.0;
            }
            used.push(y);
            c.text(x + 4.0, y + 4.0, "start", if mine { "axp-vt" } else { "axp-lt" }, &format!("{} %", num(rh * 100.0)));
        }
    }
    c.text(80.0, 20.0, "start", "axp-mt", header);
    Some((c, pl))
}

/// The pressure of a psychrometric request (stated 101325 Pa for a capability that takes none).
fn press(s: &Solved) -> (f64, bool) {
    match s.get("P") {
        Some(p) if p > 0.0 => (p, false),
        _ => (101_325.0, true),
    }
}

fn span_t(t: f64, others: &[f64]) -> (f64, f64) {
    let lo = others.iter().copied().fold(t, f64::min);
    let hi = others.iter().copied().fold(t, f64::max);
    ((lo - 8.0).min(0.0), (hi + 8.0).max(45.0))
}

/// The dew point on the chart: the state cooled at constant moisture until it saturates.
pub(crate) fn psychro_dew(s: &Solved) -> Option<Figure> {
    let (t, rh, td, pv) = (s.get("T")?, s.get("RH")?, s.get("T_dew")?, s.get("p_v")?);
    let (p, stated) = press(s);
    let w = hum(s, t, rh, p)? * 1000.0;
    let (tc, tdc) = (t - 273.15, td - 273.15);
    let (ta, tb) = span_t(tc, &[tdc]);
    let lines = rh_lines(s, p, &[0.2, 0.4, 0.6, 0.8, 1.0], ta, tb);
    let (mut c, pl) = psy_chart(&lines, None, &[(tc, w)], &format!("dew point {} °C", num(tdc)))?;
    pl.path(&mut c, &[(tc, w), (tdc, w)], "axp-c");
    pl.head(&mut c, (tc, w), (tdc, w));
    pl.dot(&mut c, (tc, w));
    pl.mark(&mut c, (tdc, w));
    pl.label(&mut c, (tc, w), 8.0, 16.0, "axp-vt", &format!("{} °C, {} %", num(tc), num(rh * 100.0)));
    pl.label(&mut c, (tdc, w), -8.0, -10.0, "axp-mt", &format!("{} °C", num(tdc)));
    Some(s.chart("chart", "Dew point on the psychrometric chart",
                 &format!("Psychrometric chart at {} Pa{} (the model's humidity ratio at 20 to 80 % and 100 % relative \
                           humidity) with the given air at {} °C and {} % ({} g/kg) cooled at constant moisture until it \
                           saturates. Marked: the model's dew point {} °C and vapour pressure {}.", num(p),
                          if stated { ", stated" } else { "" }, num(tc), num(rh * 100.0), num(w), num(tdc), q(pv, "Pa")),
                 c.body()))
}

/// The humidity ratio's state on the chart; the second chart draws the state's own relative-humidity line with
/// guides to both axes.
pub(crate) fn psychro_state(s: &Solved) -> Vec<Option<Figure>> {
    let (Some(t), Some(rh), Some(w), Some(pv)) = (s.get("T"), s.get("RH"), s.get("W"), s.get("p_v")) else { return vec![] };
    let (p, _) = press(s);
    let (tc, wg) = (t - 273.15, w * 1000.0);
    let (ta, tb) = span_t(tc, &[]);
    let mut rhs = vec![0.2, 0.4, 0.6, 0.8, 1.0];
    let lines = rh_lines(s, p, &rhs, ta, tb);
    let one = psy_chart(&lines, None, &[(tc, wg)], &format!("W = {} g/kg", num(wg))).map(|(mut c, pl)| {
        pl.mark(&mut c, (tc, wg));
        pl.label(&mut c, (tc, wg), 10.0, -8.0, "axp-mt", &format!("{} °C, {} %", num(tc), num(rh * 100.0)));
        s.chart("chart", "Psychrometric chart: humidity ratio against temperature",
                &format!("Psychrometric chart at {} Pa: humidity ratio against dry-bulb temperature at 20, 40, 60, 80 and \
                          100 % relative humidity, the model's humidity ratio every 2 °C. Marked: T = {} °C, RH = {} %, W = \
                          {} g/kg.", num(p), num(tc), num(rh * 100.0), num(wg)), c.body())
    });
    if !rhs.iter().any(|x| (x - rh).abs() < 1e-9) {
        rhs.push(rh);
    }
    let mut lines2 = lines.clone();
    if !lines2.iter().any(|l| (l.0 - rh).abs() < 1e-9) {
        lines2.extend(rh_lines(s, p, &[rh], ta, tb));
    }
    let two = psy_chart(&lines2, Some(rh), &[(tc, wg)], &format!("{} % line through the state", num(rh * 100.0))).map(|(mut c, pl)| {
        pl.dashed(&mut c, &[(tc, 0.0), (tc, wg), (pl.x.0, wg)]);
        pl.mark(&mut c, (tc, wg));
        pl.label(&mut c, (pl.x.0, wg), 6.0, -6.0, "axp-vt", &format!("W = {} g/kg", num(wg)));
        pl.label(&mut c, (tc, wg), 10.0, 16.0, "axp-vt", &format!("pv = {}", q(pv, "Pa")));
        s.chart("chart-2", "Psychrometric chart with the state point",
                &format!("Psychrometric chart at {} Pa (the model's humidity ratio every 2 °C) with the given state's own \
                          {} % relative-humidity line in the accent colour: air at {} °C. Marked: W = {} g/kg, vapour \
                          pressure {}.", num(p), num(rh * 100.0), num(tc), num(wg), q(pv, "Pa")), c.body())
    });
    vec![one, two]
}

/// The wet-bulb temperature on the chart: the state and the saturated state at its wet-bulb temperature (the
/// model's saturated humidity ratio there).
pub(crate) fn psychro_wet_bulb(s: &Solved) -> Option<Figure> {
    let (t, rh, twb, w) = (s.get("T")?, s.get("RH")?, s.get("T_wb")?, s.get("W")?);
    let (p, _) = press(s);
    let ws = hum(s, twb, 1.0, p)? * 1000.0;
    let (tc, twc, wg) = (t - 273.15, twb - 273.15, w * 1000.0);
    let (ta, tb) = span_t(tc, &[twc]);
    let lines = rh_lines(s, p, &[0.2, 0.4, 0.6, 0.8, 1.0], ta, tb);
    let (mut c, pl) = psy_chart(&lines, None, &[(tc, wg), (twc, ws)], &format!("wet bulb {} °C", num(twc)))?;
    pl.path(&mut c, &[(tc, wg), (twc, ws)], "axp-c");
    pl.head(&mut c, (tc, wg), (twc, ws));
    pl.dot(&mut c, (tc, wg));
    pl.mark(&mut c, (twc, ws));
    pl.label(&mut c, (tc, wg), 8.0, 16.0, "axp-vt", &format!("{} °C, {} g/kg", num(tc), num(wg)));
    pl.label(&mut c, (twc, ws), 10.0, -12.0, "axp-mt", &format!("{} °C, {} g/kg", num(twc), num(ws)));
    Some(s.chart("chart", "Wet-bulb temperature on the psychrometric chart",
                 &format!("Psychrometric chart at {} Pa (the model's humidity ratio at 20 to 80 % and 100 % relative humidity) \
                           with the given air at {} °C and {} % ({} g/kg, returned) followed toward saturation at its \
                           wet-bulb temperature. Marked: the model's T_wb = {} °C and the saturated humidity ratio there, {} \
                           g/kg.", num(p), num(tc), num(rh * 100.0), num(wg), num(twc), num(ws)), c.body()))
}

/// The wet-bulb temperature over dry-bulb temperatures and relative humidities at the visitor's pressure, each cell a
/// run of the model.
pub(crate) fn wet_bulb_table(s: &Solved) -> Option<Figure> {
    let (p, _) = press(s);
    let tcs = [10.0, 15.0, 20.0, 25.0, 30.0, 35.0, 40.0];
    let rhs = [0.2, 0.4, 0.6, 0.8, 1.0];
    let mut m = vec![];
    for tc in tcs {
        let mut row = vec![];
        for rh in rhs {
            let a = s.with_input(&[("T", Input::Float(tc + 273.15)), ("RH", Input::Float(rh)), ("P", Input::Float(p))]);
            row.push(a.and_then(|a| k::num_of(&a, "T_wb")).map(|x| (x - 273.15) * 10.0).map(|x| x.round() / 10.0)
                .unwrap_or(f64::NAN));
        }
        m.push(row);
    }
    if m.iter().flatten().filter(|x| x.is_finite()).count() < 20 {
        return None;
    }
    let rows: Vec<String> = tcs.iter().map(|t| format!("{} °C", num(*t))).collect();
    let cols: Vec<String> = rhs.iter().map(|r| format!("{}%", num(r * 100.0))).collect();
    let twb = s.get("T_wb")?;
    let body = k::heat(&m, &rows, &cols, &format!("given: {} °C", num(twb - 273.15)));
    let corner = m[6][0];
    Some(s.chart("heatmap", "Wet-bulb temperature",
                 &format!("Wet-bulb temperature at {} Pa for dry-bulb temperatures 10 to 40 °C (rows) and relative humidities \
                           20 to 100 % (columns), each cell computed by the model (°C, one decimal). At 100 % the wet bulb \
                           equals the dry bulb; at 40 °C and 20 % it is {} °C. The given air's wet bulb: {} °C.", num(p),
                          num(corner), num(twb - 273.15)), &body))
}
