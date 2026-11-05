//! Heat-transfer figures: temperature through walls, cylinders and spheres, a wall generating heat, a resistance
//! chain, radiating plates, fins, exchanger profiles and effectiveness, families of curves. Every number is the
//! model's (its answer or a further solve); a shape that is derived from the answer is said to be.

use crate::figures::{axis, calc, celsius, desc, name, own, qv, run, temp, unit};
use crate::figures_plot::{range, Plot};
use ax_engine::figkit::{self as k, num, q, Canvas, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;

/// A temperature on a plot axis: K, or °C when the visitor wrote °C.
fn tv(x: f64, c: bool) -> f64 {
    if c { x - 273.15 } else { x }
}

fn tlab(c: bool) -> &'static str {
    if c { "Temperature T (°C)" } else { "Temperature T (K)" }
}

/// Steady conduction through a plane wall: the temperature at 13 depths, each the model's face temperature of a wall
/// that thick carrying the same heat rate.
pub(crate) fn plane_wall(s: &Solved) -> Option<Figure> {
    let (kk, a, t1, t2, l, qq) = (s.get("k")?, s.get("A")?, s.get("T1")?, s.get("T2")?, s.get("L")?, s.get("q")?);
    if !(l > 0.0) {
        return None;
    }
    let c_ = celsius(s, "T1");
    let mut pts = vec![(0.0, tv(t1, c_))];
    for x in k::span(l / 12.0, l, 12, false) {
        pts.push((x, tv(calc(s, s.key, &[("k", kk), ("A", a), ("T1", t1), ("L", x), ("q", qq)])?, c_)));
    }
    let ys: Vec<f64> = pts.iter().map(|p| p.1).collect();
    let (ya, yb) = range(&ys, false, false)?;
    let mut c = Canvas::new(320.0);
    let pl = Plot::new((96.0, 50.0, 470.0, 258.0), (-0.25 * l, 1.25 * l, false), (ya, yb, false), "w")?;
    c.raw(&format!("<rect class=\"axp-ka\" x=\"{:.1}\" y=\"50\" width=\"{:.1}\" height=\"208\"/>", pl.px(0.0),
                   pl.px(l) - pl.px(0.0)));
    pl.frame(&mut c, "Position through the wall x (m)", tlab(c_));
    pl.path(&mut c, &pts, "axp-c");
    pl.dot(&mut c, pts[0]);
    pl.mark(&mut c, (l, tv(t2, c_)));
    pl.label(&mut c, pts[0], 8.0, 18.0, "axp-vt", &format!("T1 = {}", temp(t1, c_)));
    pl.label(&mut c, (l, tv(t2, c_)), 8.0, -6.0, "axp-vt", &format!("T2 = {}", temp(t2, c_)));
    c.bold(96.0, 24.0, "start", &format!("q = {} (flux {} W/m²)", q(qq, "W"), num(qq / a)));
    c.name(pl.px(l / 2.0), 68.0, "middle", "wall");
    Some(s.figure("profile", "Conduction through a plane wall",
                  &format!("Steady conduction through a plane wall of thickness L = {}, area {} m², k = {}, faces at {} and \
                            {}: the temperature at 12 depths is the model's face temperature of a wall that thick carrying \
                            the same heat rate, a straight line. Marked: the model's heat rate q = {} (flux {} W/m², q \
                            divided by the area).", q(l, "m"), num(a), qv(s, "k", kk), temp(t1, c_), temp(t2, c_),
                           q(qq, "W"), num(qq / a)), c.body(), c.h))
}

