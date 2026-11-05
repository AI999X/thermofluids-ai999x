//! Thermofluids' own live figures (ax_engine::figkit): every capability's published figure kinds (tanks and pipes
//! with their solved pressures and speeds, friction-factor and psychrometric charts, temperature profiles through
//! walls and along exchangers, cycles on P-V and T-s axes, maps of a correlation over two inputs), drawn for the
//! visitor's inputs from what the model computes: the answer, and further solves of the model's own capabilities.
//! A figure the inputs make impossible is left out, never drawn wrong. Written by hand for the Rust build.

use crate::figures_flow as fl;
use crate::figures_heat as ht;
use crate::figures_thermo as th;
use ax_engine::figkit::{self as k, num, Canvas, Solved};
use ax_engine::figures::Figure;
use ax_engine::{Answer, Input};

/// The figures for one solved request (the dispatcher ax_engine calls: Pack::figures).
pub fn live(s: &Solved) -> Vec<Figure> {
    let v: Vec<Option<Figure>> = match s.key {
        // maps of one result over two inputs
        "churchill_bernstein_cylinder" => vec![field_around(s, "Nu", "Re", "Pr", (false, false))],
        "churchill_chu_vertical_plate" => vec![churchill_chu(s)],
        "buoyant_force" => vec![field_around(s, "F", "rho", "V", (false, false))],
        "dynamic_pressure" => vec![field_around(s, "q", "v", "rho", (false, false))],
        "kinematic_viscosity" => vec![field_around(s, "nu", "mu", "rho", (false, false))],
        "minor_loss_head" => vec![field_around(s, "hL", "v", "K", (false, false))],
        "mach_number" => vec![field_around(s, "M", "v", "a", (false, false))],
        "froude_number" => vec![field_around(s, "Fr", "v", "h", (false, false))],
        "dittus_boelter_heating" | "dittus_boelter_cooling" => vec![field_around(s, "Nu", "Re", "Pr", (false, false))],
        "flat_plate_laminar_average_nusselt" => vec![field_around(s, "Nu", "ReL", "Pr", (false, false))],
        "flat_plate_laminar_local_nusselt" => vec![field_around(s, "Nux", "Rex", "Pr", (false, false))],
        "rayleigh_number" => vec![field_around(s, "Ra", "Gr", "Pr", (false, false))],
        "effectiveness_parallel_flow" => vec![effectiveness_map(s)],
        "humidity_ratio_vapour_pressure" => vec![field_around(s, "W", "pv", "P", (false, false))],
        "moist_air_enthalpy" => vec![field_around(s, "h", "W", "T", (false, false))],
        // fluids
        "hydrostatic_gauge_pressure" => vec![fl::tank(s, false)],
        "absolute_pressure_depth" => vec![fl::tank(s, true)],
        "continuity_speed" => vec![fl::contraction(s)],
        "bernoulli_horizontal" => vec![fl::bernoulli(s)],
        "hagen_poiseuille" => vec![fl::poiseuille(s)],
        "normal_shock_mach_air" | "normal_shock_pressure_air" => vec![fl::shock(s)],
        "colebrook_friction_factor" => fl::colebrook(s),
        "laminar_friction_factor" | "blasius_friction_factor" => vec![fl::smooth_chart(s)],
        "swamee_jain_friction_factor" | "haaland_friction_factor" => vec![fl::explicit_chart(s)],
        "blasius_displacement_thickness" | "blasius_momentum_thickness" => vec![fl::boundary_layer(s)],
        // heat transfer
        "plane_wall_conduction" => vec![ht::plane_wall(s)],
        "cylinder_conduction" => vec![ht::radial(s, false)],
        "sphere_conduction" => vec![ht::radial(s, true)],
        "wall_generation_max_temperature" => vec![ht::generation(s)],
        "overall_u_plane_wall" => vec![ht::network(s)],
        "radiation_parallel_plates" => vec![ht::plates(s)],
        "infinite_fin_heat_rate" => vec![ht::long_fin(s)],
        "pin_fin_efficiency" => vec![ht::pin_fin(s)],
        "hx_effectiveness_ntu" => vec![ht::ntu_chart(s), ht::ntu_profile(s)],
        "lmtd" => vec![ht::lmtd_profile(s)],
        "critical_insulation_radius" => vec![ht::family(s, "r", "h", "k")],
        "lumped_capacitance_temperature" => vec![ht::family(s, "T", "Ti", "h")],
        "lumped_time_constant" => vec![ht::family(s, "tau", "h", "rho")],
        // thermodynamics and moist air
        "carnot_efficiency" => vec![th::carnot(s)],
        "carnot_cop_refrigerator" | "carnot_cop_heat_pump" => vec![th::reversed_carnot(s)],
        "otto_efficiency_air" => vec![th::otto(s)],
        "diesel_efficiency_air" => vec![th::diesel(s)],
        "brayton_efficiency_air" => vec![th::brayton(s)],
        "compressor_outlet_temperature_air" => vec![th::compressor(s)],
        "isothermal_work_ideal_gas" => vec![th::isothermal(s)],
        "entropy_change_ideal_gas" => vec![th::entropy(s)],
        "dew_point_temperature" => vec![th::psychro_dew(s)],
        "humidity_ratio_from_relative_humidity" => th::psychro_state(s),
        "wet_bulb_temperature" => vec![th::psychro_wet_bulb(s), th::wet_bulb_table(s)],
        _ => vec![],
    };
    v.into_iter().flatten().collect()
}

