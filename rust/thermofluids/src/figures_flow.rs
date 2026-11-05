//! Fluid figures: a tank with its wall pressures, pipes with their speeds and pressures, a normal shock, friction
//! factor charts, the boundary layer along a plate. Every number is the model's (its answer or a further solve).

use crate::figures::{calc, own, qv};
use crate::figures_plot::{last, range, Plot};
use ax_engine::figkit::{self as k, num, q, Canvas, Solved};
use ax_engine::figures::Figure;

/// A tank of liquid to depth h, the pressure on its wall at h/4, h/2, 3h/4 and h (the surface too) as arrows to one
/// scale, each the model's pressure at that depth.
pub(crate) fn tank(s: &Solved, absolute: bool) -> Option<Figure> {
    let (h, p) = (s.get("h")?, s.get("P")?);
    if !(h > 0.0) {
        return None;
    }
    let mut pts: Vec<(f64, f64)> = vec![];
    if absolute {
        pts.push((0.0, s.get("P0")?));
    } else if let Some(p0) = own(s, &[("h", 0.0)]) {
        pts.push((0.0, p0));
    }
    for f in [0.25, 0.5, 0.75] {
        pts.push((h * f, own(s, &[("h", h * f)])?));
    }
    pts.push((h, p));
    let pmax = pts.iter().map(|x| x.1).fold(0.0f64, f64::max);
    if !(pmax > 0.0) || pts.iter().any(|x| x.1 < 0.0) {
        return None;
    }
    let mut c = Canvas::new(330.0);
    let (left, right, top, bottom) = (70.0, 230.0, 60.0, 290.0);
    let y = |z: f64| top + (bottom - top) * z / h;
    // the liquid, the walls and the free surface
    c.raw(&format!("<rect class=\"axp-s2\" x=\"{left}\" y=\"{top}\" width=\"{}\" height=\"{}\"/>", right - left, bottom - top));
    c.wire(&[(left, top - 30.0), (left, bottom), (right, bottom), (right, top - 30.0)]);
    c.line((left + 40.0, top), (right - 40.0, top));
    c.raw(&format!("<path class=\"axp-kd\" d=\"M{:.1},{:.1} l-7,-10 l14,0 Z\"/>", (left + right) / 2.0, top - 1.0));
    c.name((left + right) / 2.0, top - 16.0, "middle", "free surface");
    // the pressure on the right wall: arrows pointing at the wall, lengths to one scale, the envelope through tails
    let scale = 170.0 / pmax;
    let mut env = vec![];
    for (z, pz) in &pts {
        let yy = y(*z);
        let len = pz * scale;
        if len > 6.0 {
            c.arrow((right + 4.0 + len, yy), (right + 3.0, yy));
        }
        env.push((right + 4.0 + len, yy));
        c.value(right + 10.0 + len, yy + 4.0, "start", &q(*pz, "Pa"));
        c.name(left - 8.0, yy + 4.0, "end", &q(*z, "m"));
        c.dashed((left - 4.0, yy), (left + 6.0, yy));
    }
    c.wire(&env);
    c.bold(16.0, 24.0, "start", &format!("{} = {} at h = {}", "P", q(p, "Pa"), q(h, "m")));
    let rho = s.get("rho")?;
    c.value((left + right) / 2.0, (top + bottom) / 2.0, "middle", &format!("ρ = {}", qv(s, "rho", rho)));
    c.name((left + right) / 2.0, bottom + 24.0, "middle", "depth below the surface");
    let what = if absolute { "absolute" } else { "gauge" };
    let list = pts.iter().map(|(z, pz)| format!("{} at {}", q(*pz, "Pa"), q(*z, "m"))).collect::<Vec<_>>().join(", ");
    Some(s.figure("schematic", &format!("Hydrostatic {what} pressure on a tank wall"),
                  &format!("A tank of liquid (ρ = {}) to depth h = {} with the {what} pressure on its wall, each value \
                            the model's pressure at that depth, drawn as arrows to one scale: {list}.",
                           qv(s, "rho", rho), q(h, "m")), c.body(), c.h))
}