/// Radial conduction through a cylinder or sphere wall: the cross-section with the heat flow, and the temperature at
/// 16 radii, each the model's outer temperature of a wall ending at that radius with the same heat rate.
pub(crate) fn radial(s: &Solved, sphere: bool) -> Option<Figure> {
    let (kk, t1, t2, r1, r2, qq) = (s.get("k")?, s.get("T1")?, s.get("T2")?, s.get("r1")?, s.get("r2")?, s.get("q")?);
    if !(r1 > 0.0 && r2 > r1) {
        return None;
    }
    let len = if sphere { None } else { Some(s.get("L")?) };
    let c_ = celsius(s, "T1");
    let mut pts = vec![(r1, tv(t1, c_))];
    for r in k::span(r1 + (r2 - r1) / 15.0, r2, 15, false) {
        let mut set = vec![("k", kk), ("T1", t1), ("r1", r1), ("r2", r), ("q", qq)];
        if let Some(l) = len {
            set.push(("L", l));
        }
        pts.push((r, tv(calc(s, s.key, &set)?, c_)));
    }
    let mut c = Canvas::new(474.0);
    // the cross-section
    let (cx, cy, big) = (130.0, 112.0, 88.0);
    let a1 = big * r1 / r2;
    c.raw(&format!("<path class=\"axp-ka\" fill-rule=\"evenodd\" d=\"M{:.1},{cy} a{big},{big} 0 1,0 {:.1},0 a{big},{big} 0 1,0 \
                    {:.1},0 Z M{:.1},{cy} a{a1:.1},{a1:.1} 0 1,0 {:.1},0 a{a1:.1},{a1:.1} 0 1,0 {:.1},0 Z\"/>",
                   cx - big, 2.0 * big, -2.0 * big, cx - a1, 2.0 * a1, -2.0 * a1));
    c.raw(&format!("<circle class=\"axp-k\" cx=\"{cx}\" cy=\"{cy}\" r=\"{big}\"/><circle class=\"axp-k\" cx=\"{cx}\" cy=\"{cy}\" r=\"{a1:.1}\"/>"));
    let out = qq > 0.0;
    for i in 0..8 {
        let th = std::f64::consts::PI * i as f64 / 4.0 + 0.39;
        let (u0, u1) = (th.cos(), -th.sin());
        let (ra, rb) = if out { (a1 + 4.0, big + 18.0) } else { (big + 18.0, a1 + 4.0) };
        c.arrow((cx + u0 * ra, cy + u1 * ra), (cx + u0 * rb, cy + u1 * rb));
    }
    c.bold(260.0, 40.0, "start", &format!("q = {}", q(qq, "W")));
    c.value(260.0, 64.0, "start", &format!("r1 = {}, T1 = {}", q(r1, "m"), temp(t1, c_)));
    c.value(260.0, 86.0, "start", &format!("r2 = {}, T2 = {}", q(r2, "m"), temp(t2, c_)));
    c.name(260.0, 110.0, "start", &format!("k = {}", qv(s, "k", kk)));
    if let Some(l) = len {
        c.name(260.0, 130.0, "start", &format!("length {}", q(l, "m")));
    }
    c.name(260.0, 152.0, "start", if out { "heat flows outward" } else { "heat flows inward" });
    let ys: Vec<f64> = pts.iter().map(|p| p.1).collect();
    let (ya, yb) = range(&ys, false, false)?;
    let pl = Plot::new((96.0, 236.0, 470.0, 412.0), (r1, r2, false), (ya, yb, false), "r")?;
    pl.frame(&mut c, "Radius r (m)", tlab(c_));
    pl.path(&mut c, &pts, "axp-c");
    pl.dot(&mut c, pts[0]);
    pl.mark(&mut c, (r2, tv(t2, c_)));
    let what = if sphere { "sphere" } else { "cylinder" };
    Some(s.figure("profile", &format!("Radial conduction through a {what} wall"),
                  &format!("Steady radial conduction through a {what} wall: r1 = {} at {}, r2 = {} at {}, k = {}{}. Top: the \
                            cross-section with the radial heat flow; bottom: the temperature at 16 radii, each the model's \
                            outer temperature of a wall ending at that radius with the same heat rate. Marked: the model's \
                            heat rate q = {}.", q(r1, "m"), temp(t1, c_), q(r2, "m"), temp(t2, c_), qv(s, "k", kk),
                           len.map(|l| format!(", length {}", q(l, "m"))).unwrap_or_default(), q(qq, "W")), c.body(), c.h))
}

