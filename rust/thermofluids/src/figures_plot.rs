//! A plot panel placed anywhere on a figure's canvas (several per figure: P-V over T-s, a profile under a drawing),
//! with linear or logarithmic axes: the frame, ticks and grid in the site's figure classes, curves clipped to it.

use crate::figures::nice_ticks;
use ax_engine::figkit::{num, Canvas};

pub(crate) struct Plot {
    /// the plot area on the canvas: left, top, right, bottom
    pub r: (f64, f64, f64, f64),
    pub x: (f64, f64, bool),
    pub y: (f64, f64, bool),
    clip: String,
}

/// A padded range over values (logarithmic: by a factor; linear: 6 % each side), optionally from zero.
pub(crate) fn range(v: &[f64], log: bool, from_zero: bool) -> Option<(f64, f64)> {
    let v: Vec<f64> = v.iter().copied().filter(|x| x.is_finite() && (!log || *x > 0.0)).collect();
    if v.is_empty() {
        return None;
    }
    let (mut lo, mut hi) = v.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(a, b), x| (a.min(*x), b.max(*x)));
    if log {
        if hi <= lo {
            hi = lo * 10.0;
            lo /= 10.0;
        }
        return Some((lo / 1.15, hi * 1.15));
    }
    if from_zero {
        lo = lo.min(0.0);
        hi = hi.max(0.0);
    }
    if !(hi > lo) {
        let d = if lo == 0.0 { 1.0 } else { lo.abs() * 0.1 };
        return Some((lo - d, hi + d));
    }
    let d = (hi - lo) * 0.06;
    Some((if from_zero && lo == 0.0 { 0.0 } else { lo - d }, if from_zero && hi == 0.0 { 0.0 } else { hi + d }))
}