// ------------------------------------------------------------------------------------------------ model solves
/// The single unknown of a formula (or the result of a procedure's primary output) solved by the model with these
/// inputs (SI numbers).
pub(crate) fn calc(s: &Solved, key: &str, set: &[(&str, f64)]) -> Option<f64> {
    if set.iter().any(|x| !x.1.is_finite()) {
        return None;
    }
    let v: Vec<(&str, Input)> = set.iter().map(|(n, x)| (*n, Input::Float(*x))).collect();
    let a = s.solve_other(key, &v)?;
    if let Some(pr) = s.p.procedure(key) {
        return k::num_of(&a, &pr.primary);
    }
    a.value.filter(|x| x.is_finite())
}

/// A procedure of the model solved with these inputs.
pub(crate) fn run(s: &Solved, key: &str, set: &[(&str, Input)]) -> Option<Answer> {
    s.solve_other(key, set)
}

/// This capability's result (a formula's left-hand side, a procedure's primary output) for its inputs as in the
/// request with some replaced (SI): whichever quantity the visitor solved for, every other quantity is set.
pub(crate) fn own(s: &Solved, set: &[(&str, f64)]) -> Option<f64> {
    if let Some(rel) = s.p.relation(s.key) {
        let mut v: Vec<(&str, f64)> = vec![];
        for sy in &rel.symbols {
            if sy.name == rel.lhs {
                continue;
            }
            let x = match set.iter().find(|x| x.0 == sy.name) {
                Some(x) => x.1,
                None => s.get(&sy.name)?,
            };
            v.push((sy.name.as_str(), x));
        }
        return calc(s, s.key, &v);
    }
    let pr = s.p.procedure(s.key)?;
    let a = s.with(set)?;
    k::num_of(&a, &pr.primary)
}

/// The unit of a quantity of a capability, as its kind states it ("" for a pure number).
pub(crate) fn unit(s: &Solved, key: &str, sym: &str) -> String {
    let kind = if let Some(r) = s.p.relation(key) {
        r.sym(sym).map(|x| x.kind.clone())
    } else if let Some(p) = s.p.procedure(key) {
        p.inputs.iter().find(|i| i.symbol == sym).map(|i| i.kind.clone())
            .or_else(|| p.outputs.iter().find(|o| o.symbol == sym).map(|o| o.kind.clone()))
    } else {
        None
    };
    let u = kind.and_then(|kd| s.p.kinds.get(&kd).map(|x| x.unit.clone())).unwrap_or_default();
    if u == "1" { String::new() } else { u }
}