/// A plane wall generating heat: the parabolic temperature, each point the model's mid-plane maximum for a wall of
/// half-thickness sqrt(L^2 - x^2) (the same parabola), the faces at Ts.
pub(crate) fn generation(s: &Solved) -> Option<Figure> {
    let (ts, qg, l, kk, tm) = (s.get("Ts")?, s.get("q")?, s.get("L")?, s.get("k")?, s.get("Tmax")?);
    if !(l > 0.0) {
        return None;
    }
    let c_ = celsius(s, "Ts");
    let mut pts = vec![(-l, tv(ts, c_))];
    for x in k::span(-l, l, 23, false).into_iter().skip(1).take(21) {
        let he = (l * l - x * x).max(0.0).sqrt();
        pts.push((x, tv(calc(s, s.key, &[("Ts", ts), ("q", qg), ("L", he), ("k", kk)])?, c_)));
    }
    pts.push((l, tv(ts, c_)));
    let ys: Vec<f64> = pts.iter().map(|p| p.1).collect();
    let (ya, yb) = range(&ys, false, false)?;
    let mut c = Canvas::new(320.0);
    let pl = Plot::new((96.0, 50.0, 470.0, 258.0), (-1.3 * l, 1.3 * l, false), (ya, yb, false), "g")?;
    c.raw(&format!("<rect class=\"axp-ka\" x=\"{:.1}\" y=\"50\" width=\"{:.1}\" height=\"208\"/>", pl.px(-l), pl.px(l) - pl.px(-l)));
    pl.frame(&mut c, "Position from the mid-plane x (m)", tlab(c_));
    pl.path(&mut c, &pts, "axp-c");
    pl.mark(&mut c, (0.0, tv(tm, c_)));
    pl.label(&mut c, (0.0, tv(tm, c_)), 10.0, -8.0, "axp-mt", &format!("Tmax = {}", temp(tm, c_)));
    pl.label(&mut c, (l, tv(ts, c_)), 6.0, -6.0, "axp-vt", &format!("Ts = {}", temp(ts, c_)));
    c.bold(96.0, 24.0, "start", &format!("q = {} W/m³, k = {}", num(qg), qv(s, "k", kk)));
    Some(s.figure("profile", "Heat generation in a wall: the parabolic temperature",
                  &format!("A plane wall of half-thickness L = {} (k = {}) generating heat uniformly at {} W/m³, both faces \
                            held at {}. Each point is the model's mid-plane maximum for a wall of half-thickness √(L² - x²), \
                            which is the temperature at x (the parabola). Marked: Tmax = {}.", q(l, "m"), qv(s, "k", kk),
                           num(qg), temp(ts, c_), temp(tm, c_)), c.body(), c.h))
}

/// The overall coefficient of a wall between two fluids as a chain of resistances per square metre: 1/h1, L/k, 1/h2
/// (the inputs inverted) add to 1/U, U the model's.
pub(crate) fn network(s: &Solved) -> Option<Figure> {
    let (h1, l, kk, h2, u) = (s.get("h1")?, s.get("L")?, s.get("k")?, s.get("h2")?, s.get("U")?);
    if !(h1 > 0.0 && h2 > 0.0 && kk > 0.0 && l > 0.0 && u > 0.0) {
        return None;
    }
    let rs = [1.0 / h1, l / kk, 1.0 / h2];
    let total: f64 = rs.iter().sum();
    if (total * u - 1.0).abs() > 1e-6 {
        return None;
    }
    let mut c = Canvas::new(280.0);
    // the wall between the fluids
    c.raw("<rect class=\"axp-s2\" x=\"40\" y=\"40\" width=\"140\" height=\"100\"/>");
    c.raw("<rect class=\"axp-s3\" x=\"340\" y=\"40\" width=\"140\" height=\"100\"/>");
    c.raw("<rect class=\"axp-ka\" x=\"180\" y=\"40\" width=\"160\" height=\"100\"/>");
    c.raw("<rect class=\"axp-k\" x=\"180\" y=\"40\" width=\"160\" height=\"100\"/>");
    c.tag(110.0, 85.0, "middle", "fluid 1", &format!("h1 = {}", qv(s, "h1", h1)));
    c.tag(260.0, 85.0, "middle", "wall", &format!("L = {}", q(l, "m")));
    c.value(260.0, 120.0, "middle", &format!("k = {}", qv(s, "k", kk)));
    c.tag(410.0, 85.0, "middle", "fluid 2", &format!("h2 = {}", qv(s, "h2", h2)));
    c.arrow((60.0, 150.0), (460.0, 150.0));
    c.name(260.0, 166.0, "middle", "heat flow");
    // the chain
    let y = 210.0;
    c.terminal((40.0, y), "");
    let xs = [40.0, 170.0, 300.0, 430.0];
    let names = ["1/h1", "L/k", "1/h2"];
    for i in 0..3 {
        c.resistor((xs[i], y), (xs[i + 1], y));
        c.name((xs[i] + xs[i + 1]) / 2.0, y - 16.0, "middle", names[i]);
        c.value((xs[i] + xs[i + 1]) / 2.0, y + 26.0, "middle", &num(rs[i]));
    }
    c.terminal((430.0, y), "");
    c.line((430.0, y), (480.0, y));
    c.bold(16.0, 262.0, "start", &format!("sum = {} m²·K/W, U = 1/sum = {}", num(total), qv(s, "U", u)));
    Some(s.figure("network", "Overall heat-transfer coefficient: a resistance chain",
                  &format!("A plane wall between two fluids (h1 = {}, wall L = {}, k = {}, h2 = {}) and its thermal-resistance \
                            chain per square metre: 1/h1 = {}, L/k = {} and 1/h2 = {} m²·K/W are the inputs inverted \
                            (derived); they add in series to {} m²·K/W. Marked: the model's U = {}.", qv(s, "h1", h1),
                           q(l, "m"), qv(s, "k", kk), qv(s, "h2", h2), num(rs[0]), num(rs[1]), num(rs[2]), num(total),
                           qv(s, "U", u)), c.body(), c.h))
}

