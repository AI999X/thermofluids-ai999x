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
    "bernoulli_with_height",
    "churchill_bernstein_cylinder",
    "churchill_chu_vertical_plate",
    "colebrook_friction_factor",
    "dew_point_temperature",
    "entropy_change_reservoir",
    "entropy_generation_heat_transfer",
    "equivalence_ratio",
    "first_law_internal_energy_change",
    "gnielinski_nusselt",
    "humidity_ratio_from_relative_humidity",
    "hx_effectiveness_ntu",
    "hydraulic_press_pascal",
    "isobaric_boundary_work",
    "lmtd",
    "pin_fin_efficiency",
    "saturation_vapour_pressure",
    "stoichiometric_air_fuel_ratio",
    "wet_bulb_temperature",
    "wet_steam_mixture",
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
        "bernoulli_with_height" => {
            let r = f_bernoulli_height(match v.get("P1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_bernoulli_height(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
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
        "entropy_change_reservoir" => {
            let r = f_entropy_reservoir(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_entropy_reservoir(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "entropy_generation_heat_transfer" => {
            let r = f_entropy_heat_transfer(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_hot") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_cold") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_entropy_heat_transfer(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "equivalence_ratio" => {
            let r = f_equivalence(match v.get("fuel") { Some(x) => py::from_val(x), None => { return None } }, match v.get("afr") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_equiv(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "first_law_internal_energy_change" => {
            let r = f_first_law(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_first_law(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
        "hydraulic_press_pascal" => {
            let r = f_hydraulic_press(match v.get("F1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_hydraulic_press(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "isobaric_boundary_work" => {
            let r = f_isobaric_work(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_isobaric_work(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
            finish(r, |i: &Py, o: &Py| -> f64 { match s_182(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "stoichiometric_air_fuel_ratio" => {
            let r = f_stoich(match v.get("fuel") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_stoich(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "wet_bulb_temperature" => {
            let r = f_wet_bulb(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("RH") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_wb(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "wet_steam_mixture" => {
            let r = f_wet_steam(match v.get("vf") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("vg") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("hf") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("hfg") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("sf") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("sfg") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("x") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("v") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("h") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("s") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f_verify_wet_steam(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
    if ({ let t165 = { let t161 = v_d1.clone(); Py::Bool({ let t162 = Py::Int(0i64); if py::le(&t161, &t162)? { true } else { false } }) }; if t165.truthy() { t165 } else { { let t163 = v_d2.clone(); Py::Bool({ let t164 = Py::Int(0i64); if py::le(&t163, &t164)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("each terminal temperature difference must be positive (the streams' temperatures cross)"))));
    }
    return Ok((if ({ let t166 = py::abs(&py::sub(&v_d1.clone(), &v_d2.clone())?)?; Py::Bool({ let t167 = py::mul(&Py::Float(1e-12), &py::minmax(&[v_d1.clone(), v_d2.clone()], true)?)?; if py::le(&t166, &t167)? { true } else { false } }) }).truthy() { v_d1.clone() } else { py::truediv(&py::sub(&v_d1.clone(), &v_d2.clone())?, &py::math1("log", &py::truediv(&v_d1.clone(), &v_d2.clone())?)?)? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rh(mut v_RH: Py) -> R {
    if (py::not(&{ let t185 = Py::Int(0i64); Py::Bool({ let t186 = v_RH.clone(); if py::lt(&t185, &t186)? { { let t187 = Py::Int(1i64); if py::le(&t186, &t187)? { true } else { false } } } else { false } }) })).truthy() {
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
    for t197 in py::range_iter(&[(Py::Int(200i64)).i()?]) {
        v__ = t197;
        v_mid = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
        if ({ let t198 = v_mid.clone(); Py::Bool({ let t199 = Py::tuple(vec![v_lo.clone(), v_hi.clone()]); if py::contains(&t199, &t198)? { true } else { false } }) }).truthy() {
            break;
        }
        v_fm = py::call(&v_fun.clone(), &[v_mid.clone()])?;
        if ({ let t200 = { let t202 = v_fm.clone(); Py::Bool({ let t203 = Py::Int(0i64); if py::lt(&t202, &t203)? { true } else { false } }) }; Py::Bool({ let t201 = { let t204 = v_flo.clone(); Py::Bool({ let t205 = Py::Int(0i64); if py::lt(&t204, &t205)? { true } else { false } }) }; if py::eq(&t200, &t201) { true } else { false } }) }).truthy() {
            let t206 = (Py::tuple(vec![v_mid.clone(), v_fm.clone()])).items()?;
            if t206.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_lo = t206[0].clone();
            v_flo = t206[1].clone();
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
    let t209 = (Py::tuple(vec![py::sub(&v_T.clone(), &Py::Float(273.15))?, py::sub(&v_Twb.clone(), &Py::Float(273.15))?])).items()?;
    if t209.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_t = t209[0].clone();
    v_tw = t209[1].clone();
    v_ps = f_pws(v_Twb.clone())?;
    v_ws = py::truediv(&py::mul(&Py::Float(0.621945), &v_ps.clone())?, &py::sub(&v_P.clone(), &v_ps.clone())?)?;
    if ({ let t210 = v_tw.clone(); Py::Bool({ let t211 = Py::Int(0i64); if py::ge(&t210, &t211)? { true } else { false } }) }).truthy() {
        return Ok(py::truediv(&py::sub(&py::mul(&py::sub(&Py::Int(2501i64), &py::mul(&Py::Float(2.326), &v_tw.clone())?)?, &v_ws.clone())?, &py::mul(&Py::Float(1.006), &py::sub(&v_t.clone(), &v_tw.clone())?)?)?, &py::sub(&py::add(&Py::Int(2501i64), &py::mul(&Py::Float(1.86), &v_t.clone())?)?, &py::mul(&Py::Float(4.186), &v_tw.clone())?)?)?);
    }
    return Ok(py::truediv(&py::sub(&py::mul(&py::sub(&Py::Int(2830i64), &py::mul(&Py::Float(0.24), &v_tw.clone())?)?, &v_ws.clone())?, &py::mul(&Py::Float(1.006), &py::sub(&v_t.clone(), &v_tw.clone())?)?)?, &py::sub(&py::add(&Py::Int(2830i64), &py::mul(&Py::Float(1.86), &v_t.clone())?)?, &py::mul(&Py::Float(2.1), &v_tw.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bernoulli_height(mut v_P1: Py, mut v_v1: Py, mut v_h1: Py, mut v_v2: Py, mut v_h2: Py, mut v_rho: Py, mut v_g: Py) -> R {
    let mut v_P2: Py = Py::None;
    if (py::not(&{ let t7 = py::math1("isfinite", &v_rho.clone())?; if !t7.truthy() { t7 } else { { let t6 = { let t1 = v_rho.clone(); Py::Bool({ let t2 = Py::Int(0i64); if py::gt(&t1, &t2)? { true } else { false } }) }; if !t6.truthy() { t6 } else { { let t5 = py::math1("isfinite", &v_g.clone())?; if !t5.truthy() { t5 } else { { let t3 = v_g.clone(); Py::Bool({ let t4 = Py::Int(0i64); if py::gt(&t3, &t4)? { true } else { false } }) } } } } } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the density and g must be positive"))));
    }
    if (py::not(&{ let t10 = py::math1("isfinite", &v_P1.clone())?; if !t10.truthy() { t10 } else { { let t8 = v_P1.clone(); Py::Bool({ let t9 = Py::Int(0i64); if py::gt(&t8, &t9)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("P1 is an absolute pressure and must be positive"))));
    }
    if (py::not(&{ let t17 = py::math1("isfinite", &v_v1.clone())?; if !t17.truthy() { t17 } else { { let t16 = py::math1("isfinite", &v_v2.clone())?; if !t16.truthy() { t16 } else { { let t15 = { let t11 = v_v1.clone(); Py::Bool({ let t12 = Py::Int(0i64); if py::ge(&t11, &t12)? { true } else { false } }) }; if !t15.truthy() { t15 } else { { let t13 = v_v2.clone(); Py::Bool({ let t14 = Py::Int(0i64); if py::ge(&t13, &t14)? { true } else { false } }) } } } } } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give the speeds as magnitudes (v >= 0)"))));
    }
    if (py::not(&{ let t18 = py::math1("isfinite", &v_h1.clone())?; if !t18.truthy() { t18 } else { py::math1("isfinite", &v_h2.clone())? } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the heights must be finite numbers"))));
    }
    v_P2 = py::add(&py::add(&v_P1.clone(), &py::mul(&py::mul(&Py::Float(0.5), &v_rho.clone())?, &py::sub(&py::mul(&v_v1.clone(), &v_v1.clone())?, &py::mul(&v_v2.clone(), &v_v2.clone())?)?)?)?, &py::mul(&py::mul(&v_rho.clone(), &v_g.clone())?, &py::sub(&v_h1.clone(), &v_h2.clone())?)?)?;
    if (py::not(&{ let t19 = v_P2.clone(); Py::Bool({ let t20 = Py::Int(0i64); if py::gt(&t19, &t20)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the computed absolute pressure P2 is not positive: the flow cannot exist so (the liquid would cavitate)"))));
    }
    return Ok(Py::dict(vec![(Py::str("P2"), v_P2.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_churchill_bernstein(mut v_Re: Py, mut v_Pr: Py) -> R {
    if (py::not(&{ let t29 = { let t22 = v_Re.clone(); Py::Bool({ let t23 = Py::Int(0i64); if py::gt(&t22, &t23)? { true } else { false } }) }; if !t29.truthy() { t29 } else { { let t28 = { let t24 = v_Pr.clone(); Py::Bool({ let t25 = Py::Int(0i64); if py::gt(&t24, &t25)? { true } else { false } }) }; if !t28.truthy() { t28 } else { { let t26 = py::mul(&v_Re.clone(), &v_Pr.clone())?; Py::Bool({ let t27 = Py::Float(0.2); if py::ge(&t26, &t27)? { true } else { false } }) } } } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("Churchill-Bernstein holds for Re * Pr >= 0.2"))));
    }
    return Ok(Py::dict(vec![(Py::str("Nu"), py::add(&Py::Float(0.3), &py::mul(&py::truediv(&py::mul(&py::mul(&Py::Float(0.62), &py::pow(&v_Re.clone(), &Py::Float(0.5))?)?, &py::pow(&v_Pr.clone(), &py::truediv(&Py::Int(1i64), &Py::Int(3i64))?)?)?, &py::pow(&py::add(&Py::Int(1i64), &py::pow(&py::truediv(&Py::Float(0.4), &v_Pr.clone())?, &py::truediv(&Py::Int(2i64), &Py::Int(3i64))?)?)?, &Py::Float(0.25))?)?, &py::pow(&py::add(&Py::Int(1i64), &py::pow(&py::truediv(&v_Re.clone(), &Py::Int(282000i64))?, &Py::Float(0.625))?)?, &Py::Float(0.8))?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_churchill_chu(mut v_Ra: Py, mut v_Pr: Py) -> R {
    let mut v_s: Py = Py::None;
    if (py::not(&{ let t36 = { let t31 = Py::Int(0i64); Py::Bool({ let t32 = v_Ra.clone(); if py::lt(&t31, &t32)? { { let t33 = Py::Float(10000000000000.0); if py::le(&t32, &t33)? { true } else { false } } } else { false } }) }; if !t36.truthy() { t36 } else { { let t34 = v_Pr.clone(); Py::Bool({ let t35 = Py::Int(0i64); if py::gt(&t34, &t35)? { true } else { false } }) } } })).truthy() {
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
    if (py::not(&{ let t41 = py::math1("isfinite", &v_Re.clone())?; if !t41.truthy() { t41 } else { { let t39 = v_Re.clone(); Py::Bool({ let t40 = Py::Int(4000i64); if py::ge(&t39, &t40)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the Colebrook equation is for turbulent flow: Re must be at least 4000"))));
    }
    if (py::not(&{ let t42 = Py::Int(0i64); Py::Bool({ let t43 = v_eD.clone(); if py::le(&t42, &t43)? { { let t44 = Py::Float(0.05); if py::le(&t43, &t44)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("relative roughness eD must lie between 0 and 0.05"))));
    }
    v_x = py::truediv(&Py::Int(1i64), &py::math1("sqrt", &py::truediv(&Py::Float(0.25), &py::pow(&py::math1("log10", &py::add(&py::truediv(&v_eD.clone(), &Py::Float(3.7))?, &py::truediv(&Py::Float(5.74), &py::pow(&v_Re.clone(), &Py::Float(0.9))?)?)?)?, &Py::Int(2i64))?)?)?)?;
    for t45 in py::range_iter(&[(Py::Int(100i64)).i()?]) {
        v__ = t45;
        let t46 = (Py::tuple(vec![py::truediv(&v_eD.clone(), &Py::Float(3.7))?, py::truediv(&Py::Float(2.51), &v_Re.clone())?])).items()?;
        if t46.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t46[0].clone();
        v_b = t46[1].clone();
        v_g = py::add(&v_x.clone(), &py::mul(&Py::Int(2i64), &py::math1("log10", &py::add(&v_a.clone(), &py::mul(&v_b.clone(), &v_x.clone())?)?)?)?)?;
        v_dg = py::add(&Py::Int(1i64), &py::truediv(&py::mul(&Py::Int(2i64), &v_b.clone())?, &py::mul(&py::add(&v_a.clone(), &py::mul(&v_b.clone(), &v_x.clone())?)?, &py::math1("log", &Py::Int(10i64))?)?)?)?;
        v_step = py::truediv(&v_g.clone(), &v_dg.clone())?;
        v_x = py::sub(&v_x, &v_step.clone())?;
        if ({ let t47 = py::abs(&v_step.clone())?; Py::Bool({ let t48 = py::mul(&Py::Float(1e-15), &py::abs(&v_x.clone())?)?; if py::le(&t47, &t48)? { true } else { false } }) }).truthy() {
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
    v_Td = (if ({ let t195 = v_RH.clone(); Py::Bool({ let t196 = Py::Int(1i64); if py::eq(&t195, &t196) { true } else { false } }) }).truthy() { v_T.clone() } else { f__solve_T({ let v_RH = v_RH.clone(); let v_T = v_T.clone(); let v_pv = v_pv.clone(); py::func(move |a: &[Py]| -> R {
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
fn f_entropy_heat_transfer(mut v_Q: Py, mut v_T_hot: Py, mut v_T_cold: Py) -> R {
    if (py::not(&{ let t84 = py::math1("isfinite", &v_T_hot.clone())?; if !t84.truthy() { t84 } else { { let t83 = py::math1("isfinite", &v_T_cold.clone())?; if !t83.truthy() { t83 } else { { let t82 = { let t78 = v_T_hot.clone(); Py::Bool({ let t79 = Py::Int(0i64); if py::gt(&t78, &t79)? { true } else { false } }) }; if !t82.truthy() { t82 } else { { let t80 = v_T_cold.clone(); Py::Bool({ let t81 = Py::Int(0i64); if py::gt(&t80, &t81)? { true } else { false } }) } } } } } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the absolute temperatures must be positive"))));
    }
    if (py::not(&{ let t87 = py::math1("isfinite", &v_Q.clone())?; if !t87.truthy() { t87 } else { { let t85 = v_Q.clone(); Py::Bool({ let t86 = Py::Int(0i64); if py::gt(&t85, &t86)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("Q is the heat that flows from the hot to the cold reservoir and must be positive"))));
    }
    if ({ let t88 = v_T_hot.clone(); Py::Bool({ let t89 = v_T_cold.clone(); if py::lt(&t88, &t89)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("heat cannot flow by itself from the colder to the hotter reservoir (the total entropy would fall)"))));
    }
    return Ok(Py::dict(vec![(Py::str("dS_total"), py::truediv(&py::mul(&v_Q.clone(), &py::sub(&v_T_hot.clone(), &v_T_cold.clone())?)?, &py::mul(&v_T_hot.clone(), &v_T_cold.clone())?)?), (Py::str("dS_hot"), py::truediv(&py::neg(&v_Q.clone())?, &v_T_hot.clone())?), (Py::str("dS_cold"), py::truediv(&v_Q.clone(), &v_T_cold.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_entropy_reservoir(mut v_Q: Py, mut v_T: Py) -> R {
    if (py::not(&{ let t74 = py::math1("isfinite", &v_T.clone())?; if !t74.truthy() { t74 } else { { let t72 = v_T.clone(); Py::Bool({ let t73 = Py::Int(0i64); if py::gt(&t72, &t73)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the absolute temperature must be positive"))));
    }
    if (py::not(&py::math1("isfinite", &v_Q.clone())?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the heat must be a finite number"))));
    }
    return Ok(Py::dict(vec![(Py::str("dS"), py::truediv(&v_Q.clone(), &v_T.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_eps_ntu(mut v_NTU: Py, mut v_Cr: Py, mut v_arrangement: Py) -> R {
    let mut v_e: Py = Py::None;
    if ({ let t125 = v_arrangement.clone(); Py::Bool({ let t126 = Py::str("counterflow"); if py::eq(&t125, &t126) { true } else { false } }) }).truthy() {
        if ({ let t127 = py::abs(&py::sub(&Py::Int(1i64), &v_Cr.clone())?)?; Py::Bool({ let t128 = Py::Float(1e-12); if py::lt(&t127, &t128)? { true } else { false } }) }).truthy() {
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
    if (py::not(&{ let t91 = v_afr.clone(); Py::Bool({ let t92 = Py::Int(0i64); if py::gt(&t91, &t92)? { true } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the air-fuel ratio must be positive"))));
    }
    v_s = py::getitem(&f_stoich(v_fuel.clone())?, &Py::str("afr_stoich"))?;
    return Ok(Py::dict(vec![(Py::str("phi"), py::truediv(&v_s.clone(), &v_afr.clone())?), (Py::str("afr_stoich"), v_s.clone()), (Py::str("excess_air"), py::sub(&py::truediv(&v_afr.clone(), &v_s.clone())?, &Py::Int(1i64))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_first_law(mut v_Q: Py, mut v_W: Py) -> R {
    if (py::not(&{ let t94 = py::math1("isfinite", &v_Q.clone())?; if !t94.truthy() { t94 } else { py::math1("isfinite", &v_W.clone())? } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the heat and the work must be finite numbers"))));
    }
    return Ok(Py::dict(vec![(Py::str("dU"), py::sub(&v_Q.clone(), &v_W.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gnielinski(mut v_Re: Py, mut v_Pr: Py, mut v_f: Py) -> R {
    if (py::not(&{ let t107 = { let t97 = Py::Int(3000i64); Py::Bool({ let t98 = v_Re.clone(); if py::le(&t97, &t98)? { { let t99 = Py::Float(5000000.0); if py::le(&t98, &t99)? { true } else { false } } } else { false } }) }; if !t107.truthy() { t107 } else { { let t106 = { let t100 = Py::Float(0.5); Py::Bool({ let t101 = v_Pr.clone(); if py::le(&t100, &t101)? { { let t102 = Py::Int(2000i64); if py::le(&t101, &t102)? { true } else { false } } } else { false } }) }; if !t106.truthy() { t106 } else { { let t103 = Py::Int(0i64); Py::Bool({ let t104 = v_f.clone(); if py::lt(&t103, &t104)? { { let t105 = Py::Float(0.2); if py::lt(&t104, &t105)? { true } else { false } } } else { false } }) } } } } })).truthy() {
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
    if (py::not(&{ let t193 = v_P.clone(); Py::Bool({ let t194 = v_pv.clone(); if py::gt(&t193, &t194)? { true } else { false } }) })).truthy() {
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
    if ({ let t119 = py::minmax(&[v_C_hot.clone(), v_C_cold.clone(), v_UA.clone()], false)?; Py::Bool({ let t120 = Py::Int(0i64); if py::le(&t119, &t120)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("heat capacity rates and UA must be positive"))));
    }
    if ({ let t121 = v_Th_in.clone(); Py::Bool({ let t122 = v_Tc_in.clone(); if py::le(&t121, &t122)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the hot inlet must be hotter than the cold inlet"))));
    }
    let t123 = (Py::tuple(vec![py::minmax(&[v_C_hot.clone(), v_C_cold.clone()], false)?, py::minmax(&[v_C_hot.clone(), v_C_cold.clone()], true)?])).items()?;
    if t123.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Cmin = t123[0].clone();
    v_Cmax = t123[1].clone();
    let t124 = (Py::tuple(vec![py::truediv(&v_UA.clone(), &v_Cmin.clone())?, py::truediv(&v_Cmin.clone(), &v_Cmax.clone())?])).items()?;
    if t124.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_NTU = t124[0].clone();
    v_Cr = t124[1].clone();
    v_eps = f_eps_ntu(v_NTU.clone(), v_Cr.clone(), v_arrangement.clone())?;
    v_Q = py::mul(&py::mul(&v_eps.clone(), &v_Cmin.clone())?, &py::sub(&v_Th_in.clone(), &v_Tc_in.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("Q"), v_Q.clone()), (Py::str("Th_out"), py::sub(&v_Th_in.clone(), &py::truediv(&v_Q.clone(), &v_C_hot.clone())?)?), (Py::str("Tc_out"), py::add(&v_Tc_in.clone(), &py::truediv(&v_Q.clone(), &v_C_cold.clone())?)?), (Py::str("effectiveness"), v_eps.clone()), (Py::str("NTU"), v_NTU.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hydraulic_press(mut v_F1: Py, mut v_A1: Py, mut v_A2: Py) -> R {
    let mut v_P: Py = Py::None;
    if (py::not(&{ let t132 = py::math1("isfinite", &v_F1.clone())?; if !t132.truthy() { t132 } else { { let t130 = v_F1.clone(); Py::Bool({ let t131 = Py::Int(0i64); if py::gt(&t130, &t131)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the input force must be positive"))));
    }
    if (py::not(&{ let t139 = py::math1("isfinite", &v_A1.clone())?; if !t139.truthy() { t139 } else { { let t138 = py::math1("isfinite", &v_A2.clone())?; if !t138.truthy() { t138 } else { { let t137 = { let t133 = v_A1.clone(); Py::Bool({ let t134 = Py::Int(0i64); if py::gt(&t133, &t134)? { true } else { false } }) }; if !t137.truthy() { t137 } else { { let t135 = v_A2.clone(); Py::Bool({ let t136 = Py::Int(0i64); if py::gt(&t135, &t136)? { true } else { false } }) } } } } } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the piston areas must be positive"))));
    }
    v_P = py::truediv(&v_F1.clone(), &v_A1.clone())?;
    return Ok(Py::dict(vec![(Py::str("F2"), py::mul(&v_P.clone(), &v_A2.clone())?), (Py::str("P"), v_P.clone()), (Py::str("MA"), py::truediv(&v_A2.clone(), &v_A1.clone())?), (Py::str("d_ratio"), py::truediv(&v_A1.clone(), &v_A2.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_isobaric_work(mut v_P: Py, mut v_V1: Py, mut v_V2: Py) -> R {
    if (py::not(&{ let t143 = py::math1("isfinite", &v_P.clone())?; if !t143.truthy() { t143 } else { { let t141 = v_P.clone(); Py::Bool({ let t142 = Py::Int(0i64); if py::gt(&t141, &t142)? { true } else { false } }) } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the pressure must be positive (an absolute pressure)"))));
    }
    if (py::not(&{ let t150 = py::math1("isfinite", &v_V1.clone())?; if !t150.truthy() { t150 } else { { let t149 = py::math1("isfinite", &v_V2.clone())?; if !t149.truthy() { t149 } else { { let t148 = { let t144 = v_V1.clone(); Py::Bool({ let t145 = Py::Int(0i64); if py::gt(&t144, &t145)? { true } else { false } }) }; if !t148.truthy() { t148 } else { { let t146 = v_V2.clone(); Py::Bool({ let t147 = Py::Int(0i64); if py::gt(&t146, &t147)? { true } else { false } }) } } } } } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the initial and final volumes must be positive"))));
    }
    return Ok(Py::dict(vec![(Py::str("W"), py::mul(&v_P.clone(), &py::sub(&v_V2.clone(), &v_V1.clone())?)?), (Py::str("dV"), py::sub(&v_V2.clone(), &v_V1.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lmtd(mut v_Th_in: Py, mut v_Th_out: Py, mut v_Tc_in: Py, mut v_Tc_out: Py, mut v_arrangement: Py) -> R {
    let mut v_d1: Py = Py::None;
    let mut v_d2: Py = Py::None;
    if ({ let t156 = { let t152 = v_Th_out.clone(); Py::Bool({ let t153 = v_Th_in.clone(); if py::gt(&t152, &t153)? { true } else { false } }) }; if t156.truthy() { t156 } else { { let t154 = v_Tc_out.clone(); Py::Bool({ let t155 = v_Tc_in.clone(); if py::lt(&t154, &t155)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the hot stream must cool and the cold stream must warm"))));
    }
    if ({ let t157 = v_arrangement.clone(); Py::Bool({ let t158 = Py::str("counterflow"); if py::eq(&t157, &t158) { true } else { false } }) }).truthy() {
        let t159 = (Py::tuple(vec![py::sub(&v_Th_in.clone(), &v_Tc_out.clone())?, py::sub(&v_Th_out.clone(), &v_Tc_in.clone())?])).items()?;
        if t159.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_d1 = t159[0].clone();
        v_d2 = t159[1].clone();
    } else {
        let t160 = (Py::tuple(vec![py::sub(&v_Th_in.clone(), &v_Tc_in.clone())?, py::sub(&v_Th_out.clone(), &v_Tc_out.clone())?])).items()?;
        if t160.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_d1 = t160[0].clone();
        v_d2 = t160[1].clone();
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
    if ({ let t175 = py::minmax(&[v_h.clone(), v_k.clone(), v_D.clone(), v_L.clone()], false)?; Py::Bool({ let t176 = Py::Int(0i64); if py::le(&t175, &t176)? { true } else { false } }) }).truthy() {
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
    if (py::not(&{ let t188 = py::neg(&Py::Int(100i64))?; Py::Bool({ let t189 = v_t.clone(); if py::le(&t188, &t189)? { { let t190 = Py::Int(200i64); if py::le(&t189, &t190)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("temperature must lie between -100 °C and 200 °C"))));
    }
    if ({ let t191 = v_t.clone(); Py::Bool({ let t192 = Py::Float(0.01); if py::le(&t191, &t192)? { true } else { false } }) }).truthy() {
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
    let t183 = (py::getitem(&c_FUELS()?, &v_fuel.clone())?).items()?;
    if t183.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v__sp = t183[0].clone();
    v_x = t183[1].clone();
    v_y = t183[2].clone();
    v_o2 = py::add(&v_x.clone(), &py::truediv(&v_y.clone(), &Py::Int(4i64))?)?;
    return Ok(Py::dict(vec![(Py::str("afr_stoich"), py::truediv(&py::mul(&v_o2.clone(), &c_AIR_PER_O2()?)?, &py::add(&py::mul(&v_x.clone(), &py::getitem(&c_AW()?, &Py::str("C"))?)?, &py::mul(&v_y.clone(), &py::getitem(&c_AW()?, &Py::str("H"))?)?)?)?), (Py::str("o2_moles"), v_o2.clone()), (Py::str("air_moles"), py::mul(&Py::Float(4.76), &v_o2.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_bernoulli_height(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_lhs: Py = Py::None;
    let mut v_rho: Py = Py::None;
    let mut v_rhs: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t21 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("rho"))?, py::getitem(&v_i.clone(), &Py::str("g"))?])).items()?;
    if t21.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rho = t21[0].clone();
    v_g = t21[1].clone();
    v_lhs = py::add(&py::add(&py::getitem(&v_o.clone(), &Py::str("P2"))?, &py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_rho.clone())?, &py::getitem(&v_i.clone(), &Py::str("v2"))?)?, &py::getitem(&v_i.clone(), &Py::str("v2"))?)?)?, &py::mul(&py::mul(&v_rho.clone(), &v_g.clone())?, &py::getitem(&v_i.clone(), &Py::str("h2"))?)?)?;
    v_rhs = py::add(&py::add(&py::getitem(&v_i.clone(), &Py::str("P1"))?, &py::mul(&py::mul(&py::mul(&Py::Float(0.5), &v_rho.clone())?, &py::getitem(&v_i.clone(), &Py::str("v1"))?)?, &py::getitem(&v_i.clone(), &Py::str("v1"))?)?)?, &py::mul(&py::mul(&v_rho.clone(), &v_g.clone())?, &py::getitem(&v_i.clone(), &Py::str("h1"))?)?)?;
    v_s = py::add(&py::add(&py::getitem(&v_i.clone(), &Py::str("P1"))?, &py::mul(&py::mul(&Py::Float(0.5), &v_rho.clone())?, &py::add(&py::mul(&py::getitem(&v_i.clone(), &Py::str("v1"))?, &py::getitem(&v_i.clone(), &Py::str("v1"))?)?, &py::mul(&py::getitem(&v_i.clone(), &Py::str("v2"))?, &py::getitem(&v_i.clone(), &Py::str("v2"))?)?)?)?)?, &py::mul(&py::mul(&v_rho.clone(), &v_g.clone())?, &py::add(&py::abs(&py::getitem(&v_i.clone(), &Py::str("h1"))?)?, &py::abs(&py::getitem(&v_i.clone(), &Py::str("h2"))?)?)?)?)?;
    return Ok(py::truediv(&py::abs(&py::sub(&v_lhs.clone(), &v_rhs.clone())?)?, &v_s.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_cb(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_Pr: Py = Py::None;
    let mut v_Re: Py = Py::None;
    let mut v_core: Py = Py::None;
    let t30 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("Re"))?, py::getitem(&v_i.clone(), &Py::str("Pr"))?])).items()?;
    if t30.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Re = t30[0].clone();
    v_Pr = t30[1].clone();
    v_core = py::mul(&py::truediv(&py::mul(&py::mul(&Py::Float(0.62), &py::pow(&v_Re.clone(), &Py::Float(0.5))?)?, &py::pow(&v_Pr.clone(), &py::truediv(&Py::Int(1i64), &Py::Int(3i64))?)?)?, &py::pow(&py::add(&Py::Int(1i64), &py::pow(&py::truediv(&Py::Float(0.4), &v_Pr.clone())?, &py::truediv(&Py::Int(2i64), &Py::Int(3i64))?)?)?, &Py::Float(0.25))?)?, &py::pow(&py::add(&Py::Int(1i64), &py::pow(&py::truediv(&v_Re.clone(), &Py::Int(282000i64))?, &Py::Float(0.625))?)?, &Py::Float(0.8))?)?;
    return Ok(py::truediv(&py::abs(&py::sub(&py::sub(&py::getitem(&v_o.clone(), &Py::str("Nu"))?, &Py::Float(0.3))?, &v_core.clone())?)?, &py::add(&Py::Float(0.3), &v_core.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_cc(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_s: Py = Py::None;
    v_s = py::add(&Py::Float(0.825), &py::truediv(&py::mul(&Py::Float(0.387), &py::pow(&py::getitem(&v_i.clone(), &Py::str("Ra"))?, &py::truediv(&Py::Int(1i64), &Py::Int(6i64))?)?)?, &py::pow(&py::add(&Py::Int(1i64), &py::pow(&py::truediv(&Py::Float(0.492), &py::getitem(&v_i.clone(), &Py::str("Pr"))?)?, &py::truediv(&Py::Int(9i64), &Py::Int(16i64))?)?)?, &py::truediv(&Py::Int(8i64), &Py::Int(27i64))?)?)?)?;
    return Ok((if ({ let t37 = py::getitem(&v_o.clone(), &Py::str("Nu"))?; Py::Bool({ let t38 = Py::Int(0i64); if py::gt(&t37, &t38)? { true } else { false } }) }).truthy() { py::truediv(&py::abs(&py::sub(&py::math1("sqrt", &py::getitem(&v_o.clone(), &Py::str("Nu"))?)?, &v_s.clone())?)?, &v_s.clone())? } else { Py::Float(f64::INFINITY) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_colebrook(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_f: Py = Py::None;
    let mut v_lhs: Py = Py::None;
    v_f = py::getitem(&v_o.clone(), &Py::str("f"))?;
    if (py::not(&{ let t51 = Py::Bool(matches!(v_f.clone(), Py::Float(_))); if !t51.truthy() { t51 } else { { let t49 = v_f.clone(); Py::Bool({ let t50 = Py::Int(0i64); if py::gt(&t49, &t50)? { true } else { false } }) } } })).truthy() {
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
fn f_verify_entropy_heat_transfer(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_Q: Py = Py::None;
    let mut v_Tc: Py = Py::None;
    let mut v_Th: Py = Py::None;
    let t90 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("Q"))?, py::getitem(&v_i.clone(), &Py::str("T_hot"))?, py::getitem(&v_i.clone(), &Py::str("T_cold"))?])).items()?;
    if t90.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Q = t90[0].clone();
    v_Th = t90[1].clone();
    v_Tc = t90[2].clone();
    return Ok(py::add(&py::add(&py::truediv(&py::abs(&py::add(&py::mul(&py::getitem(&v_o.clone(), &Py::str("dS_hot"))?, &v_Th.clone())?, &v_Q.clone())?)?, &v_Q.clone())?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("dS_cold"))?, &v_Tc.clone())?, &v_Q.clone())?)?, &v_Q.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::mul(&py::getitem(&v_o.clone(), &Py::str("dS_total"))?, &v_Th.clone())?, &v_Tc.clone())?, &py::mul(&v_Q.clone(), &py::sub(&v_Th.clone(), &v_Tc.clone())?)?)?)?, &py::mul(&v_Q.clone(), &v_Th.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_entropy_reservoir(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_Q: Py = Py::None;
    let mut v_T: Py = Py::None;
    let t75 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("Q"))?, py::getitem(&v_i.clone(), &Py::str("T"))?])).items()?;
    if t75.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Q = t75[0].clone();
    v_T = t75[1].clone();
    if ({ let t76 = v_Q.clone(); Py::Bool({ let t77 = Py::Int(0i64); if py::eq(&t76, &t77) { true } else { false } }) }).truthy() {
        return Ok(py::abs(&py::getitem(&v_o.clone(), &Py::str("dS"))?)?);
    }
    return Ok(py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("dS"))?, &v_T.clone())?, &v_Q.clone())?)?, &py::abs(&v_Q.clone())?)?);
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
fn f_verify_first_law(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_s: Py = Py::None;
    v_s = py::minmax(&[py::abs(&py::getitem(&v_i.clone(), &Py::str("Q"))?)?, py::abs(&py::getitem(&v_i.clone(), &Py::str("W"))?)?], true)?;
    if ({ let t95 = v_s.clone(); Py::Bool({ let t96 = Py::Int(0i64); if py::eq(&t95, &t96) { true } else { false } }) }).truthy() {
        return Ok(py::abs(&py::getitem(&v_o.clone(), &Py::str("dU"))?)?);
    }
    return Ok(py::truediv(&py::abs(&py::sub(&py::add(&py::getitem(&v_o.clone(), &Py::str("dU"))?, &py::getitem(&v_i.clone(), &Py::str("W"))?)?, &py::getitem(&v_i.clone(), &Py::str("Q"))?)?)?, &v_s.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_gn(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_Pr: Py = Py::None;
    let mut v_Re: Py = Py::None;
    let mut v_f: Py = Py::None;
    let t108 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("f"))?, py::getitem(&v_i.clone(), &Py::str("Re"))?, py::getitem(&v_i.clone(), &Py::str("Pr"))?])).items()?;
    if t108.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_f = t108[0].clone();
    v_Re = t108[1].clone();
    v_Pr = t108[2].clone();
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
    let t129 = (Py::tuple(vec![py::minmax(&[py::getitem(&v_i.clone(), &Py::str("C_hot"))?, py::getitem(&v_i.clone(), &Py::str("C_cold"))?], false)?, py::minmax(&[py::getitem(&v_i.clone(), &Py::str("C_hot"))?, py::getitem(&v_i.clone(), &Py::str("C_cold"))?], true)?])).items()?;
    if t129.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Cmin = t129[0].clone();
    v_Cmax = t129[1].clone();
    v_eps = f_eps_ntu(py::truediv(&py::getitem(&v_i.clone(), &Py::str("UA"))?, &v_Cmin.clone())?, py::truediv(&v_Cmin.clone(), &v_Cmax.clone())?, py::getitem(&v_i.clone(), &Py::str("arrangement"))?)?;
    v_Qmax = py::mul(&v_Cmin.clone(), &py::sub(&py::getitem(&v_i.clone(), &Py::str("Th_in"))?, &py::getitem(&v_i.clone(), &Py::str("Tc_in"))?)?)?;
    return Ok(py::add(&py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("effectiveness"))?, &v_eps.clone())?)?, &v_eps.clone())?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("Q"))?, &py::mul(&v_eps.clone(), &v_Qmax.clone())?)?)?, &v_Qmax.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_i.clone(), &Py::str("C_hot"))?, &py::sub(&py::getitem(&v_i.clone(), &Py::str("Th_in"))?, &py::getitem(&v_o.clone(), &Py::str("Th_out"))?)?)?, &py::getitem(&v_o.clone(), &Py::str("Q"))?)?)?, &v_Qmax.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_i.clone(), &Py::str("C_cold"))?, &py::sub(&py::getitem(&v_o.clone(), &Py::str("Tc_out"))?, &py::getitem(&v_i.clone(), &Py::str("Tc_in"))?)?)?, &py::getitem(&v_o.clone(), &Py::str("Q"))?)?)?, &v_Qmax.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("NTU"))?, &py::truediv(&py::getitem(&v_i.clone(), &Py::str("UA"))?, &v_Cmin.clone())?)?)?, &py::getitem(&v_o.clone(), &Py::str("NTU"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_hydraulic_press(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_A1: Py = Py::None;
    let mut v_A2: Py = Py::None;
    let mut v_F1: Py = Py::None;
    let t140 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("F1"))?, py::getitem(&v_i.clone(), &Py::str("A1"))?, py::getitem(&v_i.clone(), &Py::str("A2"))?])).items()?;
    if t140.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_F1 = t140[0].clone();
    v_A1 = t140[1].clone();
    v_A2 = t140[2].clone();
    return Ok(py::add(&py::add(&py::add(&py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("F2"))?, &v_A1.clone())?, &py::mul(&v_F1.clone(), &v_A2.clone())?)?)?, &py::mul(&v_F1.clone(), &v_A2.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("P"))?, &v_A1.clone())?, &v_F1.clone())?)?, &v_F1.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("MA"))?, &v_A1.clone())?, &v_A2.clone())?)?, &v_A2.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::mul(&py::getitem(&v_o.clone(), &Py::str("d_ratio"))?, &v_A2.clone())?, &v_A1.clone())?)?, &v_A1.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_verify_isobaric_work(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_V1: Py = Py::None;
    let mut v_V2: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t151 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("P"))?, py::getitem(&v_i.clone(), &Py::str("V1"))?, py::getitem(&v_i.clone(), &Py::str("V2"))?])).items()?;
    if t151.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_P = t151[0].clone();
    v_V1 = t151[1].clone();
    v_V2 = t151[2].clone();
    v_s = py::minmax(&[v_V1.clone(), v_V2.clone()], true)?;
    return Ok(py::add(&py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("W"))?, &py::mul(&v_P.clone(), &py::sub(&v_V2.clone(), &v_V1.clone())?)?)?)?, &py::mul(&v_P.clone(), &v_s.clone())?)?, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("dV"))?, &py::sub(&v_V2.clone(), &v_V1.clone())?)?)?, &v_s.clone())?)?);
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
    let t168 = (Py::tuple(vec![py::getitem(&v_o.clone(), &Py::str("dT1"))?, py::getitem(&v_o.clone(), &Py::str("dT2"))?, py::getitem(&v_o.clone(), &Py::str("lmtd"))?])).items()?;
    if t168.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_d1 = t168[0].clone();
    v_d2 = t168[1].clone();
    v_L = t168[2].clone();
    if ({ let t169 = py::getitem(&v_i.clone(), &Py::str("arrangement"))?; Py::Bool({ let t170 = Py::str("counterflow"); if py::eq(&t169, &t170) { true } else { false } }) }).truthy() {
        let t171 = (Py::tuple(vec![py::sub(&py::getitem(&v_i.clone(), &Py::str("Th_in"))?, &py::getitem(&v_i.clone(), &Py::str("Tc_out"))?)?, py::sub(&py::getitem(&v_i.clone(), &Py::str("Th_out"))?, &py::getitem(&v_i.clone(), &Py::str("Tc_in"))?)?])).items()?;
        if t171.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_e1 = t171[0].clone();
        v_e2 = t171[1].clone();
    } else {
        let t172 = (Py::tuple(vec![py::sub(&py::getitem(&v_i.clone(), &Py::str("Th_in"))?, &py::getitem(&v_i.clone(), &Py::str("Tc_in"))?)?, py::sub(&py::getitem(&v_i.clone(), &Py::str("Th_out"))?, &py::getitem(&v_i.clone(), &Py::str("Tc_out"))?)?])).items()?;
        if t172.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_e1 = t172[0].clone();
        v_e2 = t172[1].clone();
    }
    v_r = py::add(&py::truediv(&py::abs(&py::sub(&v_d1.clone(), &v_e1.clone())?)?, &py::abs(&v_e1.clone())?)?, &py::truediv(&py::abs(&py::sub(&v_d2.clone(), &v_e2.clone())?)?, &py::abs(&v_e2.clone())?)?)?;
    if ({ let t173 = py::abs(&py::sub(&v_d1.clone(), &v_d2.clone())?)?; Py::Bool({ let t174 = py::mul(&Py::Float(1e-12), &py::minmax(&[v_d1.clone(), v_d2.clone()], true)?)?; if py::le(&t173, &t174)? { true } else { false } }) }).truthy() {
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
    let t184 = (py::getitem(&c_FUELS()?, &py::getitem(&v_i.clone(), &Py::str("fuel"))?)?).items()?;
    if t184.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v__sp = t184[0].clone();
    v_x = t184[1].clone();
    v_y = t184[2].clone();
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
fn f_verify_wet_steam(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_hf: Py = Py::None;
    let mut v_hfg: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_res: Py = Py::None;
    let mut v_sc: Py = Py::None;
    let mut v_sf: Py = Py::None;
    let mut v_sfg: Py = Py::None;
    let mut v_vf: Py = Py::None;
    let mut v_vg: Py = Py::None;
    if ({ let t301 = { let t297 = Py::str("x"); Py::Bool({ let t298 = v_o.clone(); if !py::contains(&t298, &t297)? { true } else { false } }) }; if t301.truthy() { t301 } else { { let t299 = Py::str("moisture"); Py::Bool({ let t300 = v_o.clone(); if !py::contains(&t300, &t299)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_q = py::getitem(&v_o.clone(), &Py::str("x"))?;
    if (py::not(&{ let t305 = py::math1("isfinite", &v_q.clone())?; if !t305.truthy() { t305 } else { { let t302 = Py::Int(0i64); Py::Bool({ let t303 = v_q.clone(); if py::le(&t302, &t303)? { { let t304 = Py::Int(1i64); if py::le(&t303, &t304)? { true } else { false } } } else { false } }) } } })).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_res = py::abs(&py::sub(&py::add(&py::getitem(&v_o.clone(), &Py::str("moisture"))?, &v_q.clone())?, &Py::Float(1.0))?)?;
    let t306 = (Py::tuple(vec![py::method(&v_i.clone(), "get", &[Py::str("vf")])?, py::method(&v_i.clone(), "get", &[Py::str("vg")])?, py::method(&v_i.clone(), "get", &[Py::str("hf")])?, py::method(&v_i.clone(), "get", &[Py::str("hfg")])?, py::method(&v_i.clone(), "get", &[Py::str("sf")])?, py::method(&v_i.clone(), "get", &[Py::str("sfg")])?])).items()?;
    if t306.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_vf = t306[0].clone();
    v_vg = t306[1].clone();
    v_hf = t306[2].clone();
    v_hfg = t306[3].clone();
    v_sf = t306[4].clone();
    v_sfg = t306[5].clone();
    if ({ let t307 = v_vf.clone(); Py::Bool({ let t308 = Py::None; if !py::is(&t307, &t308) { true } else { false } }) }).truthy() {
        if ({ let t309 = Py::str("v"); Py::Bool({ let t310 = v_o.clone(); if !py::contains(&t310, &t309)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_res = py::iadd(&v_res, &py::truediv(&py::abs(&py::sub(&py::sub(&py::getitem(&v_o.clone(), &Py::str("v"))?, &v_vf.clone())?, &py::mul(&v_q.clone(), &py::sub(&v_vg.clone(), &v_vf.clone())?)?)?)?, &v_vg.clone())?)?;
        if ({ let t311 = py::method(&v_i.clone(), "get", &[Py::str("v")])?; Py::Bool({ let t312 = Py::None; if !py::is(&t311, &t312) { true } else { false } }) }).truthy() {
            v_res = py::iadd(&v_res, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("v"))?, &py::getitem(&v_i.clone(), &Py::str("v"))?)?)?, &v_vg.clone())?)?;
        }
    }
    if ({ let t313 = v_hf.clone(); Py::Bool({ let t314 = Py::None; if !py::is(&t313, &t314) { true } else { false } }) }).truthy() {
        if ({ let t319 = { let t315 = Py::str("h"); Py::Bool({ let t316 = v_o.clone(); if !py::contains(&t316, &t315)? { true } else { false } }) }; if t319.truthy() { t319 } else { { let t317 = Py::str("hg"); Py::Bool({ let t318 = v_o.clone(); if !py::contains(&t318, &t317)? { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_sc = py::add(&py::abs(&v_hf.clone())?, &v_hfg.clone())?;
        v_res = py::iadd(&v_res, &py::add(&py::truediv(&py::abs(&py::sub(&py::sub(&py::getitem(&v_o.clone(), &Py::str("h"))?, &v_hf.clone())?, &py::mul(&v_q.clone(), &v_hfg.clone())?)?)?, &v_sc.clone())?, &py::truediv(&py::abs(&py::sub(&py::sub(&py::getitem(&v_o.clone(), &Py::str("hg"))?, &v_hf.clone())?, &v_hfg.clone())?)?, &v_sc.clone())?)?)?;
        if ({ let t320 = py::method(&v_i.clone(), "get", &[Py::str("h")])?; Py::Bool({ let t321 = Py::None; if !py::is(&t320, &t321) { true } else { false } }) }).truthy() {
            v_res = py::iadd(&v_res, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("h"))?, &py::getitem(&v_i.clone(), &Py::str("h"))?)?)?, &v_sc.clone())?)?;
        }
    }
    if ({ let t322 = v_sf.clone(); Py::Bool({ let t323 = Py::None; if !py::is(&t322, &t323) { true } else { false } }) }).truthy() {
        if ({ let t328 = { let t324 = Py::str("s"); Py::Bool({ let t325 = v_o.clone(); if !py::contains(&t325, &t324)? { true } else { false } }) }; if t328.truthy() { t328 } else { { let t326 = Py::str("sg"); Py::Bool({ let t327 = v_o.clone(); if !py::contains(&t327, &t326)? { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_sc = py::add(&py::abs(&v_sf.clone())?, &v_sfg.clone())?;
        v_res = py::iadd(&v_res, &py::add(&py::truediv(&py::abs(&py::sub(&py::sub(&py::getitem(&v_o.clone(), &Py::str("s"))?, &v_sf.clone())?, &py::mul(&v_q.clone(), &v_sfg.clone())?)?)?, &v_sc.clone())?, &py::truediv(&py::abs(&py::sub(&py::sub(&py::getitem(&v_o.clone(), &Py::str("sg"))?, &v_sf.clone())?, &v_sfg.clone())?)?, &v_sc.clone())?)?)?;
        if ({ let t329 = py::method(&v_i.clone(), "get", &[Py::str("s")])?; Py::Bool({ let t330 = Py::None; if !py::is(&t329, &t330) { true } else { false } }) }).truthy() {
            v_res = py::iadd(&v_res, &py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("s"))?, &py::getitem(&v_i.clone(), &Py::str("s"))?)?)?, &v_sc.clone())?)?;
        }
    }
    if ({ let t331 = py::method(&v_i.clone(), "get", &[Py::str("x")])?; Py::Bool({ let t332 = Py::None; if !py::is(&t331, &t332) { true } else { false } }) }).truthy() {
        v_res = py::iadd(&v_res, &py::abs(&py::sub(&v_q.clone(), &py::getitem(&v_i.clone(), &Py::str("x"))?)?)?)?;
    }
    return Ok(v_res.clone());
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
    v_Twb = (if ({ let t207 = v_RH.clone(); Py::Bool({ let t208 = Py::Int(1i64); if py::eq(&t207, &t208) { true } else { false } }) }).truthy() { v_T.clone() } else { f__solve_T({ let v_P = v_P.clone(); let v_RH = v_RH.clone(); let v_T = v_T.clone(); let v_Td = v_Td.clone(); let v_W = v_W.clone(); py::func(move |a: &[Py]| -> R {
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
fn f_wet_steam(mut v_vf: Py, mut v_vg: Py, mut v_hf: Py, mut v_hfg: Py, mut v_sf: Py, mut v_sfg: Py, mut v_x: Py, mut v_v: Py, mut v_h: Py, mut v_s: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_val: Py = Py::None;
    if ({ let t231 = { let t212 = { let t214 = v_vf.clone(); Py::Bool({ let t215 = Py::None; if py::is(&t214, &t215) { true } else { false } }) }; Py::Bool({ let t213 = { let t216 = v_vg.clone(); Py::Bool({ let t217 = Py::None; if py::is(&t216, &t217) { true } else { false } }) }; if py::ne(&t212, &t213) { true } else { false } }) }; if t231.truthy() { t231 } else { { let t230 = { let t218 = { let t220 = v_hf.clone(); Py::Bool({ let t221 = Py::None; if py::is(&t220, &t221) { true } else { false } }) }; Py::Bool({ let t219 = { let t222 = v_hfg.clone(); Py::Bool({ let t223 = Py::None; if py::is(&t222, &t223) { true } else { false } }) }; if py::ne(&t218, &t219) { true } else { false } }) }; if t230.truthy() { t230 } else { { let t224 = { let t226 = v_sf.clone(); Py::Bool({ let t227 = Py::None; if py::is(&t226, &t227) { true } else { false } }) }; Py::Bool({ let t225 = { let t228 = v_sfg.clone(); Py::Bool({ let t229 = Py::None; if py::is(&t228, &t229) { true } else { false } }) }; if py::ne(&t224, &t225) { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give each saturated pair whole: vf with vg, hf with hfg, sf with sfg"))));
    }
    if ({ let t239 = { let t232 = v_vf.clone(); Py::Bool({ let t233 = Py::None; if py::is(&t232, &t233) { true } else { false } }) }; if !t239.truthy() { t239 } else { { let t238 = { let t234 = v_hf.clone(); Py::Bool({ let t235 = Py::None; if py::is(&t234, &t235) { true } else { false } }) }; if !t238.truthy() { t238 } else { { let t236 = v_sf.clone(); Py::Bool({ let t237 = Py::None; if py::is(&t236, &t237) { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give at least one saturated pair (vf and vg, hf and hfg, or sf and sfg)"))));
    }
    v_n = Py::Int(0i64);
    for t240 in py::iter(&(Py::tuple(vec![v_x.clone(), v_v.clone(), v_h.clone(), v_s.clone()])))? {
        v_val = t240;
        if ({ let t241 = v_val.clone(); Py::Bool({ let t242 = Py::None; if !py::is(&t241, &t242) { true } else { false } }) }).truthy() {
            v_n = py::iadd(&v_n, &Py::Int(1i64))?;
        }
    }
    if ({ let t243 = v_n.clone(); Py::Bool({ let t244 = Py::Int(1i64); if py::ne(&t243, &t244) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("state the mixture by exactly one of the quality x, or its v, h or s"))));
    }
    if ({ let t245 = v_vf.clone(); Py::Bool({ let t246 = Py::None; if !py::is(&t245, &t246) { true } else { false } }) }).truthy() {
        if (py::not(&{ let t251 = py::math1("isfinite", &v_vf.clone())?; if !t251.truthy() { t251 } else { { let t250 = py::math1("isfinite", &v_vg.clone())?; if !t250.truthy() { t250 } else { { let t247 = Py::Int(0i64); Py::Bool({ let t248 = v_vf.clone(); if py::lt(&t247, &t248)? { { let t249 = v_vg.clone(); if py::lt(&t248, &t249)? { true } else { false } } } else { false } }) } } } } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the specific volumes must satisfy 0 < vf < vg"))));
        }
    }
    if ({ let t252 = v_hf.clone(); Py::Bool({ let t253 = Py::None; if !py::is(&t252, &t253) { true } else { false } }) }).truthy() {
        if (py::not(&{ let t257 = py::math1("isfinite", &v_hf.clone())?; if !t257.truthy() { t257 } else { { let t256 = py::math1("isfinite", &v_hfg.clone())?; if !t256.truthy() { t256 } else { { let t254 = v_hfg.clone(); Py::Bool({ let t255 = Py::Int(0i64); if py::gt(&t254, &t255)? { true } else { false } }) } } } } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the enthalpy of vaporization hfg must be positive"))));
        }
    }
    if ({ let t258 = v_sf.clone(); Py::Bool({ let t259 = Py::None; if !py::is(&t258, &t259) { true } else { false } }) }).truthy() {
        if (py::not(&{ let t263 = py::math1("isfinite", &v_sf.clone())?; if !t263.truthy() { t263 } else { { let t262 = py::math1("isfinite", &v_sfg.clone())?; if !t262.truthy() { t262 } else { { let t260 = v_sfg.clone(); Py::Bool({ let t261 = Py::Int(0i64); if py::gt(&t260, &t261)? { true } else { false } }) } } } } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the entropy of vaporization sfg must be positive"))));
        }
    }
    if ({ let t264 = v_x.clone(); Py::Bool({ let t265 = Py::None; if !py::is(&t264, &t265) { true } else { false } }) }).truthy() {
        if (py::not(&{ let t269 = py::math1("isfinite", &v_x.clone())?; if !t269.truthy() { t269 } else { { let t266 = Py::Int(0i64); Py::Bool({ let t267 = v_x.clone(); if py::le(&t266, &t267)? { { let t268 = Py::Int(1i64); if py::le(&t267, &t268)? { true } else { false } } } else { false } }) } } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the quality x must be a fraction from 0 to 1"))));
        }
        v_q = v_x.clone();
    } else {
        if ({ let t270 = v_v.clone(); Py::Bool({ let t271 = Py::None; if !py::is(&t270, &t271) { true } else { false } }) }).truthy() {
            if ({ let t272 = v_vf.clone(); Py::Bool({ let t273 = Py::None; if py::is(&t272, &t273) { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("a mixture specific volume needs vf and vg"))));
            }
            v_q = py::truediv(&py::sub(&v_v.clone(), &v_vf.clone())?, &py::sub(&v_vg.clone(), &v_vf.clone())?)?;
        } else {
            if ({ let t274 = v_h.clone(); Py::Bool({ let t275 = Py::None; if !py::is(&t274, &t275) { true } else { false } }) }).truthy() {
                if ({ let t276 = v_hf.clone(); Py::Bool({ let t277 = Py::None; if py::is(&t276, &t277) { true } else { false } }) }).truthy() {
                    return Err(py::value_error(py::to_str(&Py::str("a mixture enthalpy needs hf and hfg"))));
                }
                v_q = py::truediv(&py::sub(&v_h.clone(), &v_hf.clone())?, &v_hfg.clone())?;
            } else {
                if ({ let t278 = v_sf.clone(); Py::Bool({ let t279 = Py::None; if py::is(&t278, &t279) { true } else { false } }) }).truthy() {
                    return Err(py::value_error(py::to_str(&Py::str("a mixture entropy needs sf and sfg"))));
                }
                v_q = py::truediv(&py::sub(&v_s.clone(), &v_sf.clone())?, &v_sfg.clone())?;
            }
        }
        if (py::not(&py::math1("isfinite", &v_q.clone())?)).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the mixture property must be a finite number"))));
        }
        if ({ let t280 = py::neg(&Py::Float(1e-12))?; Py::Bool({ let t281 = v_q.clone(); if py::lt(&t280, &t281)? { { let t282 = Py::Int(0i64); if py::lt(&t281, &t282)? { true } else { false } } } else { false } }) }).truthy() {
            v_q = Py::Float(0.0);
        }
        if ({ let t283 = Py::Int(1i64); Py::Bool({ let t284 = v_q.clone(); if py::lt(&t283, &t284)? { { let t285 = py::add(&Py::Int(1i64), &Py::Float(1e-12))?; if py::lt(&t284, &t285)? { true } else { false } } } else { false } }) }).truthy() {
            v_q = Py::Float(1.0);
        }
        if ({ let t290 = { let t286 = v_q.clone(); Py::Bool({ let t287 = Py::Int(0i64); if py::lt(&t286, &t287)? { true } else { false } }) }; if t290.truthy() { t290 } else { { let t288 = v_q.clone(); Py::Bool({ let t289 = Py::Int(1i64); if py::gt(&t288, &t289)? { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the state is not a wet mixture: the property lies outside the saturated-liquid to saturated-vapour range (compressed liquid or superheated vapour)"))));
        }
    }
    v_out = Py::dict(vec![(Py::str("x"), v_q.clone()), (Py::str("moisture"), py::sub(&Py::Float(1.0), &v_q.clone())?)]);
    if ({ let t291 = v_vf.clone(); Py::Bool({ let t292 = Py::None; if !py::is(&t291, &t292) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("v"), py::add(&v_vf.clone(), &py::mul(&v_q.clone(), &py::sub(&v_vg.clone(), &v_vf.clone())?)?)?)?;
    }
    if ({ let t293 = v_hf.clone(); Py::Bool({ let t294 = Py::None; if !py::is(&t293, &t294) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("h"), py::add(&v_hf.clone(), &py::mul(&v_q.clone(), &v_hfg.clone())?)?)?;
        py::setitem(&v_out.clone(), &Py::str("hg"), py::add(&v_hf.clone(), &v_hfg.clone())?)?;
    }
    if ({ let t295 = v_sf.clone(); Py::Bool({ let t296 = Py::None; if !py::is(&t295, &t296) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("s"), py::add(&v_sf.clone(), &py::mul(&v_q.clone(), &v_sfg.clone())?)?)?;
        py::setitem(&v_out.clone(), &Py::str("sg"), py::add(&v_sf.clone(), &v_sfg.clone())?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_182(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(py::truediv(&py::abs(&py::sub(&py::getitem(&v_o.clone(), &Py::str("p_sat"))?, &f_pws(py::getitem(&v_i.clone(), &Py::str("T"))?)?)?)?, &f_pws(py::getitem(&v_i.clone(), &Py::str("T"))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}