impl Plot {
    pub fn new(r: (f64, f64, f64, f64), x: (f64, f64, bool), y: (f64, f64, bool), tag: &str) -> Option<Plot> {
        let ok = |a: (f64, f64, bool)| a.0.is_finite() && a.1.is_finite() && a.1 > a.0 && (!a.2 || a.0 > 0.0);
        (ok(x) && ok(y)).then(|| Plot { r, x, y, clip: format!("CLIPID{tag}") })
    }
    fn f(v: f64, a: (f64, f64, bool)) -> f64 {
        if a.2 { (v.log10() - a.0.log10()) / (a.1.log10() - a.0.log10()) } else { (v - a.0) / (a.1 - a.0) }
    }
    pub fn px(&self, v: f64) -> f64 {
        self.r.0 + Plot::f(v, self.x) * (self.r.2 - self.r.0)
    }
    pub fn py(&self, v: f64) -> f64 {
        self.r.3 - Plot::f(v, self.y) * (self.r.3 - self.r.1)
    }
    pub fn p(&self, pt: (f64, f64)) -> (f64, f64) {
        (self.px(pt.0), self.py(pt.1))
    }
    fn ticks(a: (f64, f64, bool)) -> Vec<f64> {
        if a.2 {
            let (e0, e1) = (a.0.log10().ceil() as i32, a.1.log10().floor() as i32);
            let mut v: Vec<f64> = (e0..=e1).map(|e| 10f64.powi(e)).collect();
            if v.len() > 7 {
                v = v.into_iter().step_by(2).collect();
            }
            if v.len() < 2 {
                // less than two decades: 1, 2, 5 steps
                v = (e0 - 1..=e1 + 1).flat_map(|e| [1.0, 2.0, 5.0].map(|m| m * 10f64.powi(e)))
                    .filter(|t| *t >= a.0 && *t <= a.1).collect();
            }
            return v;
        }
        nice_ticks(a.0, a.1, 5)
    }
    /// Grid, ticks, the frame and the axis labels (the vertical one turned).
    pub fn frame(&self, c: &mut Canvas, xlab: &str, ylab: &str) {
        let (l, t, r, b) = self.r;
        for v in Plot::ticks(self.x) {
            let x = self.px(v);
            c.raw(&format!("<line class=\"axp-g\" x1=\"{x:.1}\" y1=\"{t:.1}\" x2=\"{x:.1}\" y2=\"{b:.1}\"/>"));
            c.text(x, b + 18.0, "middle", "axp-t", &num(v));
        }
        for v in Plot::ticks(self.y) {
            let y = self.py(v);
            c.raw(&format!("<line class=\"axp-g\" x1=\"{l:.1}\" y1=\"{y:.1}\" x2=\"{r:.1}\" y2=\"{y:.1}\"/>"));
            c.text(l - 6.0, y + 5.0, "end", "axp-t", &num(v));
        }
        c.raw(&format!("<rect class=\"axp-f\" x=\"{l:.1}\" y=\"{t:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>", r - l, b - t));
        c.raw(&format!("<clipPath id=\"{}\"><rect x=\"{l:.1}\" y=\"{t:.1}\" width=\"{:.1}\" height=\"{:.1}\"/></clipPath>",
                       self.clip, r - l, b - t));
        if !xlab.is_empty() {
            c.text((l + r) / 2.0, b + 40.0, "middle", "axp-l", xlab);
        }
        if !ylab.is_empty() {
            c.raw(&format!("<text class=\"axp-l\" transform=\"translate({:.1},{:.1}) rotate(-90)\" text-anchor=\"middle\">{}</text>",
                           (l - 58.0).max(16.0), (t + b) / 2.0, esc(ylab)));
        }
    }
    /// A polyline through points (non-finite points break it), clipped to the plot.
    pub fn path(&self, c: &mut Canvas, pts: &[(f64, f64)], class: &str) {
        let d = self.d(pts);
        if !d.is_empty() {
            c.raw(&format!("<path class=\"{class}\" clip-path=\"url(#{})\" d=\"{d}\"/>", self.clip));
        }
    }
    /// A dashed polyline (a guide).
    pub fn dashed(&self, c: &mut Canvas, pts: &[(f64, f64)]) {
        let d = self.d(pts);
        if !d.is_empty() {
            c.raw(&format!("<path class=\"axp-c4\" stroke-dasharray=\"5 4\" clip-path=\"url(#{})\" d=\"{d}\"/>", self.clip));
        }
    }
    /// A filled region (a closed polygon), clipped.
    pub fn area(&self, c: &mut Canvas, pts: &[(f64, f64)], class: &str) {
        let d = self.d(pts);
        if !d.is_empty() {
            c.raw(&format!("<path class=\"{class}\" style=\"stroke:none\" clip-path=\"url(#{})\" d=\"{d} Z\"/>", self.clip));
        }
    }
    fn d(&self, pts: &[(f64, f64)]) -> String {
        let mut d = String::new();
        let mut pen = false;
        for p in pts {
            let ok = p.0.is_finite() && p.1.is_finite() && (!self.x.2 || p.0 > 0.0) && (!self.y.2 || p.1 > 0.0);
            if !ok {
                pen = false;
                continue;
            }
            let (x, y) = self.p(*p);
            d.push_str(&format!("{}{x:.1},{y:.1} ", if pen { "L" } else { "M" }));
            pen = true;
        }
        d.trim_end().to_string()
    }
    /// The visitor's point (a ringed dot).
    pub fn mark(&self, c: &mut Canvas, pt: (f64, f64)) {
        let (x, y) = self.p(pt);
        if x.is_finite() && y.is_finite() {
            c.raw(&format!("<circle class=\"axp-k\" cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"7\"/>\
                            <circle class=\"axp-m\" cx=\"{x:.1}\" cy=\"{y:.1}\" r=\"4\"/>"));
        }
    }
    /// A small dot on a curve (a state of a cycle).
    pub fn dot(&self, c: &mut Canvas, pt: (f64, f64)) {
        let (x, y) = self.p(pt);
        if x.is_finite() && y.is_finite() {
            c.dot((x, y));
        }
    }
    /// An arrowhead on a curve between two of its points (its direction).
    pub fn head(&self, c: &mut Canvas, a: (f64, f64), b: (f64, f64)) {
        let (pa, pb) = (self.p(a), self.p(b));
        let (dx, dy) = (pb.0 - pa.0, pb.1 - pa.1);
        let d = (dx * dx + dy * dy).sqrt();
        if !(d > 0.5) {
            return;
        }
        let (u, n) = ((dx / d, dy / d), (-dy / d, dx / d));
        let m = ((pa.0 + pb.0) / 2.0 + u.0 * 5.0, (pa.1 + pb.1) / 2.0 + u.1 * 5.0);
        let bb = (m.0 - u.0 * 10.0, m.1 - u.1 * 10.0);
        c.raw(&format!("<path class=\"axp-kd\" d=\"M{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} Z\"/>", m.0, m.1,
                       bb.0 + n.0 * 4.5, bb.1 + n.1 * 4.5, bb.0 - n.0 * 4.5, bb.1 - n.1 * 4.5));
    }
    /// Text beside a point of the plot, kept inside the plot's width.
    pub fn label(&self, c: &mut Canvas, pt: (f64, f64), dx: f64, dy: f64, class: &str, t: &str) {
        let (x, y) = self.p(pt);
        if x.is_finite() && y.is_finite() {
            let anchor = if dx < 0.0 { "end" } else { "start" };
            c.text(x + dx, y + dy, anchor, class, t);
        }
    }
}

pub(crate) fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// The point of a curve at its largest horizontal value (where a curve's name is written).
pub(crate) fn last(pts: &[(f64, f64)]) -> Option<(f64, f64)> {
    pts.iter().copied().filter(|p| p.0.is_finite() && p.1.is_finite()).last()
}