/// Two large grey plates exchanging radiation: the model's net flux and each surface's blackbody emissive power.
pub(crate) fn plates(s: &Solved) -> Option<Figure> {
    let (t1, t2, e1, e2, qf) = (s.get("T1")?, s.get("T2")?, s.get("e1")?, s.get("e2")?, s.get("qf")?);
    let eb1 = calc(s, "blackbody_emissive_power", &[("T", t1)])?;
    let eb2 = calc(s, "blackbody_emissive_power", &[("T", t2)])?;
    let mut c = Canvas::new(250.0);
    c.raw("<rect class=\"axp-c\" x=\"150\" y=\"50\" width=\"20\" height=\"130\"/>");
    c.raw("<rect class=\"axp-c2\" x=\"350\" y=\"50\" width=\"20\" height=\"130\"/>");
    let fwd = qf >= 0.0;
    for y in [75.0, 115.0, 155.0] {
        if fwd {
            c.arrow((178.0, y), (342.0, y));
        } else {
            c.arrow((342.0, y), (178.0, y));
        }
    }
    let c_ = celsius(s, "T1");
    c.tag(140.0, 90.0, "end", "surface 1", &temp(t1, c_));
    c.value(140.0, 124.0, "end", &format!("ε1 = {}", num(e1)));
    c.tag(380.0, 90.0, "start", "surface 2", &temp(t2, c_));
    c.value(380.0, 124.0, "start", &format!("ε2 = {}", num(e2)));
    c.value(160.0, 204.0, "middle", &format!("Eb1 = {} W/m²", num(eb1)));
    c.value(360.0, 204.0, "middle", &format!("Eb2 = {} W/m²", num(eb2)));
    c.bold(260.0, 32.0, "middle", &format!("net flux q = {} W/m²", num(qf.abs())));
    c.name(260.0, 236.0, "middle", "large parallel grey plates");
    Some(s.figure("schematic", "Radiation exchange between parallel plates",
                  &format!("Radiation between two large parallel grey plates at {} (emissivity {}) and {} (emissivity {}). \
                            Marked: the model's net flux q = {} W/m², and the blackbody emissive powers of the two surfaces \
                            from its blackbody capability, {} and {} W/m².", temp(t1, c_), num(e1), temp(t2, c_), num(e2),
                           num(qf), num(eb1), num(eb2)), c.body(), c.h))
}

/// A very long fin: the rod and its temperature, T = Tinf + (Tb - Tinf) e^(-m x), with m = h P (Tb - Tinf) / q
/// derived from the model's heat rate q.
pub(crate) fn long_fin(s: &Solved) -> Option<Figure> {
    let (h, p, kk, ac, tb, ti, qq) = (s.get("h")?, s.get("P")?, s.get("k")?, s.get("Ac")?, s.get("Tb")?, s.get("Tinf")?,
                                      s.get("q")?);
    let dt = tb - ti;
    if dt == 0.0 || qq == 0.0 {
        return None;
    }
    let m = h * p * dt / qq;
    if !(m > 0.0 && m.is_finite()) {
        return None;
    }
    let c_ = celsius(s, "Tb");
    let xs = k::span(0.0, 5.0 / m, 41, false);
    let pts: Vec<(f64, f64)> = xs.iter().map(|x| (*x, tv(ti + dt * (-m * x).exp(), c_))).collect();
    let mut c = Canvas::new(310.0);
    let pl = Plot::new((96.0, 96.0, 470.0, 250.0), (0.0, 5.0 / m, false),
                       range(&[tv(tb, c_), tv(ti, c_)], false, false).map(|(a, b)| (a, b, false))?, "f")?;
    // the rod over the plot's length, the base wall hatched
    let (x0, x1) = (pl.px(0.0), pl.px(5.0 / m));
    c.raw(&format!("<rect class=\"axp-ka\" x=\"{x0:.1}\" y=\"52\" width=\"{:.1}\" height=\"14\"/>\
                    <rect class=\"axp-k\" x=\"{x0:.1}\" y=\"52\" width=\"{:.1}\" height=\"14\"/>", x1 - x0, x1 - x0));
    c.line((x0, 36.0), (x0, 82.0));
    for i in 0..5 {
        c.line((x0, 40.0 + 10.0 * i as f64), (x0 - 8.0, 48.0 + 10.0 * i as f64));
    }
    c.name(x1, 46.0, "end", "very long rod, fluid all round");
    pl.frame(&mut c, "Distance from the base x (m)", tlab(c_));
    pl.dashed(&mut c, &[(0.0, tv(ti, c_)), (5.0 / m, tv(ti, c_))]);
    pl.path(&mut c, &pts, "axp-c");
    pl.mark(&mut c, (1.0 / m, tv(ti + dt * (-1.0f64).exp(), c_)));
    pl.label(&mut c, (1.0 / m, tv(ti + dt * (-1.0f64).exp(), c_)), 10.0, -8.0, "axp-vt", &format!("x = 1/m = {}", q(1.0 / m, "m")));
    pl.label(&mut c, (5.0 / m, tv(ti, c_)), -4.0, -6.0, "axp-lt", &format!("Tinf = {}", temp(ti, c_)));
    c.bold(16.0, 24.0, "start", &format!("q = {}, m = {} 1/m", q(qq, "W"), num(m)));
    Some(s.figure("fin", "A very long fin: temperature along the rod",
                  &format!("Temperature along a very long fin (h = {}, perimeter {}, k = {}, cross-section {} m², base {} in \
                            fluid at {}): T = Tinf + (Tb - Tinf) e^(-m x) with m = h P (Tb - Tinf) / q = {} 1/m, derived \
                            from the model's heat rate q = {}. Marked: x = 1/m, where the excess has fallen to 1/e.",
                           qv(s, "h", h), q(p, "m"), qv(s, "k", kk), num(ac), temp(tb, c_), temp(ti, c_), num(m), q(qq, "W")),
                  c.body(), c.h))
}

