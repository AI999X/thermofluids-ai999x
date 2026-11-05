//! thermofluids: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/thermofluids/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "churchill_bernstein_cylinder",
    "churchill_chu_vertical_plate",
    "colebrook_friction_factor",
    "dew_point_temperature",
    "equivalence_ratio",
    "gnielinski_nusselt",
    "humidity_ratio_from_relative_humidity",
    "hx_effectiveness_ntu",
    "lmtd",
    "pin_fin_efficiency",
    "saturation_vapour_pressure",
    "stoichiometric_air_fuel_ratio",
    "wet_bulb_temperature",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "churchill_bernstein_cylinder" => {
            let r = f_churchill_bernstein(match v.get("Re") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pr") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_cb(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "churchill_chu_vertical_plate" => {
            let r = f_churchill_chu(match v.get("Ra") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pr") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_cc(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "colebrook_friction_factor" => {
            let r = f_colebrook(match v.get("Re") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eD") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_colebrook(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "dew_point_temperature" => {
            let r = f_dew_point(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RH") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_dew(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "equivalence_ratio" => {
            let r = f_equivalence(match v.get("fuel") { Some(x) => py::from_val(x), None => { return None } }, match v.get("afr") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_equiv(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gnielinski_nusselt" => {
            let r = f_gnielinski(match v.get("Re") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pr") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_gn(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "humidity_ratio_from_relative_humidity" => {
            let r = f_hum_ratio_rh(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RH") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_hum(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "hx_effectiveness_ntu" => {
            let r = f_hx_rating(match v.get("C_hot") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_cold") { Some(x) => py::from_val(x), None => { return None } }, match v.get("UA") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Th_in") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tc_in") { Some(x) => py::from_val(x), None => { return None } }, match v.get("arrangement") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_hx(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "lmtd" => {
            let r = f_lmtd(match v.get("Th_in") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Th_out") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tc_in") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tc_out") { Some(x) => py::from_val(x), None => { return None } }, match v.get("arrangement") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_lmtd(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "pin_fin_efficiency" => {
            let r = f_pin_fin(match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_fin(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "saturation_vapour_pressure" => {
            let r = f_sat_pressure(match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_117(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "stoichiometric_air_fuel_ratio" => {
            let r = f_stoich(match v.get("fuel") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_stoich(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "wet_bulb_temperature" => {
            let r = f_wet_bulb(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RH") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_wb(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c_AIR_PER_O2() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(137.33064)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_AW() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("H"), Py::Float(1.008)), (Py::str("C"), Py::Float(12.011)), (Py::str("O"), Py::Float(15.999)), (Py::str("N"), Py::Float(14.007))])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_FUELS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("methane"), Py::tuple(vec![Py::str("CH4"), Py::Int(1i64), Py::Int(4i64)])), (Py::str("ethane"), Py::tuple(vec![Py::str("C2H6"), Py::Int(2i64), Py::Int(6i64)])), (Py::str("propane"), Py::tuple(vec![Py::str("C3H8"), Py::Int(3i64), Py::Int(8i64)])), (Py::str("ethylene"), Py::tuple(vec![Py::str("C2H4"), Py::Int(2i64), Py::Int(4i64)])), (Py::str("acetylene"), Py::tuple(vec![Py::str("C2H2"), Py::Int(2i64), Py::Int(2i64)])), (Py::str("hydrogen"), Py::tuple(vec![Py::str("H2"), Py::Int(0i64), Py::Int(2i64)]))])) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lm(mut v_d1: Py, mut v_d2: Py) -> R {
    if ({ let t100 = { let t96 = v_d1.clone(); Py::Bool({ let t97 = Py::Int(0i64); if py::le(&t96, &t97)? { true } else { false } }) }; if t100.truthy() { t100 } else { { let t98 = v_d2.clone(); Py::Bool({ let t99 = Py::Int(0i64); if py::le(&t98, &t99)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("each terminal temperature difference must be positive (the streams' temperatures cross)"))));
    }
    return Ok((if ({ let t101 = py::abs(&py::sub(&v_d1.clone(), &v_d2.clone())?)?; Py::Bool({ let t102 = py::mul(&Py::Float(1e-12), &py::minmax(&[v_d1.clone(), v_d2.clone()], true)?)?; if py::le(&t101, &t102)? { true } else { false } }) }).truthy() { v_d1.clone() } else { py::truediv(&py::sub(&v_d1.clone(), &v_d2.clone())?, &py::math1("log", &py::truediv(&v_d1.clone(), &v_d2.clone())?)?)? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rh(mut v_RH: Py) -> R {
    if (py::not(&{ let t120 = Py::Int(0i64); Py::Bool({ let t121 = v_RH.clone(); if py::lt(&t120, &t121)? { { let t122 = Py::Int(1i64); if py::le(&t121, &t122)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("relative humidity must be a fraction between 0 and 1 (or a percentage written with %)"))));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__solve_T(mut v_fun: Py, mut v_lo: Py, mut v_hi: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_flo: Py = Py::None;
    let mut v_fm: Py = Py::None;
    let mut v_mid: Py = Py::None;
    v_flo = py::call(&v_fun.clone(), &[v_lo.clone()])?;
    for t132 in py::range_iter(&[(Py::Int(200i64)).i()?]) {
        v__ = t132;
        v_mid = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
        if ({ let t133 = v_mid.clone(); Py::Bool({ let t134 = Py::tuple(vec![v_lo.clone(), v_hi.clone()]); if py::contains(&t134, &t133)? { true } else { false } }) }).truthy() {
            break;
        }
        v_fm = py::call(&v_fun.clone(), &[v_mid.clone()])?;
        if ({ let t135 = { let t137 = v_fm.clone(); Py::Bool({ let t138 = Py::Int(0i64); if py::lt(&t137, &t138)? { true } else { false } }) }; Py::Bool({ let t136 = { let t139 = v_flo.clone(); Py::Bool({ let t140 = Py::Int(0i64); if py::lt(&t139, &t140)? { true } else { false } }) }; if py::eq(&t135, &t136) { true } else { false } }) }).truthy() {
            let t141 = (Py::tuple(vec![v_mid.clone(), v_fm.clone()])).items()?;
            if t141.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_lo = t141[0].clone();
            v_flo = t141[1].clone();
        } else {
            v_hi = v_mid.clone();
        }
    }
    return Ok(py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__w_from_wb(mut v_T: Py, mut v_Twb: Py, mut v_P: Py) -> R {
    let mut v_ps: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_tw: Py = Py::None;
    let mut v_ws: Py = Py::None;
    let t144 = (Py::tuple(vec![py::sub(&v_T.clone(), &Py::Float(273.15))?, py::sub(&v_Twb.clone(), &Py::Float(273.15))?])).items()?;
    if t144.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_t = t144[0].clone();
    v_tw = t144[1].clone();
    v_ps = f_pws(v_Twb.clone())?;
    v_ws = py::truediv(&py::mul(&Py::Float(0.621945), &v_ps.clone())?, &py::sub(&v_P.clone(), &v_ps.clone())?)?;
    if ({ let t145 = v_tw.clone(); Py::Bool({ let t146 = Py::Int(0i64); if py::ge(&t145, &t146)? { true } else { false } }) }).truthy() {
        return Ok(py::truediv(&py::sub(&py::mul(&py::sub(&Py::Int(2501i64), &py::mul(&Py::Float(2.326), &v_tw.clone())?)?, &v_ws.clone())?, &py::mul(&Py::Float(1.006), &py::sub(&v_t.clone(), &v_tw.clone())?)?)?, &py::sub(&py::add(&Py::Int(2501i64), &py::mul(&Py::Float(1.86), &v_t.clone())?)?, &py::mul(&Py::Float(4.186), &v_tw.clone())?)?)?);
    }
    return Ok(py::truediv(&py::sub(&py::mul(&py::sub(&Py::Int(2830i64), &py::mul(&Py::Float(0.24), &v_tw.clone())?)?, &v_ws.clone())?, &py::mul(&Py::Float(1.006), &py::sub(&v_t.clone(), &v_tw.clone())?)?)?, &py::sub(&py::add(&Py::Int(2830i64), &py::mul(&Py::Float(1.86), &v_t.clone())?)?, &py::mul(&Py::Float(2.1), &v_tw.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_churchill_bernstein(mut v_Re: Py, mut v_Pr: Py) -> R {
    if (py::not(&{ let t8 = { let t1 = v_Re.clone(); Py::Bool({ let t2 = Py::Int(0i64); if py::gt(&t1, &t2)? { true } else { false } }) }; if !t8.truthy() { t8 } else { { let t7 = { let t3 = v_Pr.clone(); Py::Bool({ let t4 = Py::Int(0i64); if py::gt(&t3, &t4)? { true } else { false } }) }; if !t7.truthy() { t7 } else { { let t5 = py::mul(&v_Re.clone(), &v_Pr.clone())?; Py::Bool({ let t6 = Py::Float(0.2); if py::ge(&t5, &t6)? { true } else { false } }) } } } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("Churchill-Bernstein holds for Re * Pr >= 0.2"))));
    }
    return Ok(Py::dict(vec![(Py::str("Nu"), py::add(&Py::Float(0.3), &py::mul(&py::truediv(&py::mul(&py::mul(&Py::Float(0.62), &py::pow(&v_Re.clone(), &Py::Float(0.5))?)?, &py::pow(&v_Pr.clone(), &py::truediv(&Py::Int(1i64), &Py::Int(3i64))?)?)?, &py::pow(&py::add(&Py::Int(1i64), &py::pow(&py::truediv(&Py::Float(0.4), &v_Pr.clone())?, &py::truediv(&Py::Int(2i64), &Py::Int(3i64))?)?)?, &Py::Float(0.25))?)?, &py::pow(&py::add(&Py::Int(1i64), &py::pow(&py::truediv(&v_Re.clone(), &Py::Int(282000i64))?, &Py::Float(0.625))?)?, &Py::Float(0.8))?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_churchill_chu(mut v_Ra: Py, mut v_Pr: Py) -> R {
    let mut v_s: Py = Py::None;
    if (py::not(&{ let t15 = { let t10 = Py::Int(0i64); Py::Bool({ let t11 = v_Ra.clone(); if py::lt(&t10, &t11)? { { let t12 = Py::Float(10000000000000.0); if py::le(&t11, &t12)? { true } else { false } } } else { false } }) }; if !t15.truthy() { t15 } else { { let t13 = v_Pr.clone(); Py::Bool({ let t14 = Py::Int(0i64); if py::gt(&t13, &t14)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("Churchill-Chu holds for 0 < Ra <= 1e13 and Pr > 0"))));
    }
    v_s = py::add(&Py::Float(0.825), &py::truediv(&py::mul(&Py::Float(0.387), &py::pow(&v_Ra.clone(), &py::truediv(&Py::Int(1i64), &Py::Int(6i64))?)?)?, &py::pow(&py::add(&Py::Int(1i64), &py::pow(&py::truediv(&Py::Float(0.492), &v_Pr.clone())?, &py::truediv(&Py::Int(9i64), &Py::Int(16i64))?)?)?, &py::truediv(&Py::Int(8i64), &Py::Int(27i64))?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("Nu"), py::mul(&v_s.clone(), &v_s.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_colebrook(mut v_Re: Py, mut v_eD: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_dg: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_step: Py = Py::None;
    let mut v_x: Py = Py::None;
    if (py::not(&{ let t20 = py::math1("isfinite", &v_Re.clone())?; if !t20.truthy() { t20 } else { { let t18 = v_Re.clone(); Py::Bool({ let t19 = Py::Int(4000i64); if py::ge(&t18, &t19)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the Colebrook equation is for turbulent flow: Re must be at least 4000"))));
    }
    if (py::not(&{ let t21 = Py::Int(0i64); Py::Bool({ let t22 = v_eD.clone(); if py::le(&t21, &t22)? { { let t23 = Py::Float(0.05); if py::le(&t22, &t23)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("relative roughness eD must lie between 0 and 0.05"))));
    }
    v_x = py::truediv(&Py::Int(1i64), &py::math1("sqrt", &py::truediv(&Py::Float(0.25), &py::pow(&py::math1("log10", &py::add(&py::truediv(&v_eD.clone(), &Py::Float(3.7))?, &py::truediv(&Py::Float(5.74), &py::pow(&v_Re.clone(), &Py::Float(0.9))?)?)?)?, &Py::Int(2i64))?)?)?)?;
    for t24 in py::range_iter(&[(Py::Int(100i64)).i()?]) {
        v__ = t24;
        let t25 = (Py::tuple(vec![py::truediv(&v_eD.clone(), &Py::Float(3.7))?, py::truediv(&Py::Float(2.51), &v_Re.clone())?])).items()?;
        if t25.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t25[0].clone();
        v_b = t25[1].clone();
        v_g = py::add(&v_x.clone(), &py::mul(&Py::Int(2i64), &py::math1("log10", &py::add(&v_a.clone(), &py::mul(&v_b.clone(), &v_x.clone())?)?)?)?)?;
        v_dg = py::add(&Py::Int(1i64), &py::truediv(&py::mul(&Py::Int(2i64), &v_b.clone())?, &py::mul(&py::add(&v_a.clone(), &py::mul(&v_b.clone(), &v_x.clone())?)?, &py::math1("log", &Py::Int(10i64))?)?)?)?;
        v_step = py::truediv(&v_g.clone(), &v_dg.clone())?;
        v_x = py::sub(&v_x, &v_step.clone())?;
        if ({ let t26 = py::abs(&v_step.clone())?; Py::Bool({ let t27 = py::mul(&Py::Float(1e-15), &py::abs(&v_x.clone())?)?; if py::le(&t26, &t27)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    return Ok(Py::dict(vec![(Py::str("f"), py::truediv(&Py::Int(1i64), &py::mul(&v_x.clone(), &v_x.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dew_point(mut v_T: Py, mut v_RH: Py) -> R {
    let mut v_Td: Py = Py::None;
    let mut v_pv: Py = Py::None;
    let _ = f__rh(v_RH.clone())?;
    v_pv = py::mul(&v_RH.clone(), &f_pws(v_T.clone())?)?;
    v_Td = (if ({ let t130 = v_RH.clone(); Py::Bool({ let t131 = Py::Int(1i64); if py::eq(&t130, &t131) { true } else { false } }) }).truthy() { v_T.clone() } else { f__solve_T({ let v_RH = v_RH.clone(); let v_T = v_T.clone(); let v_pv = v_pv.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&f_pws(v_x.clone())?, &v_pv.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::Float(173.15), v_T.clone())? });
    return Ok(Py::dict(vec![(Py::str("T_dew"), v_Td.clone()), (Py::str("p_v"), v_pv.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_eps_ntu(mut v_NTU: Py, mut v_Cr: Py, mut v_arrangement: Py) -> R {
    let mut v_e: Py = Py::None;
    if ({ let t82 = v_arrangement.clone(); Py::Bool({ let t83 = Py::str("counterflow"); if py::eq(&t82, &t83) { true } else { false } }) }).truthy() {
        if ({ let t84 = py::abs(&py::sub(&Py::Int(1i64), &v_Cr.clone())?)?; Py::Bool({ let t85 = Py::Float(1e-12); if py::lt(&t84, &t85)? { true } else { false } }) }).truthy() {
            return Ok(py::truediv(&v_NTU.clone(), &py::add(&Py::Int(1i64), &v_NTU.clone())?)?);
        }
        v_e = py::math1("exp", &py::mul(&py::neg(&v_NTU.clone())?, &py::sub(&Py::Int(1i64), &v_Cr.clone())?)?)?;
        return Ok(py::truediv(&py::sub(&Py::Int(1i64), &v_e.clone())?, &py::sub(&Py::Int(1i64), &py::mul(&v_Cr.clone(), &v_e.clone())?)?)?);
    }
    return Ok(py::truediv(&py::sub(&Py::Int(1i64), &py::math1("exp", &py::mul(&py::neg(&v_NTU.clone())?, &py::add(&Py::Int(1i64), &v_Cr.clone())?)?)?)?, &py::add(&Py::Int(1i64), &v_Cr.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_equivalence(mut v_fuel: Py, mut v_afr: Py) -> R {
    let mut v_s: Py = Py::None;
    if (py::not(&{ let t51 = v_afr.clone(); Py::Bool({ let t52 = Py::Int(0i64); if py::gt(&t51, &t52)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the air-fuel ratio must be positive"))));
    }
    v_s = py::getitem(&f_stoich(v_fuel.clone())?, &Py::str("afr_stoich"))?;
    return Ok(Py::dict(vec![(Py::str("phi"), py::truediv(&v_s.clone(), &v_afr.clone())?), (Py::str("afr_stoich"), v_s.clone()), (Py::str("excess_air"), py::sub(&py::truediv(&v_afr.clone(), &v_s.clone())?, &Py::Int(1i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gnielinski(mut v_Re: Py, mut v_Pr: Py, mut v_f: Py) -> R {
    if (py::not(&{ let t64 = { let t54 = Py::Int(3000i64); Py::Bool({ let t55 = v_Re.clone(); if py::le(&t54, &t55)? { { let t56 = Py::Float(5000000.0); if py::le(&t55, &t56)? { true } else { false } } } else { false } }) }; if !t64.truthy() { t64 } else { { let t63 = { let t57 = Py::Float(0.5); Py::Bool({ let t58 = v_Pr.clone(); if py::le(&t57, &t58)? { { let t59 = Py::Int(2000i64); if py::le(&t58, &t59)? { true } else { false } } } else { false } }) }; if !t63.truthy() { t63 } else { { let t60 = Py::Int(0i64); Py::Bool({ let t61 = v_f.clone(); if py::lt(&t60, &t61)? { { let t62 = Py::Float(0.2); if py::lt(&t61, &t62)? { true } else { false } } } else { false } }) } } } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("Gnielinski holds for 3000 <= Re <= 5e6, 0.5 <= Pr <= 2000 and a Darcy friction factor 0 < f < 0.2"))));
    }
    return Ok(Py::dict(vec![(Py::str("Nu"), py::truediv(&py::mul(&py::mul(&py::truediv(&v_f.clone(), &Py::Int(8i64))?, &py::sub(&v_Re.clone(), &Py::Int(1000i64))?)?, &v_Pr.clone())?, &py::add(&Py::Int(1i64), &py::mul(&py::mul(&Py::Float(12.7), &py::math1("sqrt", &py::truediv(&v_f.clone(), &Py::Int(8i64))?)?)?, &py::sub(&py::pow(&v_Pr.clone(), &py::truediv(&Py::Int(2i64), &Py::Int(3i64))?)?, &Py::Int(1i64))?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hum_ratio_rh(mut v_T: Py, mut v_RH: Py, mut v_P: Py) -> R {
    let mut v_pv: Py = Py::None;
    let _ = f__rh(v_RH.clone())?;
    v_pv = py::mul(&v_RH.clone(), &f_pws(v_T.clone())?)?;
    if (py::not(&{ let t128 = v_P.clone(); Py::Bool({ let t129 = v_pv.clone(); if py::gt(&t128, &t129)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the total pressure must exceed the vapour pressure"))));
    }
    return Ok(Py::dict(vec![(Py::str("W"), py::truediv(&py::mul(&Py::Float(0.621945), &v_pv.clone())?, &py::sub(&v_P.clone(), &v_pv.clone())?)?), (Py::str("p_v"), v_pv.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hx_rating(mut v_C_hot: Py, mut v_C_cold: Py, mut v_UA: Py, mut v_Th_in: Py, mut v_Tc_in: Py, mut v_arrangement: Py) -> R {
    let mut v_Cmax: Py = Py::None;
    let mut v_Cmin: Py = Py::None;
    let mut v_Cr: Py = Py::None;
    let mut v_NTU: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_eps: Py = Py::None;
    if ({ let t76 = py::minmax(&[v_C_hot.clone(), v_C_cold.clone(), v_UA.clone()], false)?; Py::Bool({ let t77 = Py::Int(0i64); if py::le(&t76, &t77)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("heat capacity rates and UA must be positive"))));
    }
    if ({ let t78 = v_Th_in.clone(); Py::Bool({ let t79 = v_Tc_in.clone(); if py::le(&t78, &t79)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the hot inlet must be hotter than the cold inlet"))));
    }
    let t80 = (Py::tuple(vec![py::minmax(&[v_C_hot.clone(), v_C_cold.clone()], false)?, py::minmax(&[v_C_hot.clone(), v_C_cold.clone()], true)?])).items()?;
    if t80.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Cmin = t80[0].clone();
    v_Cmax = t80[1].clone();
    let t81 = (Py::tuple(vec![py::truediv(&v_UA.clone(), &v_Cmin.clone())?, py::truediv(&v_Cmin.clone(), &v_Cmax.clone())?])).items()?;
    if t81.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_NTU = t81[0].clone();
    v_Cr = t81[1].clone();
    v_eps = f_eps_ntu(v_NTU.clone(), v_Cr.clone(), v_arrangement.clone())?;
    v_Q = py::mul(&py::mul(&v_eps.clone(), &v_Cmin.clone())?, &py::sub(&v_Th_in.clone(), &v_Tc_in.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("Q"), v_Q.clone()), (Py::str("Th_out"), py::sub(&v_Th_in.clone(), &py::truediv(&v_Q.clone(), &v_C_hot.clone())?)?), (Py::str("Tc_out"), py::add(&v_Tc_in.clone(), &py::truediv(&v_Q.clone(), &v_C_cold.clone())?)?), (Py::str("effectiveness"), v_eps.clone()), (Py::str("NTU"), v_NTU.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lmtd(mut v_Th_in: Py, mut v_Th_out: Py, mut v_Tc_in: Py, mut v_Tc_out: Py, mut v_arrangement: Py) -> R {
    let mut v_d1: Py = Py::None;
    let mut v_d2: Py = Py::None;
    if ({ let t91 = { let t87 = v_Th_out.clone(); Py::Bool({ let t88 = v_Th_in.clone(); if py::gt(&t87, &t88)? { true } else { false } }) }; if t91.truthy() { t91 } else { { let t89 = v_Tc_out.clone(); Py::Bool({ let t90 = v_Tc_in.clone(); if py::lt(&t89, &t90)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the hot stream must cool and the cold stream must warm"))));
    }
    if ({ let t92 = v_arrangement.clone(); Py::Bool({ let t93 = Py::str("counterflow"); if py::eq(&t92, &t93) { true } else { false } }) }).truthy() {
        let t94 = (Py::tuple(vec![py::sub(&v_Th_in.clone(), &v_Tc_out.clone())?, py::sub(&v_Th_out.clone(), &v_Tc_in.clone())?])).items()?;
        if t94.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_d1 = t94[0].clone();
        v_d2 = t94[1].clone();
    } else {
        let t95 = (Py::tuple(vec![py::sub(&v_Th_in.clone(), &v_Tc_in.clone())?, py::sub(&v_Th_out.clone(), &v_Tc_out.clone())?])).items()?;
        if t95.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_d1 = t95[0].clone();
        v_d2 = t95[1].clone();
    }
    return Ok(Py::dict(vec![(Py::str("lmtd"), f__lm(v_d1.clone(), v_d2.clone())?), (Py::str("dT1"), v_d1.clone()), (Py::str("dT2"), v_d2.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pin_fin(mut v_h: Py, mut v_k: Py, mut v_D: Py, mut v_L: Py) -> R {
    let mut v_eta: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_mL: Py = Py::None;
    if ({ let t110 = py::minmax(&[v_h.clone(), v_k.clone(), v_D.clone(), v_L.clone()], false)?; Py::Bool({ let t111 = Py::Int(0i64); if py::le(&t110, &t111)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("h, k, D and L must be positive"))));
    }
    v_m = py::math1("sqrt", &py::truediv(&py::mul(&Py::Int(4i64), &v_h.clone())?, &py::mul(&v_k.clone(), &v_D.clone())?)?)?;
    v_mL = py::mul(&v_m.clone(), &v_L.clone())?;
    v_eta = py::truediv(&py::math1("tanh", &v_mL.clone())?, &v_mL.clone())?;
    return Ok(Py::dict(vec![(Py::str("efficiency"), v_eta.clone()), (Py::str("mL"), v_mL.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pws(mut v_T: Py) -> R {
    let mut v_ln: Py = Py::None;
    let mut v_t: Py = Py::None;
    v_t = py::sub(&v_T.clone(), &Py::Float(273.15))?;
    if (py::not(&{ let t123 = py::neg(&Py::Int(100i64))?; Py::Bool({ let t124 = v_t.clone(); if py::le(&t123, &t124)? { { let t125 = Py::Int(200i64); if py::le(&t124, &t125)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("temperature must lie between -100 °C and 200 °C"))));
    }
    if ({ let t126 = v_t.clone(); Py::Bool({ let t127 = Py::Float(0.01); if py::le(&t126, &t127)? { true } else { false } }) }).truthy() {
        v_ln = py::add(&py::sub(&py::add(&py::add(&py::sub(&py::add(&py::truediv(&py::neg(&Py::Float(5674.5359))?, &v_T.clone())?, &Py::Float(6.3925247))?, &py::mul(&Py::Float(0.009677843), &v_T.clone())?)?, &py::mul(&Py::Float(6.2215701e-07), &py::pow(&v_T.clone(), &Py::Int(2i64))?)?)?, &py::mul(&Py::Float(2.0747825e-09), &py::pow(&v_T.clone(), &Py::Int(3i64))?)?)?, &py::mul(&Py::Float(9.484024e-13), &py::pow(&v_T.clone(), &Py::Int(4i64))?)?)?, &py::mul(&Py::Float(4.1635019), &py::math1("log", &v_T.clone())?)?)?;
    } else {
        v_ln = py::add(&py::sub(&py::add(&py::sub(&py::add(&py::truediv(&py::neg(&Py::Float(5800.2206))?, &v_T.clone())?, &Py::Float(1.3914993))?, &py::mul(&Py::Float(0.048640239), &v_T.clone())?)?, &py::mul(&Py::Float(4.1764768e-05), &py::pow(&v_T.clone(), &Py::Int(2i64))?)?)?, &py::mul(&Py::Float(1.4452093e-08), &py::pow(&v_T.clone(), &Py::Int(3i64))?)?)?, &py::mul(&Py::Float(6.5459673), &py::math1("log", &v_T.clone())?)?)?;
    }
    return Ok(py::math1("exp", &v_ln.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sat_pressure(mut v_T: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("p_sat"), f_pws(v_T.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stoich(mut v_fuel: Py) -> R {
    let mut v__sp: Py = Py::None;
    let mut v_o2: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let t118 = (py::getitem(&c_FUELS()?, &v_fuel.clone())?).items()?;
    if t118.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v__sp = t118[0].clone();
    v_x = t118[1].clone();
    v_y = t118[2].clone();
    v_o2 = py::add(&v_x.clone(), &py::truediv(&v_y.clone(), &Py::Int(4i64))?)?;
    return Ok(Py::dict(vec![(Py::str("afr_stoich"), py::truediv(&py::mul(&v_o2.clone(), &c_AIR_PER_O2()?)?, &py::add(&py::mul(&v_x.clone(), &py::getitem(&c_AW()?, &Py::str("C"))?)?, &py::mul(&v_y.clone(), &py::getitem(&c_AW()?, &Py::str("H"))?)?)?)?), (Py::str("o2_moles"), v_o2.clone()), (Py::str("air_moles"), py::mul(&Py::Float(4.76), &v_o2.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_cb(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_Pr: Py = Py::None;
    let mut v_Re: Py = Py::None;
    let mut v_core: Py = Py::None;
    let t9 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("Re"))?, py::getitem(&v_i.clone(), &Py::str("Pr"))?])).items()?;
    if t9.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Re = t9[0].clone();
    v_Pr = t9[1].clone();
    v_core = py::mul(&py::truediv(&py::mul(&py::mul(&Py::Float(0.62), &py::pow(&v_Re.clone(), &Py::Float(0.5))?)?, &py::pow(&v_Pr.clone(), &py::truediv(&Py::Int(1i64), &Py::Int(3i64))?)?)?, &py::pow(&py::add(&Py::Int(1i64), &py::pow(&py::truediv(&Py::Float(0.4), &v_Pr.clone())?, &py::truediv(&Py::Int(2i64), &Py::Int(3i64))?)?)?, &Py::Float(0.25))?)?, &py::pow(&py::add(&Py::Int(1i64), &py::pow(&py::truediv(&v_Re.clone(), &Py::Int(282000i64))?, &Py::Float(0.625))?)?, &Py::Float(0.8))?)?;
    return Ok(py::truediv(&py::abs(&py::sub(&py::sub(&py::getitem(&v_o.clone(), &Py::str("Nu"))?, &Py::Float(0.3))?, &v_core.clone())?)?, &py::add(&Py::Float(0.3), &v_core.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_cc(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_s: Py = Py::None;
    v_s = py::add(&Py::Float(0.825), &py::truediv(&py::mul(&Py::Float(0.387), &py::pow(&py::getitem(&v_i.clone(), &Py::str("Ra"))?, &py::truediv(&Py::Int(1i64), &Py::Int(6i64))?)?)?, &py::pow(&py::add(&Py::Int(1i64), &py::pow(&py::truediv(&Py::Float(0.492), &py::getitem(&v_i.clone(), &Py::str("Pr"))?)?, &py::truediv(&Py::Int(9i64), &Py::Int(16i64))?)?)?, &py::truediv(&Py::Int(8i64), &Py::Int(27i64))?)?)?)?;
    return Ok((if ({ let t16 = py::getitem(&v_o.clone(), &Py::str("Nu"))?; Py::Bool({ let t17 = Py::Int(0i64); if py::gt(&t16, &t17)? { true } else { false } }) }).truthy() { py::truediv(&py::abs(&py::sub(&py::math1("sqrt", &py::getitem(&v_o.clone(), &Py::str("Nu"))?)?, &v_s.clone())?)?, &v_s.clone())? } else { Py::Float(f64::INFINITY) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_colebrook(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_f: Py = Py::None;
    let mut v_lhs: Py = Py::None;
    v_f = py::getitem(&v_o.clone(), &Py::str("f"))?;
    if (py::not(&{ let t30 = Py::Bool(matches!(v_f.clone(), Py::Float(_))); if !t30.truthy() { t30 } else { { let t28 = v_f.clone(); Py::Bool({ let t29 = Py::Int(0i64); if py::gt(&t28, &t29)? { true } else { false } }) } } })).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_lhs = py::truediv(&Py::Int(1i64), &py::math1("sqrt", &v_f.clone())?)?;
    return Ok(py::truediv(&py::abs(&py::add(&v_lhs.clone(), &py::mul(&Py::Int(2i64), &py::math1("log10", &py::add(&py::truediv(&py::getitem(&v_i.clone(), &Py::str("eD"))?, &Py::Float(3.7))?, &py::truediv(&Py::Float(2.51), &py::mul(&py::getitem(&v_i.clone(), &Py::str("Re"))?, &py::math1("sqrt", &v_f.clone())?)?)?)?)?)?)?)?, &v_lhs.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_dew(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_pv: Py = Py::None;
    v_pv = py::mul(&py::getitem(&v_i.clone(), &Py::str("RH"))?, &f_pws(py::getitem(&v_i.clone(), &Py::str("T"))?)?)?;
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&f_pws(py::getitem(&v_o.clone(), &Py::str("T_dew"))?)?, &v_pv.clone())?)?, &v_pv.clone())?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("p_v"))?, &v_pv.clone())?)?, &v_pv.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_equiv(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_s: Py = Py::None;
    v_s = py::getitem(&f_stoich(py::getitem(&v_i.clone(), &Py::str("fuel"))?)?, &Py::str("afr_stoich"))?;
    return Ok(py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("phi"))?, &py::getitem(&v_i.clone(), &Py::str("afr"))?)?, &v_s.clone())?)?, &v_s.clone())?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("afr_stoich"))?, &v_s.clone())?)?, &v_s.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::add(&py::getitem(&v_o.clone(), &Py::str("excess_air"))?, &Py::Int(1i64))?, &v_s.clone())?, &py::getitem(&v_i.clone(), &Py::str("afr"))?)?)?, &py::getitem(&v_i.clone(), &Py::str("afr"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_fin(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_mL: Py = Py::None;
    v_mL = py::mul(&py::math1("sqrt", &py::truediv(&py::mul(&Py::Int(4i64), &py::getitem(&v_i.clone(), &Py::str("h"))?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("k"))?, &py::getitem(&v_i.clone(), &Py::str("D"))?)?)?)?, &py::getitem(&v_i.clone(), &Py::str("L"))?)?;
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("efficiency"))?, &v_mL.clone())?, &py::math1("tanh", &v_mL.clone())?)?)?, &py::math1("tanh", &v_mL.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("mL"))?, &v_mL.clone())?)?, &v_mL.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_gn(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_Pr: Py = Py::None;
    let mut v_Re: Py = Py::None;
    let mut v_f: Py = Py::None;
    let t65 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("Re"))?, py::getitem(&v_i.clone(), &Py::str("Pr"))?])).items()?;
    if t65.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_f = t65[0].clone();
    v_Re = t65[1].clone();
    v_Pr = t65[2].clone();
    return Ok(py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("Nu"))?, &py::add(&Py::Int(1i64), &py::mul(&py::mul(&Py::Float(12.7), &py::math1("sqrt", &py::truediv(&v_f.clone(), &Py::Int(8i64))?)?)?, &py::sub(&py::pow(&v_Pr.clone(), &py::truediv(&Py::Int(2i64), &Py::Int(3i64))?)?, &Py::Int(1i64))?)?)?)?, &py::mul(&py::mul(&py::truediv(&v_f.clone(), &Py::Int(8i64))?, &py::sub(&v_Re.clone(), &Py::Int(1000i64))?)?, &v_Pr.clone())?)?)?, &py::mul(&py::mul(&py::truediv(&v_f.clone(), &Py::Int(8i64))?, &py::sub(&v_Re.clone(), &Py::Int(1000i64))?)?, &v_Pr.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_hum(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_pv: Py = Py::None;
    v_pv = py::mul(&py::getitem(&v_i.clone(), &Py::str("RH"))?, &f_pws(py::getitem(&v_i.clone(), &Py::str("T"))?)?)?;
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("W"))?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("P"))?, &v_pv.clone())?)?, &py::mul(&Py::Float(0.621945), &v_pv.clone())?)?)?, &py::mul(&Py::Float(0.621945), &v_pv.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("p_v"))?, &v_pv.clone())?)?, &v_pv.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_hx(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_Cmax: Py = Py::None;
    let mut v_Cmin: Py = Py::None;
    let mut v_Qmax: Py = Py::None;
    let mut v_eps: Py = Py::None;
    let t86 = (Py::tuple(vec![py::minmax(&[py::getitem(&v_i.clone(), &Py::str("C_hot"))?, py::getitem(&v_i.clone(), &Py::str("C_cold"))?], false)?, py::minmax(&[py::getitem(&v_i.clone(), &Py::str("C_hot"))?, py::getitem(&v_i.clone(), &Py::str("C_cold"))?], true)?])).items()?;
    if t86.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Cmin = t86[0].clone();
    v_Cmax = t86[1].clone();
    v_eps = f_eps_ntu(py::truediv(&py::getitem(&v_i.clone(), &Py::str("UA"))?, &v_Cmin.clone())?, py::truediv(&v_Cmin.clone(), &v_Cmax.clone())?, py::getitem(&v_i.clone(), &Py::str("arrangement"))?)?;
    v_Qmax = py::mul(&v_Cmin.clone(), &py::sub(&py::getitem(&v_i.clone(), &Py::str("Th_in"))?, &py::getitem(&v_i.clone(), &Py::str("Tc_in"))?)?)?;
    return Ok(py::add(&py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("effectiveness"))?, &v_eps.clone())?)?, &v_eps.clone())?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("Q"))?, &py::mul(&v_eps.clone(), &v_Qmax.clone())?)?)?, &v_Qmax.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_i.clone(), &Py::str("C_hot"))?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("Th_in"))?, &py::getitem(&v_o.clone(), &Py::str("Th_out"))?)?)?, &py::getitem(&v_o.clone(), &Py::str("Q"))?)?)?, &v_Qmax.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_i.clone(), &Py::str("C_cold"))?, &py::sub(&py::getitem(&v_o.clone(), &Py::str("Tc_out"))?, &py::getitem(&v_i.clone(), &Py::str("Tc_in"))?)?)?, &py::getitem(&v_o.clone(), &Py::str("Q"))?)?)?, &v_Qmax.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("NTU"))?, &py::truediv(&py::getitem(&v_i.clone(), &Py::str("UA"))?, &v_Cmin.clone())?)?)?, &py::getitem(&v_o.clone(), &Py::str("NTU"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_lmtd(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_d1: Py = Py::None;
    let mut v_d2: Py = Py::None;
    let mut v_e1: Py = Py::None;
    let mut v_e2: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t103 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("dT1"))?, py::getitem(&v_o.clone(), &Py::str("dT2"))?, py::getitem(&v_o.clone(), &Py::str("lmtd"))?])).items()?;
    if t103.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_d1 = t103[0].clone();
    v_d2 = t103[1].clone();
    v_L = t103[2].clone();
    if ({ let t104 = py::getitem(&v_i.clone(), &Py::str("arrangement"))?; Py::Bool({ let t105 = Py::str("counterflow"); if py::eq(&t104, &t105) { true } else { false } }) }).truthy() {
        let t106 = (Py::tuple(vec![py::sub(&py::getitem(&v_i.clone(), &Py::str("Th_in"))?, &py::getitem(&v_i.clone(), &Py::str("Tc_out"))?)?, py::sub(&py::getitem(&v_i.clone(), &Py::str("Th_out"))?, &py::getitem(&v_i.clone(), &Py::str("Tc_in"))?)?])).items()?;
        if t106.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_e1 = t106[0].clone();
        v_e2 = t106[1].clone();
    } else {
        let t107 = (Py::tuple(vec![py::sub(&py::getitem(&v_i.clone(), &Py::str("Th_in"))?, &py::getitem(&v_i.clone(), &Py::str("Tc_in"))?)?, py::sub(&py::getitem(&v_i.clone(), &Py::str("Th_out"))?, &py::getitem(&v_i.clone(), &Py::str("Tc_out"))?)?])).items()?;
        if t107.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_e1 = t107[0].clone();
        v_e2 = t107[1].clone();
    }
    v_r = py::add(&py::truediv(&py::abs(&py::sub(&v_d1.clone(), &v_e1.clone())?)?, &py::abs(&v_e1.clone())?)?, &py::truediv(&py::abs(&py::sub(&v_d2.clone(), &v_e2.clone())?)?, &py::abs(&v_e2.clone())?)?)?;
    if ({ let t108 = py::abs(&py::sub(&v_d1.clone(), &v_d2.clone())?)?; Py::Bool({ let t109 = py::mul(&Py::Float(1e-12), &py::minmax(&[v_d1.clone(), v_d2.clone()], true)?)?; if py::le(&t108, &t109)? { true } else { false } }) }).truthy() {
        return Ok(py::add(&v_r.clone(), &py::truediv(&py::abs(&py::sub(&v_L.clone(), &v_d1.clone())?)?, &v_d1.clone())?)?);
    }
    return Ok(py::add(&v_r.clone(), &py::truediv(&py::abs(&py::sub(&py::mul(&v_L.clone(), &py::math1("log", &py::truediv(&v_d1.clone(), &v_d2.clone())?)?)?, &py::sub(&v_d1.clone(), &v_d2.clone())?)?)?, &py::abs(&py::sub(&v_d1.clone(), &v_d2.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_stoich(mut v_i: Py, mut v_o: Py) -> R {
    let mut v__sp: Py = Py::None;
    let mut v_mf: Py = Py::None;
    let mut v_o2: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let t119 = (py::getitem(&c_FUELS()?, &py::getitem(&v_i.clone(), &Py::str("fuel"))?)?).items()?;
    if t119.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v__sp = t119[0].clone();
    v_x = t119[1].clone();
    v_y = t119[2].clone();
    v_o2 = py::getitem(&v_o.clone(), &Py::str("o2_moles"))?;
    v_r = py::add(&py::truediv(&py::abs(&py::sub(&py::mul(&Py::Int(2i64), &v_o2.clone())?, &py::add(&py::mul(&Py::Int(2i64), &v_x.clone())?, &py::truediv(&v_y.clone(), &Py::Int(2i64))?)?)?)?, &py::mul(&Py::Int(2i64), &v_o2.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("air_moles"))?, &py::mul(&Py::Float(4.76), &v_o2.clone())?)?)?, &py::getitem(&v_o.clone(), &Py::str("air_moles"))?)?)?;
    v_mf = py::add(&py::mul(&v_x.clone(), &py::getitem(&c_AW()?, &Py::str("C"))?)?, &py::mul(&v_y.clone(), &py::getitem(&c_AW()?, &Py::str("H"))?)?)?;
    return Ok(py::add(&v_r.clone(), &py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("afr_stoich"))?, &v_mf.clone())?, &py::mul(&v_o2.clone(), &c_AIR_PER_O2()?)?)?)?, &py::mul(&v_o2.clone(), &c_AIR_PER_O2()?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_wb(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_W: Py = Py::None;
    v_W = py::getitem(&f_hum_ratio_rh(py::getitem(&v_i.clone(), &Py::str("T"))?, py::getitem(&v_i.clone(), &Py::str("RH"))?, py::getitem(&v_i.clone(), &Py::str("P"))?)?, &Py::str("W"))?;
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&f__w_from_wb(py::getitem(&v_i.clone(), &Py::str("T"))?, py::getitem(&v_o.clone(), &Py::str("T_wb"))?, py::getitem(&v_i.clone(), &Py::str("P"))?)?, &v_W.clone())?)?, &v_W.clone())?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("W"))?, &v_W.clone())?)?, &v_W.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wet_bulb(mut v_T: Py, mut v_RH: Py, mut v_P: Py) -> R {
    let mut v_Td: Py = Py::None;
    let mut v_Twb: Py = Py::None;
    let mut v_W: Py = Py::None;
    v_W = py::getitem(&f_hum_ratio_rh(v_T.clone(), v_RH.clone(), v_P.clone())?, &Py::str("W"))?;
    v_Td = py::getitem(&f_dew_point(v_T.clone(), v_RH.clone())?, &Py::str("T_dew"))?;
    v_Twb = (if ({ let t142 = v_RH.clone(); Py::Bool({ let t143 = Py::Int(1i64); if py::eq(&t142, &t143) { true } else { false } }) }).truthy() { v_T.clone() } else { f__solve_T({ let v_P = v_P.clone(); let v_RH = v_RH.clone(); let v_T = v_T.clone(); let v_Td = v_Td.clone(); let v_W = v_W.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&f__w_from_wb(v_T.clone(), v_x.clone(), v_P.clone())?, &v_W.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, py::sub(&v_Td.clone(), &Py::Float(1e-09))?, v_T.clone())? });
    return Ok(Py::dict(vec![(Py::str("T_wb"), v_Twb.clone()), (Py::str("W"), v_W.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_117(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("p_sat"))?, &f_pws(py::getitem(&v_i.clone(), &Py::str("T"))?)?)?)?, &f_pws(py::getitem(&v_i.clone(), &Py::str("T"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}