/// A pipe drawn as a contraction (or an expansion) to scale between two centre-line x positions.
fn pipe(c: &mut Canvas, cy: f64, d1: f64, d2: f64, xs: (f64, f64, f64, f64)) {
    let (a, b, cc, e) = xs;
    for sgn in [-1.0, 1.0] {
        c.wire(&[(a, cy + sgn * d1 / 2.0), (b, cy + sgn * d1 / 2.0), (cc, cy + sgn * d2 / 2.0), (e, cy + sgn * d2 / 2.0)]);
    }
    c.raw(&format!("<path class=\"axp-s2\" d=\"M{a:.1},{:.1} L{b:.1},{:.1} L{cc:.1},{:.1} L{e:.1},{:.1} L{e:.1},{:.1} \
                    L{cc:.1},{:.1} L{b:.1},{:.1} L{a:.1},{:.1} Z\"/>", cy - d1 / 2.0, cy - d1 / 2.0, cy - d2 / 2.0,
                   cy - d2 / 2.0, cy + d2 / 2.0, cy + d2 / 2.0, cy + d1 / 2.0, cy + d1 / 2.0));
}

/// Diameters in pixels for two pipe diameters (the larger 110 px, the smaller at least 6 px).
fn pipe_px(d1: f64, d2: f64, big: f64) -> (f64, f64) {
    let m = d1.max(d2);
    ((big * d1 / m).max(6.0), (big * d2 / m).max(6.0))
}

/// Flow through a contraction drawn to scale, arrows proportional to the mean speeds.
pub(crate) fn contraction(s: &Solved) -> Option<Figure> {
    let (d1, d2, v1, v2) = (s.get("D1")?, s.get("D2")?, s.get("v1")?, s.get("v2")?);
    if !(d1 > 0.0 && d2 > 0.0 && v1 > 0.0 && v2 > 0.0) {
        return None;
    }
    let mut c = Canvas::new(230.0);
    let cy = 115.0;
    let (p1, p2) = pipe_px(d1, d2, 120.0);
    pipe(&mut c, cy, p1, p2, (30.0, 210.0, 290.0, 490.0));
    let vmax = v1.max(v2);
    let (l1, l2) = (150.0 * v1 / vmax, 150.0 * v2 / vmax);
    c.arrow((120.0 - l1 / 2.0, cy), (120.0 + l1 / 2.0, cy));
    c.arrow((390.0 - l2 / 2.0, cy), (390.0 + l2 / 2.0, cy));
    c.tag(40.0, cy - p1.max(p2) / 2.0 - 26.0, "start", &format!("D1 = {}", q(d1, "m")), &format!("v1 = {}", qv(s, "v1", v1)));
    c.tag(300.0, cy - p1.max(p2) / 2.0 - 26.0, "start", &format!("D2 = {}", q(d2, "m")), "");
    c.bold(300.0, cy + p1.max(p2) / 2.0 + 30.0, "start", &format!("v2 = {}", qv(s, "v2", v2)));
    c.name(40.0, cy + p1 / 2.0 + 30.0, "start", "arrow lengths: mean speed");
    Some(s.figure("schematic", "Continuity: speed through a contraction",
                  &format!("Flow through a change of section drawn to scale: diameter {} to {}, inlet speed {}; arrow \
                            lengths are proportional to the mean speed. Marked: the model's outlet speed v2 = {}.",
                           q(d1, "m"), q(d2, "m"), qv(s, "v1", v1), qv(s, "v2", v2)), c.body(), c.h))
}