/// A pin fin drawn to scale over its temperature excess cosh(mL(1 - x/L))/cosh(mL) (adiabatic tip) from the model's
/// mL; drawn only when the excess's mean reproduces the model's efficiency.
pub(crate) fn pin_fin(s: &Solved) -> Option<Figure> {
    let (h, kk, d, l, eta, ml) = (s.get("h")?, s.get("k")?, s.get("D")?, s.get("L")?, s.get("efficiency")?, s.get("mL")?);
    if !(ml > 0.0 && l > 0.0 && d > 0.0) {
        return None;
    }
    let n = 200;
    let th = |xi: f64| (ml * (1.0 - xi)).cosh() / ml.cosh();
    let mean: f64 = (0..n).map(|i| th((i as f64 + 0.5) / n as f64)).sum::<f64>() / n as f64;
    if (mean - eta).abs() > 2e-3 * eta.max(1e-9) {
        return None;
    }
    let pts: Vec<(f64, f64)> = k::span(0.0, 1.0, 41, false).iter().map(|xi| (*xi, th(*xi))).collect();
    let mut c = Canvas::new(434.0);
    let pl = Plot::new((96.0, 170.0, 470.0, 372.0), (0.0, 1.0, false), (0.0, 1.05, false), "p")?;
    let (x0, x1) = (pl.px(0.0), pl.px(1.0));
    let hgt = ((x1 - x0) * d / l).clamp(8.0, 70.0);
    let cy = 90.0;
    c.raw(&format!("<rect class=\"axp-ka\" x=\"{x0:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{hgt:.1}\"/>\
                    <rect class=\"axp-k\" x=\"{x0:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{hgt:.1}\"/>",
                   cy - hgt / 2.0, x1 - x0, cy - hgt / 2.0, x1 - x0));
    c.line((x0, cy - 50.0), (x0, cy + 50.0));
    for i in 0..9 {
        c.line((x0, cy - 46.0 + 11.0 * i as f64), (x0 - 8.0, cy - 38.0 + 11.0 * i as f64));
    }
    c.name((x0 + x1) / 2.0, cy - hgt / 2.0 - 10.0, "middle", &format!("L = {}", q(l, "m")));
    c.name(x1, cy + hgt / 2.0 + 20.0, "end", &format!("D = {}", q(d, "m")));
    c.name(x0 + 6.0, cy + hgt / 2.0 + 20.0, "start", &format!("k = {}, h = {}", qv(s, "k", kk), qv(s, "h", h)));
    let mut area = pts.clone();
    area.push((1.0, 0.0));
    area.push((0.0, 0.0));
    pl.area(&mut c, &area, "axp-s");
    pl.frame(&mut c, "Position along the fin x/L", "Excess θ/θb");
    pl.path(&mut c, &pts, "axp-c");
    pl.dashed(&mut c, &[(0.0, eta), (1.0, eta)]);
    pl.label(&mut c, (1.0, eta), -6.0, -8.0, "axp-mt", &format!("efficiency = {}", num(eta)));
    c.bold(16.0, 24.0, "start", &format!("mL = {}, efficiency = {}", num(ml), num(eta)));
    Some(s.figure("fin", "Pin fin and its temperature excess",
                  &format!("A pin fin (D = {}, L = {}, k = {}, h = {}) drawn to scale, over its temperature excess θ/θb = \
                            cosh(m(L - x))/cosh(mL) (adiabatic tip), derived from the model's mL = {}. The efficiency is the \
                            mean excess over the base excess (shaded area). Marked: the model's efficiency {}.", q(d, "m"),
                           q(l, "m"), qv(s, "k", kk), qv(s, "h", h), num(ml), num(eta)), c.body(), c.h))
}