/// What a quantity is, in the capability's own words ("density of the fluid"); the symbol when it has none.
pub(crate) fn desc(s: &Solved, key: &str, sym: &str) -> String {
    let d = if let Some(r) = s.p.relation(key) {
        r.sym(sym).map(|x| x.desc.clone())
    } else if let Some(p) = s.p.procedure(key) {
        p.inputs.iter().find(|i| i.symbol == sym).map(|i| i.meaning.clone())
            .or_else(|| p.outputs.iter().find(|o| o.symbol == sym).map(|o| o.meaning.clone()))
    } else {
        None
    };
    match d {
        Some(d) if !d.is_empty() => d,
        _ => sym.to_string(),
    }
}

/// An axis label: "desc sym (unit)".
pub(crate) fn axis(s: &Solved, sym: &str) -> String {
    let u = unit(s, s.key, sym);
    let d = desc(s, s.key, sym);
    let mut d = d.split(" (").next().unwrap_or(&d).to_string();
    // a long description would be cut on a turned axis: its first words then
    while d.chars().count() + sym.chars().count() + u.chars().count() > 30 && d.contains(' ') {
        d = d.rsplit_once(' ').map(|x| x.0.to_string()).unwrap_or(d);
        for w in [" of", " the", " at", " a", " per", " based", " on"] {
            if d.ends_with(w) {
                d.truncate(d.len() - w.len());
            }
        }
    }
    if u.is_empty() { format!("{d} {sym}") } else { format!("{d} {sym} ({u})") }
}

/// The capability's name.
pub(crate) fn name(s: &Solved) -> String {
    s.p.relation(s.key).map(|r| r.name.clone()).or_else(|| s.p.procedure(s.key).map(|p| p.name.clone()))
        .unwrap_or_else(|| s.key.replace('_', " "))
}

/// A quantity with its unit (SI prefixes where the kit takes them).
pub(crate) fn qv(s: &Solved, sym: &str, x: f64) -> String {
    k::q(x, &unit(s, s.key, sym))
}

/// Whether the visitor wrote this temperature in degrees Celsius (then the drawing speaks in °C too).
pub(crate) fn celsius(s: &Solved, sym: &str) -> bool {
    s.given.iter().find(|g| g.0 == sym).map(|g| match &g.1 {
        Input::Text(t) => t.contains('C') && !t.contains('K'),
        _ => false,
    }).unwrap_or(false)
}

/// A temperature (K) as text, in °C when `c`.
pub(crate) fn temp(x: f64, c: bool) -> String {
    if c { format!("{} °C", num(k::snap(x - 273.15, x))) } else { format!("{} K", num(x)) }
}

// ------------------------------------------------------------------------------------------------ maps
/// The map of the result over two inputs, each from half to one and a half times the visitor's value.
fn field_around(s: &Solved, out: &str, x: &str, y: &str, log: (bool, bool)) -> Option<Figure> {
    let (xv, yv) = (s.get(x)?, s.get(y)?);
    if xv == 0.0 || yv == 0.0 {
        return None;
    }
    let around = |sym: &str, v: f64| -> (f64, f64) {
        if unit(s, s.key, sym) == "K" {
            // a temperature: half to one and a half times its value in degrees Celsius (20 K either side near 0 °C)
            let c = v - 273.15;
            if c > 2.0 { (273.15 + 0.5 * c, 273.15 + 1.5 * c) } else { (v - 20.0, v + 20.0) }
        } else if v > 0.0 {
            (v * 0.5, v * 1.5)
        } else {
            (v * 1.5, v * 0.5)
        }
    };
    let (xa, xb) = around(x, xv);
    let (ya, yb) = around(y, yv);
    field_fig(s, out, (x, xa, xb, log.0), (y, ya, yb, log.1))
}