/// Bernoulli along a horizontal contraction: the pipe (outlet diameter from the model's continuity relation for a
/// stated inlet of 0.1 m) over the pressure at 41 positions, each the model's Bernoulli pressure for the speed there
/// (the model's continuity speed for the local diameter).
pub(crate) fn bernoulli(s: &Solved) -> Option<Figure> {
    let (p1, rho, v1, v2, p2) = (s.get("P1")?, s.get("rho")?, s.get("v1")?, s.get("v2")?, s.get("P2")?);
    if !(v1 > 0.0 && v2 > 0.0) {
        return None;
    }
    let d1 = 0.1;
    let d2 = calc(s, "continuity_speed", &[("v1", v1), ("D1", d1), ("v2", v2)])?;
    // positions along the pipe (m): inlet 0 to 0.3, taper 0.3 to 0.4, outlet 0.4 to 0.7
    let xs = k::span(0.0, 0.7, 41, false);
    let dia = |x: f64| if x <= 0.3 { d1 } else if x >= 0.4 { d2 } else { d1 + (d2 - d1) * (x - 0.3) / 0.1 };
    let mut pts = vec![];
    for x in &xs {
        let v = calc(s, "continuity_speed", &[("v1", v1), ("D1", d1), ("D2", dia(*x))])?;
        let p = own(s, &[("P1", p1), ("rho", rho), ("v1", v1), ("v2", v)])?;
        pts.push((*x, p / 1000.0));
    }
    let mut c = Canvas::new(434.0);
    let cy = 112.0;
    let (q1, q2) = pipe_px(d1, d2, 90.0);
    let pl = Plot::new((96.0, 190.0, 504.0, 380.0), (0.0, 0.7, false),
                       range(&pts.iter().map(|p| p.1).collect::<Vec<_>>(), false, false)
                           .map(|(a, b)| (a, b, false))?, "b")?;
    let xp = |x: f64| pl.px(x);
    pipe(&mut c, cy, q1, q2, (xp(0.0), xp(0.3), xp(0.4), xp(0.7)));
    let vmax = v1.max(v2);
    c.arrow((xp(0.15) - 40.0 * v1 / vmax, cy), (xp(0.15) + 40.0 * v1 / vmax, cy));
    c.arrow((xp(0.55) - 40.0 * v2 / vmax, cy), (xp(0.55) + 40.0 * v2 / vmax, cy));
    c.tag(xp(0.0) + 4.0, cy - q1.max(q2) / 2.0 - 24.0, "start", &format!("D1 = {} (stated)", q(d1, "m")),
          &format!("v1 = {}", qv(s, "v1", v1)));
    c.tag(xp(0.42), cy - q1.max(q2) / 2.0 - 24.0, "start", &format!("D2 = {}", q(d2, "m")), &format!("v2 = {}", qv(s, "v2", v2)));
    pl.frame(&mut c, "Position along the pipe x (m)", "Pressure P (kPa)");
    pl.path(&mut c, &pts, "axp-c");
    pl.mark(&mut c, (0.7, p2 / 1000.0));
    pl.label(&mut c, (0.0, p1 / 1000.0), 6.0, 18.0, "axp-vt", &format!("P1 = {}", q(p1, "Pa")));
    pl.label(&mut c, (0.7, p2 / 1000.0), -10.0, if p2 < p1 { -10.0 } else { 18.0 }, "axp-vt", &format!("P2 = {}", q(p2, "Pa")));
    c.bold(504.0, 22.0, "end", &format!("ρ = {}", qv(s, "rho", rho)));
    Some(s.figure("schematic", "Bernoulli: pressure falls as the flow speeds up",
                  &format!("Bernoulli's equation along a horizontal change of section (ρ = {}, P1 = {}, v1 = {}, v2 = {}). \
                            The pipe is drawn for a stated inlet diameter of {}, the outlet {} from the model's continuity \
                            relation; the pressure at 41 positions is the model's Bernoulli pressure for the local speed. \
                            Marked: P2 = {}.", qv(s, "rho", rho), q(p1, "Pa"), qv(s, "v1", v1), qv(s, "v2", v2),
                           q(d1, "m"), q(d2, "m"), q(p2, "Pa")), c.body(), c.h))
}