/// Effectiveness against NTU for counterflow and parallel flow at the visitor's capacity rates, each point a run of
/// the model with another UA.
pub(crate) fn ntu_chart(s: &Solved) -> Option<Figure> {
    let (ua, ntu, eff) = (s.get("UA")?, s.get("NTU")?, s.get("effectiveness")?);
    let arr = s.txt("arrangement").unwrap_or_else(|| "counterflow".into());
    if !(ua > 0.0) {
        return None;
    }
    let uas = k::span(ua * 0.05, ua * 4.0, 36, false);
    let mut series = vec![];
    for a in ["counterflow", "parallel"] {
        let mut pts = vec![];
        for u in &uas {
            if let Some(b) = s.with_input(&[("UA", Input::Float(*u)), ("arrangement", Input::Text(a.into()))]) {
                if let (Some(n), Some(e)) = (k::num_of(&b, "NTU"), k::num_of(&b, "effectiveness")) {
                    pts.push((n, e));
                }
            }
        }
        if pts.len() >= 2 {
            series.push((a.to_string(), pts));
        }
    }
    if series.is_empty() {
        return None;
    }
    let body = k::lines_y(&series, "Number of transfer units NTU", "Effectiveness ε",
                          &format!("ε = {} at NTU = {} ({arr})", num(eff), num(ntu)), Some((ntu, eff)),
                          &format!("UA = {}", qv(s, "UA", ua)), false, true);
    let q_ = s.get("Q")?;
    Some(s.chart("chart", "Heat-exchanger effectiveness against NTU",
                 &format!("Heat-exchanger effectiveness against the number of transfer units for counterflow and parallel \
                           flow with heat-capacity rates {} and {} W/K, computed by the model at 36 values of UA. Marked: \
                           {arr}, UA = {} W/K, NTU = {}, ε = {}, Q = {}.", num(s.get("C_hot")?), num(s.get("C_cold")?),
                          num(ua), num(ntu), num(eff), q(q_, "W")), &body))
}

/// Stream temperatures along an exchanger, each position the model's own solution for the part of the exchanger
/// from the hot inlet to it (a share x of the conductance). Counterflow: the cold temperature there is the inlet
/// that makes the part's cold outlet equal the whole exchanger's (the outlets are linear in it: two runs per point).
/// Returns (x, Th, Tc) from the hot inlet (x = 0) to the other end (x = 1).
fn hx_profile(s: &Solved, ch: f64, cc: f64, ua: f64, thi: f64, tci: f64, arr: &str) -> Option<Vec<(f64, f64, f64)>> {
    let part = |u: f64, tc_in: f64| -> Option<(f64, f64)> {
        let a = run(s, "hx_effectiveness_ntu", &[("C_hot", Input::Float(ch)), ("C_cold", Input::Float(cc)),
                                                   ("UA", Input::Float(u)), ("Th_in", Input::Float(thi)),
                                                   ("Tc_in", Input::Float(tc_in)), ("arrangement", Input::Text(arr.into()))])?;
        Some((k::num_of(&a, "Th_out")?, k::num_of(&a, "Tc_out")?))
    };
    let (tho, tco) = part(ua, tci)?;
    let mut out = vec![(0.0, thi, if arr == "parallel" { tci } else { tco })];
    for i in 1..=16 {
        let x = i as f64 / 16.0;
        if arr == "parallel" {
            let (h, c) = part(ua * x, tci)?;
            out.push((x, h, c));
        } else {
            let (g1, g2) = (tci, (tci + thi) / 2.0);
            let (h1, c1) = part(ua * x, g1)?;
            let (h2, c2) = part(ua * x, g2)?;
            if (c2 - c1).abs() < 1e-12 {
                return None;
            }
            let t = (tco - c1) / (c2 - c1);
            out.push((x, h1 + t * (h2 - h1), g1 + t * (g2 - g1)));
        }
    }
    // the far end must be the whole exchanger's own outlets
    let e = out.last()?;
    let scale = (thi - tci).abs().max(1.0);
    let (eh, ec) = if arr == "parallel" { (tho, tco) } else { (tho, tci) };
    if (e.1 - eh).abs() > 1e-6 * scale || (e.2 - ec).abs() > 1e-6 * scale {
        return None;
    }
    Some(out)
}