fn churchill_chu(s: &Solved) -> Option<Figure> {
    let (ra, pr) = (s.get("Ra")?, s.get("Pr")?);
    if !(ra > 0.0 && pr > 0.0) {
        return None;
    }
    field_fig(s, "Nu", ("Ra", ra * 0.5, ra * 1.5, false), ("Pr", pr * 0.1, pr * 10.0, true))
}

fn effectiveness_map(s: &Solved) -> Option<Figure> {
    let (n, cr) = (s.get("N")?, s.get("Cr")?);
    if !(n > 0.0) || !(0.0..=1.0).contains(&cr) {
        return None;
    }
    field_fig(s, "eps", ("N", (n / 10.0).min(0.1), (n * 10.0).max(10.0), true), ("Cr", 0.02, 0.98, false))
}

/// A value map: `out` over a 21 x 15 grid of the inputs x and y (each cell one run of the model), shaded by value,
/// with iso-lines and the visitor's point marked.
pub(crate) fn field_fig(s: &Solved, out: &str, x: (&str, f64, f64, bool), y: (&str, f64, f64, bool)) -> Option<Figure> {
    let (xv, yv, ov) = (s.get(x.0)?, s.get(y.0)?, s.get(out)?);
    let xs = k::span(x.1, x.2, 21, x.3);
    let ys = k::span(y.1, y.2, 15, y.3);
    if s.left() < xs.len() * ys.len() {
        return None;
    }
    let mut vals: Vec<Vec<Option<f64>>> = vec![];
    let mut n = 0;
    for xi in &xs {
        let mut col = vec![];
        for yj in &ys {
            let v = own(s, &[(x.0, *xi), (y.0, *yj)]);
            n += v.is_some() as usize;
            col.push(v);
        }
        vals.push(col);
    }
    if n * 2 < xs.len() * ys.len() {
        return None;
    }
    let ou = unit(s, s.key, out);
    let header = format!("{} = {}", out, k::q(ov, &ou));
    let mut c = Canvas::new(340.0);
    c.raw(&k::field(&xs, &ys, &vals, (xv, yv), &axis(s, x.0), &axis(s, y.0), &header, (x.3, y.3)));
    let levels = isolines(&mut c, &xs, &ys, &vals, (x.3, y.3));
    let ds = desc(s, s.key, out);
    let title = format!("{}: {} over {} and {}", name(s), ds.split(" (").next().unwrap_or(&ds),
                        desc(s, s.key, x.0), desc(s, s.key, y.0));
    let lv = if levels.is_empty() { String::new() } else {
        format!(" Iso-lines at {} = {}.", out, levels.iter().map(|l| k::q(*l, &ou)).collect::<Vec<_>>().join(", "))
    };
    let caption = format!("{} ({}) over {} from {} to {} and {} from {} to {}{}: each of the {} x {} cells is one run of \
                           the model, shaded by its value.{} Marked: the given point, {} = {}, {} = {}, where {} = {}.",
                          out, ds, x.0, qv(s, x.0, x.1), qv(s, x.0, x.2), y.0, qv(s, y.0, y.1), qv(s, y.0, y.2),
                          match (x.3, y.3) {
                              (true, true) => " (logarithmic axes)",
                              (true, false) => " (logarithmic horizontal axis)",
                              (false, true) => " (logarithmic vertical axis)",
                              _ => "",
                          }, xs.len(), ys.len(), lv, x.0, qv(s, x.0, xv), y.0, qv(s, y.0, yv), out, k::q(ov, &ou));
    Some(s.chart("field", &title, &caption, c.body()))
}

/// The figkit field's plot area (figures.rs X0, X1, Y0, Y1): where a sample of a map is drawn.
fn map_px(v: f64, lo: f64, hi: f64, log: bool, a: f64, b: f64) -> f64 {
    let f = if log { (v.log10() - lo.log10()) / (hi.log10() - lo.log10()) } else { (v - lo) / (hi - lo) };
    a + f * (b - a)
}