/// Laminar pipe flow: the pipe, its parabolic velocity profile (from the model's flow rate: mean speed Q/A, the
/// centre-line speed twice it) and the stated pressure drop.
pub(crate) fn poiseuille(s: &Solved) -> Option<Figure> {
    let (d, dp, mu, l, qf) = (s.get("D")?, s.get("dP")?, s.get("mu")?, s.get("L")?, s.get("Q")?);
    if !(d > 0.0 && qf > 0.0) {
        return None;
    }
    let area = std::f64::consts::PI * d * d / 4.0;
    let um = qf / area;
    let mut c = Canvas::new(250.0);
    let (x0, x1, top, bot) = (30.0, 330.0, 70.0, 190.0);
    c.raw(&format!("<rect class=\"axp-s2\" x=\"{x0}\" y=\"{top}\" width=\"{}\" height=\"{}\"/>", x1 - x0, bot - top));
    c.line((x0, top), (x1, top));
    c.line((x0, bot), (x1, bot));
    c.dashed((x0, (top + bot) / 2.0), (x1, (top + bot) / 2.0));
    // the profile: u(r) = 2 um (1 - r^2/R^2), arrows at 9 radii from a line at x = 90
    let base = 90.0;
    let scale = 170.0 / (2.0 * um);
    let mut env = vec![];
    for i in 0..=12 {
        let f = -1.0 + 2.0 * i as f64 / 12.0;
        let u = 2.0 * um * (1.0 - f * f);
        let y = (top + bot) / 2.0 + f * (bot - top) / 2.0;
        if u * scale > 8.0 {
            c.arrow((base, y), (base + u * scale, y));
        }
        env.push((base + u * scale, y));
    }
    c.line((base, top), (base, bot));
    c.wire(&env);
    c.name(x0, top - 12.0, "start", &format!("P1, Δp = {} over L = {}", q(dp, "Pa"), q(l, "m")));
    c.name(x0, bot + 22.0, "start", &format!("D = {}, μ = {}", q(d, "m"), qv(s, "mu", mu)));
    c.bold(345.0, 90.0, "start", &format!("Q = {} m³/s", num(qf)));
    c.value(345.0, 116.0, "start", &format!("mean u = {} m/s", num(um)));
    c.value(345.0, 138.0, "start", &format!("centre u = {} m/s", num(2.0 * um)));
    c.name(345.0, 166.0, "start", "parabolic profile");
    Some(s.figure("schematic", "Hagen-Poiseuille flow and its velocity profile",
                  &format!("Laminar flow in a pipe (D = {}, pressure drop {} over {}, μ = {}). Marked: the model's volume \
                            flow Q = {} m³/s; the mean speed Q/A = {} m/s and the parabolic profile u = 2 (Q/A)(1 - r²/R²) \
                            with its centre-line maximum {} m/s are derived from it.", q(d, "m"), q(dp, "Pa"), q(l, "m"),
                           qv(s, "mu", mu), num(qf), num(um), num(2.0 * um)), c.body(), c.h))
}

/// A normal shock in a duct: Mach number, static and stagnation pressure before and after, all from the model's
/// shock and stagnation relations (an upstream pressure of 100 kPa stated when the capability takes none).
pub(crate) fn shock(s: &Solved) -> Option<Figure> {
    let m1 = s.get("M1")?;
    if !(m1 > 1.0) {
        return None;
    }
    let (p1, stated) = match s.get("P1") {
        Some(p) => (p, false),
        None => (100_000.0, true),
    };
    let m2 = match s.get("M2") {
        Some(x) => x,
        None => calc(s, "normal_shock_mach_air", &[("M1", m1)])?,
    };
    let p2 = match s.get("P2") {
        Some(x) => x,
        None => calc(s, "normal_shock_pressure_air", &[("P1", p1), ("M1", m1)])?,
    };
    let p01 = calc(s, "stagnation_pressure_air", &[("P", p1), ("M", m1)])?;
    let p02 = calc(s, "stagnation_pressure_air", &[("P", p2), ("M", m2)])?;
    let mut c = Canvas::new(230.0);
    let (top, bot) = (60.0, 150.0);
    c.line((20.0, top), (500.0, top));
    c.line((20.0, bot), (500.0, bot));
    c.raw(&format!("<path class=\"axp-c\" d=\"M260,{top} l-6,15 l12,15 l-12,15 l12,15 l-12,15 l6,15\"/>"));
    let mmax = m1.max(m2);
    let cy = (top + bot) / 2.0;
    c.arrow((140.0 - 50.0 * m1 / mmax, cy), (140.0 + 50.0 * m1 / mmax, cy));
    c.arrow((380.0 - 50.0 * m2 / mmax, cy), (380.0 + 50.0 * m2 / mmax, cy));
    c.name(260.0, top - 10.0, "middle", "normal shock");
    c.bold(140.0, top - 10.0, "middle", &format!("M1 = {}", num(m1)));
    c.bold(380.0, top - 10.0, "middle", &format!("M2 = {}", num(m2)));
    c.value(30.0, bot + 24.0, "start", &format!("P1 = {}{}", q(p1, "Pa"), if stated { " (stated)" } else { "" }));
    c.value(30.0, bot + 44.0, "start", &format!("P01 = {}", q(p01, "Pa")));
    c.value(290.0, bot + 24.0, "start", &format!("P2 = {}", q(p2, "Pa")));
    c.value(290.0, bot + 44.0, "start", &format!("P02 = {}", q(p02, "Pa")));
    c.name(290.0, bot + 66.0, "start", &format!("stagnation loss {}", q(p01 - p02, "Pa")));
    c.name(30.0, bot + 66.0, "start", "air, γ = 1.4");
    Some(s.figure("shock", "A normal shock: states either side",
                  &format!("A normal shock in air (γ = 1.4) at M1 = {} with an upstream static pressure of {}{}: the model \
                            gives M2 = {} and P2 = {}; the stagnation pressures from its stagnation-pressure relation, {} \
                            before and {} after (the loss across the shock).", num(m1), q(p1, "Pa"),
                           if stated { " (stated)" } else { "" }, num(m2), q(p2, "Pa"), q(p01, "Pa"), q(p02, "Pa")),
                  c.body(), c.h))
}