fn profile_fig(s: &Solved, prof: &[(f64, f64, f64)], arr: &str, c_: bool, header: &str) -> (String, f64) {
    let hot: Vec<(f64, f64)> = prof.iter().map(|p| (p.0, tv(p.1, c_))).collect();
    let cold: Vec<(f64, f64)> = prof.iter().map(|p| (p.0, tv(p.2, c_))).collect();
    let ys: Vec<f64> = hot.iter().chain(cold.iter()).map(|p| p.1).collect();
    let mut c = Canvas::new(340.0);
    let Some((ya, yb)) = range(&ys, false, false) else { return (String::new(), 340.0) };
    let Some(pl) = Plot::new((96.0, 44.0, 450.0, 278.0), (0.0, 1.0, false), (ya, yb, false), "x") else {
        return (String::new(), 340.0);
    };
    pl.frame(&mut c, "Position along the exchanger (share of its area)", tlab(c_));
    pl.path(&mut c, &hot, "axp-c");
    pl.path(&mut c, &cold, "axp-c2");
    pl.head(&mut c, hot[7], hot[9]);
    if arr == "parallel" {
        pl.head(&mut c, cold[7], cold[9]);
    } else {
        pl.head(&mut c, cold[9], cold[7]);
    }
    let (h0, h1, c0, c1) = (hot[0], hot[hot.len() - 1], cold[0], cold[cold.len() - 1]);
    for (p, dx, dy) in [(h0, 8.0, -8.0), (c0, 8.0, -8.0), (h1, 6.0, 4.0), (c1, 6.0, 4.0)] {
        pl.dot(&mut c, p);
        pl.label(&mut c, p, dx, dy, "axp-vt", &format!("{} {}", num(p.1), if c_ { "°C" } else { "K" }));
    }
    pl.label(&mut c, hot[8], 0.0, -10.0, "axp-lt", "hot");
    pl.label(&mut c, cold[8], 0.0, 18.0, "axp-lt", "cold");
    c.text(96.0, 20.0, "start", "axp-mt", header);
    let _ = s;
    (c.s, 340.0)
}

/// The hot and cold temperatures along the visitor's exchanger.
pub(crate) fn ntu_profile(s: &Solved) -> Option<Figure> {
    let (ch, cc, ua, thi, tci) = (s.get("C_hot")?, s.get("C_cold")?, s.get("UA")?, s.get("Th_in")?, s.get("Tc_in")?);
    let (tho, tco) = (s.get("Th_out")?, s.get("Tc_out")?);
    let arr = s.txt("arrangement").unwrap_or_else(|| "counterflow".into());
    let prof = hx_profile(s, ch, cc, ua, thi, tci, &arr)?;
    let c_ = thi < 1000.0 && thi > 250.0 && tci > 250.0;
    let (body, h) = profile_fig(s, &prof, &arr, c_, &format!("Q = {}", q(s.get("Q")?, "W")));
    if body.is_empty() {
        return None;
    }
    Some(s.figure("profile", &format!("A {arr} exchanger solved by ε-NTU: temperature profiles"),
                  &format!("Stream temperatures along a {arr} exchanger: hot stream C = {} W/K entering at {}, cold stream C \
                            = {} W/K entering at {}, UA = {} W/K. The outlets are the model's: hot {}, cold {}; each interior \
                            point is the model's solution for the part of the exchanger up to it (a share of UA).", num(ch),
                           temp(thi, c_), num(cc), temp(tci, c_), num(ua), temp(tho, c_), temp(tco, c_)), &body, h))
}