/// Iso-lines of a sampled map (marching squares through the samples), drawn over the figkit field and labelled;
/// returns the levels drawn.
fn isolines(c: &mut Canvas, xs: &[f64], ys: &[f64], vals: &[Vec<Option<f64>>], log: (bool, bool)) -> Vec<f64> {
    let all: Vec<f64> = vals.iter().flatten().flatten().copied().collect();
    if all.len() < 4 {
        return vec![];
    }
    let (lo, hi) = all.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), x| (a.min(*x), b.max(*x)));
    if !(hi > lo) {
        return vec![];
    }
    let mut levels: Vec<f64> = nice_ticks(lo, hi, 5).into_iter().filter(|l| *l > lo && *l < hi).collect();
    while levels.len() > 5 {
        levels = levels.into_iter().skip(1).step_by(2).collect();
    }
    let (nx, ny) = (xs.len(), ys.len());
    let px = |i: usize| map_px(xs[i], xs[0], xs[nx - 1], log.0, 96.0, 504.0);
    let py = |j: usize| map_px(ys[j], ys[0], ys[ny - 1], log.1, 278.0, 44.0);
    let mut drawn = vec![];
    for lv in &levels {
        let mut d = String::new();
        let mut label: Option<(f64, f64)> = None;
        for i in 0..nx - 1 {
            for j in 0..ny - 1 {
                let (Some(a), Some(b), Some(cc), Some(e)) = (vals[i][j], vals[i + 1][j], vals[i + 1][j + 1], vals[i][j + 1])
                else { continue };
                // corners in order round the cell, each with its pixel position
                let corners = [(a, px(i), py(j)), (b, px(i + 1), py(j)), (cc, px(i + 1), py(j + 1)), (e, px(i), py(j + 1))];
                let mut pts: Vec<(f64, f64)> = vec![];
                for m in 0..4 {
                    let (v0, x0, y0) = corners[m];
                    let (v1, x1, y1) = corners[(m + 1) % 4];
                    if (v0 < *lv) != (v1 < *lv) && v1 != v0 {
                        let t = (lv - v0) / (v1 - v0);
                        pts.push((x0 + t * (x1 - x0), y0 + t * (y1 - y0)));
                    }
                }
                for pair in pts.chunks(2) {
                    if pair.len() == 2 {
                        d.push_str(&format!("M{:.1},{:.1} L{:.1},{:.1} ", pair[0].0, pair[0].1, pair[1].0, pair[1].1));
                        for p in pair {
                            if label.map(|l| p.0 > l.0).unwrap_or(true) {
                                label = Some(*p);
                            }
                        }
                    }
                }
            }
        }
        if let Some((lx, ly)) = label {
            c.raw(&format!("<path class=\"axp-k\" style=\"stroke-width:1.2;opacity:0.8\" d=\"{}\"/>", d.trim_end()));
            let t = num(*lv);
            let w = k::text_w(&t, false);
            c.name((lx - 4.0).min(504.0 - 4.0).max(96.0 + w), (ly - 4.0).clamp(58.0, 274.0), "end", &t);
            drawn.push(*lv);
        }
    }
    drawn
}

/// Round tick values over [lo, hi], about n of them.
pub(crate) fn nice_ticks(lo: f64, hi: f64, n: usize) -> Vec<f64> {
    let span = hi - lo;
    if !(span > 0.0) || !span.is_finite() {
        return vec![lo];
    }
    let raw = span / n as f64;
    let mag = 10f64.powf(raw.log10().floor());
    let norm = raw / mag;
    let step = mag * if norm < 1.5 { 1.0 } else if norm < 3.0 { 2.0 } else if norm < 7.0 { 5.0 } else { 10.0 };
    let mut v = vec![];
    let mut t = (lo / step).ceil() * step;
    while t <= hi + step * 1e-9 && v.len() < 14 {
        v.push(if t.abs() < step * 1e-9 { 0.0 } else { t });
        t += step;
    }
    v
}