/// A friction-factor chart on log-log axes: curves (name, points, highlighted), labelled at their right ends, the
/// visitor's point marked.
fn moody(curves: &[(String, Vec<(f64, f64)>, bool)], marker: (f64, f64), mlabel: &str, header: &str, h: f64)
         -> Option<(String, f64)> {
    let xs: Vec<f64> = curves.iter().flat_map(|c| c.1.iter().map(|p| p.0)).chain([marker.0]).collect();
    let ys: Vec<f64> = curves.iter().flat_map(|c| c.1.iter().map(|p| p.1)).chain([marker.1]).collect();
    let (xa, xb) = range(&xs, true, false)?;
    let (ya, yb) = range(&ys, true, false)?;
    let mut c = Canvas::new(h);
    let pl = Plot::new((80.0, 44.0, 430.0, h - 62.0), (xa, xb, true), (ya, yb, true), "m")?;
    pl.frame(&mut c, "Reynolds number Re", "Darcy friction factor f");
    let mut used: Vec<f64> = vec![];
    for (n, pts, hl) in curves {
        pl.path(&mut c, pts, if *hl { "axp-c" } else { "axp-c4" });
        if let Some(e) = last(pts) {
            let mut y = pl.py(e.1) + 4.0;
            while used.iter().any(|u| (u - y).abs() < 13.0) {
                y += 13.0;
            }
            used.push(y);
            c.text(pl.px(e.0) + 5.0, y, "start", if *hl { "axp-vt" } else { "axp-lt" }, n);
        }
    }
    pl.mark(&mut c, marker);
    pl.label(&mut c, marker, 10.0, -10.0, "axp-mt", mlabel);
    c.text(80.0, 20.0, "start", "axp-mt", header);
    Some((c.s, h))
}