/// The LMTD's exchanger: capacity rates in the ratio the four end temperatures set, UA = Q / LMTD from the model's
/// LMTD; drawn only when the model's ε-NTU rating of that exchanger reproduces the given outlets.
pub(crate) fn lmtd_profile(s: &Solved) -> Option<Figure> {
    let (thi, tho, tci, tco, lm) = (s.get("Th_in")?, s.get("Th_out")?, s.get("Tc_in")?, s.get("Tc_out")?, s.get("lmtd")?);
    let arr = s.txt("arrangement").unwrap_or_else(|| "counterflow".into());
    if !(thi > tho && tco > tci && lm > 0.0) {
        return None;
    }
    let ch = 1000.0;
    let cc = ch * (thi - tho) / (tco - tci);
    let ua = ch * (thi - tho) / lm;
    let prof = hx_profile(s, ch, cc, ua, thi, tci, &arr)?;
    let e = prof.last()?;
    let far_cold = if arr == "parallel" { e.2 } else { prof[0].2 };
    let scale = (thi - tci).abs().max(1.0);
    if (e.1 - tho).abs() > 1e-6 * scale || (far_cold - tco).abs() > 1e-6 * scale {
        return None;
    }
    let c_ = celsius(s, "Th_in") || (thi < 1000.0 && thi > 250.0 && tci > 250.0);
    let (d1, d2) = (s.get("dT1")?, s.get("dT2")?);
    let (body, h) = profile_fig(s, &prof, &arr, c_, &format!("LMTD = {} K (ΔT1 = {} K, ΔT2 = {} K)", num(lm), num(d1), num(d2)));
    if body.is_empty() {
        return None;
    }
    Some(s.figure("profile", &format!("Temperature profiles of a {arr} exchanger and its LMTD"),
                  &format!("Stream temperatures along a {arr} heat exchanger with the given ends: hot {} to {}, cold {} to {}. \
                            The model returns the end differences ΔT1 = {} K, ΔT2 = {} K and the log-mean difference {} K; \
                            the curves between the ends are the model's ε-NTU solutions for parts of an exchanger with \
                            those ends (capacity rates in the ratio of the temperature changes, UA = Q / LMTD).",
                           temp(thi, c_), temp(tho, c_), temp(tci, c_), temp(tco, c_), num(d1), num(d2), num(lm)), &body, h))
}

/// A family of curves: the result against one input (half to one and a half times the visitor's value), one curve
/// for the visitor's value of a second input, half and twice it.
pub(crate) fn family(s: &Solved, out: &str, x: &str, param: &str) -> Option<Figure> {
    let (xv, pv, ov) = (s.get(x)?, s.get(param)?, s.get(out)?);
    // a temperature swept about the fluid temperature (an initial temperature), else about zero
    let base = if x == "Ti" { s.get("Tinf")? } else { 0.0 };
    let d = xv - base;
    if d == 0.0 || pv == 0.0 {
        return None;
    }
    let xs: Vec<f64> = if d > 0.0 {
        k::span(base + 0.5 * d, base + 1.5 * d, 31, false)
    } else {
        k::span(base + 1.5 * d, base + 0.5 * d, 31, false)
    };
    let xu = unit(s, s.key, x);
    let pu = unit(s, s.key, param);
    let ou = unit(s, s.key, out);
    let c_ = x == "Ti" && celsius(s, "Ti");
    let show = |v: f64| if c_ { v - 273.15 } else { v };
    let mut series = vec![];
    for f in [1.0, 0.5, 2.0] {
        let pts: Vec<(f64, f64)> = xs.iter()
            .filter_map(|xi| own(s, &[(x, *xi), (param, pv * f)]).map(|y| (show(*xi), show_out(out, y, c_)))).collect();
        if pts.len() >= 2 {
            series.push((format!("{param} = {}", k::q(pv * f, &pu)), pts));
        }
    }
    if series.is_empty() {
        return None;
    }
    let xl = if c_ { format!("{} {x} (°C)", desc(s, s.key, x)) } else { axis(s, x) };
    let yl = if out == "T" && c_ { format!("{} {out} (°C)", desc(s, s.key, out)) } else { axis(s, out) };
    let ovs = show_out(out, ov, c_);
    let body = k::lines_y(&series, &xl, &yl, &format!("{out} = {}", if out == "T" { temp(ov, c_) } else { k::q(ov, &ou) }),
                          Some((show(xv), ovs)), &format!("{x} = {}", if c_ { temp(xv, true) } else { k::q(xv, &xu) }), false,
                          true);
    let ds = desc(s, s.key, out);
    Some(s.chart("curve-2", &format!("{}: {} against {}, for three values of {}", name(s), ds, desc(s, s.key, x),
                                    desc(s, s.key, param)),
                 &format!("{out} ({ds}) against {x} ({}) over 31 values, one curve for each value of {param} ({}): the one \
                           given, half and twice it, each point computed by the model. Marked: the given point, {x} = {}, \
                           {out} = {}.", desc(s, s.key, x), desc(s, s.key, param),
                          if c_ { temp(xv, true) } else { k::q(xv, &xu) },
                          if out == "T" { temp(ov, c_) } else { k::q(ov, &ou) }), &body))
}

fn show_out(out: &str, y: f64, c_: bool) -> f64 {
    if out == "T" && c_ { y - 273.15 } else { y }
}