/// Colebrook: the turbulent friction factor against Re for six relative roughnesses (one is the visitor's), each
/// point a run of the model; the second chart adds the laminar line on log-log axes (a Moody chart).
pub(crate) fn colebrook(s: &Solved) -> Vec<Option<Figure>> {
    let (Some(re), Some(ed), Some(f)) = (s.get("Re"), s.get("eD"), s.get("f")) else { return vec![] };
    let mut eds = vec![0.0, 1e-4, 1e-3, 1e-2, 0.05];
    if !eds.iter().any(|x| (x - ed).abs() <= 1e-12 + 1e-6 * ed) {
        eds.push(ed);
    }
    eds.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let res = k::span(4000.0, 1e8, 30, true);
    let mut fam: Vec<(f64, Vec<(f64, f64)>)> = vec![];
    for e in &eds {
        let pts: Vec<(f64, f64)> = res.iter()
            .filter_map(|r| calc(s, "colebrook_friction_factor", &[("Re", *r), ("eD", *e)]).map(|y| (*r, y))).collect();
        if pts.len() >= 2 {
            fam.push((*e, pts));
        }
    }
    if fam.is_empty() {
        return vec![];
    }
    let lam: Vec<(f64, f64)> = k::span(600.0, 2300.0, 8, true).iter()
        .filter_map(|r| calc(s, "laminar_friction_factor", &[("Re", *r)]).map(|y| (*r, y))).collect();
    let mine = |e: f64| (e - ed).abs() <= 1e-12 + 1e-6 * ed;
    let label = |e: f64| if e == 0.0 { "smooth".to_string() } else { format!("ε/D = {}", num(e)) };
    // chart 1: the family, linear friction factor against log Re
    // at most four curves (the visitor's roughness, smooth, 0.001 and 0.05) so that every one has its colour
    let series: Vec<(String, Vec<(f64, f64)>)> = fam.iter()
        .filter(|(e, _)| mine(*e) || *e == 0.0 || *e == 1e-3 || *e == 0.05).take(4)
        .map(|(e, p)| (label(*e), p.clone())).collect();
    let body = k::lines(&series, "Reynolds number Re", "Darcy friction factor f", &format!("f = {}", num(f)),
                        Some((re, f)), &format!("Re = {}", num(re)), true);
    let n = series.len();
    let one = s.chart("chart", "Friction factor chart (Colebrook, turbulent flow)",
                      &format!("Darcy friction factor of turbulent pipe flow against Reynolds number (logarithmic axis) \
                                for {n} relative roughnesses from smooth to ε/D = 0.05, each from the Colebrook equation \
                                solved by the model at up to 30 Reynolds numbers between 4000 and 1e+08. Marked: Re = {}, \
                                ε/D = {}, f = {}.", num(re), num(ed), num(f)), &body);
    // chart 2: Moody, log-log with the laminar line
    let mut curves: Vec<(String, Vec<(f64, f64)>, bool)> = fam.iter().map(|(e, p)| (label(*e), p.clone(), mine(*e))).collect();
    if lam.len() >= 2 {
        curves.push(("".into(), lam.clone(), false));
    }
    let two = moody(&curves, (re, f), &format!("f = {}", num(f)), "Moody chart (Colebrook)", 360.0).map(|(b, h)| {
        s.figure("chart-2", "Moody chart with the operating point (Colebrook)",
                 &format!("Friction-factor (Moody) chart: the model's Colebrook friction factor at 30 Reynolds numbers from \
                           4000 to 1e+08 for relative roughnesses {} (labelled at the right), with its laminar 64/Re line \
                           below Re = 2300; log-log axes. Marked: Re = {}, ε/D = {}, f = {}.",
                          fam.iter().map(|x| num(x.0)).collect::<Vec<_>>().join(", "), num(re), num(ed), num(f)), &b, h)
    });
    vec![Some(one), two]
}

/// The laminar line, the Blasius smooth-pipe formula and the Colebrook factor of a smooth pipe, each from the model,
/// with the visitor's point on its own formula.
pub(crate) fn smooth_chart(s: &Solved) -> Option<Figure> {
    let (re, f) = (s.get("Re")?, s.get("f")?);
    let lam: Vec<(f64, f64)> = k::span(400.0, 2300.0, 12, true).iter()
        .filter_map(|r| calc(s, "laminar_friction_factor", &[("Re", *r)]).map(|y| (*r, y))).collect();
    let bla: Vec<(f64, f64)> = k::span(4000.0, 1e5, 14, true).iter()
        .filter_map(|r| calc(s, "blasius_friction_factor", &[("Re", *r)]).map(|y| (*r, y))).collect();
    let col: Vec<(f64, f64)> = k::span(4000.0, 1e6, 14, true).iter()
        .filter_map(|r| calc(s, "colebrook_friction_factor", &[("Re", *r), ("eD", 0.0)]).map(|y| (*r, y))).collect();
    let laminar = s.key == "laminar_friction_factor";
    let curves = vec![("64/Re".to_string(), lam, laminar), ("Blasius".to_string(), bla, !laminar),
                      ("Colebrook, smooth".to_string(), col, false)];
    if curves.iter().filter(|c| c.1.len() >= 2).count() < 2 {
        return None;
    }
    let (b, h) = moody(&curves, (re, f), &format!("f = {}", num(f)), &format!("f = {} at Re = {}", num(f), num(re)), 330.0)?;
    let which = if laminar { "laminar 64/Re line" } else { "Blasius line" };
    Some(s.figure("chart", &format!("Friction factor chart: the {} point", if laminar { "laminar 64/Re" } else { "Blasius" }),
                  &format!("Friction factor against Reynolds number (log-log): the model's laminar line 64/Re to Re = 2300, \
                            its Blasius smooth-pipe formula from 4000 to 1e+05 and, for comparison, its Colebrook factor of \
                            a smooth pipe to 1e+06. Marked: the given point on the {which}, Re = {}, f = {}.", num(re),
                           num(f)), &b, h))
}

/// Swamee-Jain or Haaland: the model's explicit friction factor for several relative roughnesses at the visitor's
/// diameter (one curve the visitor's own roughness), with the laminar line.
pub(crate) fn explicit_chart(s: &Solved) -> Option<Figure> {
    let (eps, d, re, f) = (s.get("eps")?, s.get("D")?, s.get("Re")?, s.get("f")?);
    if !(d > 0.0 && eps >= 0.0) {
        return None;
    }
    let ed = eps / d;
    let mut eds = vec![1e-4, 1e-3, 1e-2, 0.05];
    if !eds.iter().any(|x| (x - ed).abs() <= 1e-6 * ed.max(1e-12)) {
        eds.push(ed);
    }
    eds.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let res = k::span(5000.0, 1e8, 28, true);
    let mut curves: Vec<(String, Vec<(f64, f64)>, bool)> = vec![];
    for e in &eds {
        let pts: Vec<(f64, f64)> = res.iter().filter_map(|r| own(s, &[("eps", e * d), ("D", d), ("Re", *r)]).map(|y| (*r, y)))
            .collect();
        if pts.len() >= 2 {
            curves.push((format!("ε/D = {}", num(*e)), pts, (e - ed).abs() <= 1e-6 * ed.max(1e-12)));
        }
    }
    if curves.is_empty() {
        return None;
    }
    let lam: Vec<(f64, f64)> = k::span(600.0, 2300.0, 8, true).iter()
        .filter_map(|r| calc(s, "laminar_friction_factor", &[("Re", *r)]).map(|y| (*r, y))).collect();
    if lam.len() >= 2 {
        curves.push(("".into(), lam, false));
    }
    let who = if s.key == "haaland_friction_factor" { "Haaland" } else { "Swamee-Jain" };
    let (b, h) = moody(&curves, (re, f), &format!("f = {}", num(f)), &format!("{who}: f = {}", num(f)), 360.0)?;
    Some(s.figure("chart", &format!("Moody chart with the operating point ({who})"),
                  &format!("Friction-factor chart from the model's {who} formula at 28 Reynolds numbers from 5000 to 1e+08, \
                            pipe diameter D = {}, relative roughness ε/D = {} (labelled at the right), with its laminar \
                            64/Re line below Re = 2300; log-log axes. Marked: ε/D = {}, Re = {}, f = {}.", q(d, "m"),
                           eds.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "), num(ed), num(re), num(f)), &b, h))
}

/// The displacement and momentum thickness along a flat plate (Re_x growing in proportion to x), each from the
/// model's Blasius relations at 40 positions; the visitor's point marked.
pub(crate) fn boundary_layer(s: &Solved) -> Option<Figure> {
    let (x, rex) = (s.get("x")?, s.get("Rex")?);
    if !(x > 0.0 && rex > 0.0) {
        return None;
    }
    let disp = s.key == "blasius_displacement_thickness";
    let val = s.get(if disp { "ds" } else { "th" })?;
    let xs = k::span(x / 40.0, 2.0 * x, 40, false);
    let mut ds = vec![];
    let mut th = vec![];
    for xi in &xs {
        let r = rex * xi / x;
        if let Some(v) = calc(s, "blasius_displacement_thickness", &[("x", *xi), ("Rex", r)]) {
            ds.push((*xi, v * 1000.0));
        }
        if let Some(v) = calc(s, "blasius_momentum_thickness", &[("x", *xi), ("Rex", r)]) {
            th.push((*xi, v * 1000.0));
        }
    }
    if ds.len() < 2 || th.len() < 2 {
        return None;
    }
    let series = vec![("δ* (displacement)".to_string(), ds), ("θ (momentum)".to_string(), th)];
    let sym = if disp { "δ*" } else { "θ" };
    let body = k::lines_y(&series, "Distance from the leading edge x (m)", "Thickness (mm)",
                          &format!("{sym} = {} mm at x = {}", num(val * 1000.0), q(x, "m")), Some((x, val * 1000.0)),
                          &format!("Re_x = {}", num(rex)), false, true);
    Some(s.chart("profile", "Boundary-layer thickness along a flat plate",
                 &format!("Laminar boundary layer on a flat plate: displacement and momentum thickness from the model's \
                           Blasius relations at 40 positions, the local Reynolds number growing in proportion to x (Re_x = \
                           {} at x = {}). Marked: the given point, {sym} = {} mm at x = {}.", num(rex), q(x, "m"),
                          num(val * 1000.0), q(x, "m")), &body))
}
