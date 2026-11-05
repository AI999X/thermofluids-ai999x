//! Every Thermofluids capability's own live figures (src/figures*.rs): its worked example gives each published
//! figure kind, a changed input changes the drawing, and no drawing holds a non-finite number or text outside its
//! canvas. FIG_DUMP=<dir> writes every drawing of the worked examples there (for looking at them).

use ax_engine::figures::{figures, Figure};
use ax_engine::Input;
use thermofluids_ai999x::pack;

/// The published figure kinds of every capability (publish/website/models/thermofluids-ai999x/figures, except the
/// generic "curve"), as tools/figure_parity.py lists them, with the page's worked example.
const KINDS: &[(&str, &[&str], &[(&str, &str)])] = &[
    ("churchill_bernstein_cylinder", &["field"], &[("Re", "10000"), ("Pr", "0.7")]),
    ("churchill_chu_vertical_plate", &["field"], &[("Ra", "1000000000"), ("Pr", "0.71")]),
    ("colebrook_friction_factor", &["chart", "chart-2"], &[("Re", "100000"), ("eD", "0.0002")]),
    ("dew_point_temperature", &["chart"], &[("T", "298.15"), ("RH", "0.6")]),
    ("humidity_ratio_from_relative_humidity", &["chart", "chart-2"], &[("T", "303.15"), ("RH", "0.5"), ("P", "101325")]),
    ("hx_effectiveness_ntu", &["chart", "profile"], &[("C_hot", "2000"), ("C_cold", "4000"), ("UA", "3000"),
                                                      ("Th_in", "423.15"), ("Tc_in", "303.15"), ("arrangement", "counterflow")]),
    ("lmtd", &["profile"], &[("Th_in", "423.15"), ("Th_out", "363.15"), ("Tc_in", "303.15"), ("Tc_out", "343.15"),
                             ("arrangement", "counterflow")]),
    ("pin_fin_efficiency", &["fin"], &[("h", "50"), ("k", "200"), ("D", "0.005"), ("L", "0.05")]),
    ("wet_bulb_temperature", &["chart", "heatmap"], &[("T", "303.15"), ("RH", "0.4"), ("P", "101325")]),
    ("hydrostatic_gauge_pressure", &["schematic"], &[("rho", "1000 kg/m^3"), ("h", "12 m")]),
    ("absolute_pressure_depth", &["schematic"], &[("P0", "101325 Pa"), ("rho", "1025 kg/m^3"), ("h", "20 m")]),
    ("buoyant_force", &["field"], &[("rho", "1000 kg/m^3"), ("V", "0.02 m^3")]),
    ("continuity_speed", &["schematic"], &[("v1", "2 m/s"), ("D1", "0.1 m"), ("D2", "0.05 m")]),
    ("bernoulli_horizontal", &["schematic"], &[("P1", "200 kPa"), ("rho", "1000 kg/m^3"), ("v1", "2 m/s"), ("v2", "6 m/s")]),
    ("dynamic_pressure", &["field"], &[("rho", "1.225 kg/m^3"), ("v", "40 m/s")]),
    ("kinematic_viscosity", &["field"], &[("mu", "0.002 Pa*s"), ("rho", "800 kg/m^3")]),
    ("laminar_friction_factor", &["chart"], &[("Re", "1200 1")]),
    ("blasius_friction_factor", &["chart"], &[("Re", "50000 1")]),
    ("swamee_jain_friction_factor", &["chart"], &[("eps", "4.5e-05 m"), ("D", "0.1 m"), ("Re", "100000 1")]),
    ("haaland_friction_factor", &["chart"], &[("eps", "0.00015 m"), ("D", "0.05 m"), ("Re", "80000 1")]),
    ("hagen_poiseuille", &["schematic"], &[("D", "0.01 m"), ("dP", "20000 Pa"), ("mu", "0.05 Pa*s"), ("L", "10 m")]),
    ("minor_loss_head", &["field"], &[("K", "10 1"), ("v", "3 m/s")]),
    ("mach_number", &["field"], &[("v", "250 m/s"), ("a", "300 m/s")]),
    ("normal_shock_mach_air", &["shock"], &[("M1", "2 1")]),
    ("normal_shock_pressure_air", &["shock"], &[("P1", "100 kPa"), ("M1", "2 1")]),
    ("blasius_displacement_thickness", &["profile"], &[("x", "0.5 m"), ("Rex", "200000 1")]),
    ("blasius_momentum_thickness", &["profile"], &[("x", "0.5 m"), ("Rex", "200000 1")]),
    ("froude_number", &["field"], &[("v", "3 m/s"), ("h", "0.5 m")]),
    ("plane_wall_conduction", &["profile"], &[("k", "0.7 W/(m*K)"), ("A", "10 m^2"), ("T1", "20 °C"), ("T2", "5 °C"),
                                              ("L", "0.2 m")]),
    ("cylinder_conduction", &["profile"], &[("k", "45 W/(m*K)"), ("L", "5 m"), ("T1", "120 °C"), ("T2", "119 °C"),
                                            ("r1", "0.05 m"), ("r2", "0.06 m")]),
    ("sphere_conduction", &["profile"], &[("k", "0.04 W/(m*K)"), ("T1", "400 K"), ("T2", "300 K"), ("r1", "0.1 m"),
                                          ("r2", "0.15 m")]),
    ("overall_u_plane_wall", &["network"], &[("h1", "10 W/(m^2*K)"), ("L", "0.2 m"), ("k", "0.7 W/(m*K)"),
                                             ("h2", "25 W/(m^2*K)")]),
    ("critical_insulation_radius", &["curve-2"], &[("k", "0.05 W/(m*K)"), ("h", "10 W/(m^2*K)")]),
    ("wall_generation_max_temperature", &["profile"], &[("Ts", "350 K"), ("q", "1000000.0 W/m^3"), ("L", "0.01 m"),
                                                        ("k", "20 W/(m*K)")]),
    ("dittus_boelter_heating", &["field"], &[("Re", "50000 1"), ("Pr", "5 1")]),
    ("dittus_boelter_cooling", &["field"], &[("Re", "50000 1"), ("Pr", "5 1")]),
    ("flat_plate_laminar_average_nusselt", &["field"], &[("ReL", "100000 1"), ("Pr", "0.7 1")]),
    ("flat_plate_laminar_local_nusselt", &["field"], &[("Rex", "100000 1"), ("Pr", "0.7 1")]),
    ("rayleigh_number", &["field"], &[("Gr", "100000000.0 1"), ("Pr", "0.71 1")]),
    ("radiation_parallel_plates", &["schematic"], &[("T1", "800 K"), ("T2", "400 K"), ("e1", "0.8 1"), ("e2", "0.6 1")]),
    ("infinite_fin_heat_rate", &["fin"], &[("h", "25 W/(m^2*K)"), ("P", "0.0314 m"), ("k", "200 W/(m*K)"),
                                           ("Ac", "7.85e-05 m^2"), ("Tb", "100 °C"), ("Tinf", "25 °C")]),
    ("lumped_capacitance_temperature", &["curve-2"], &[("Tinf", "25 °C"), ("Ti", "300 °C"), ("h", "40 W/(m^2*K)"),
                                                       ("A", "0.002 m^2"), ("t", "120 s"), ("rho", "7800 kg/m^3"),
                                                       ("V", "5e-06 m^3"), ("c", "450 J/(kg*K)")]),
    ("lumped_time_constant", &["curve-2"], &[("rho", "7800 kg/m^3"), ("V", "5e-06 m^3"), ("c", "450 J/(kg*K)"),
                                             ("h", "40 W/(m^2*K)"), ("A", "0.002 m^2")]),
    ("effectiveness_parallel_flow", &["field"], &[("N", "1 1"), ("Cr", "0.5 1")]),
    ("carnot_efficiency", &["cycle"], &[("Tc", "300 K"), ("Th", "600 K")]),
    ("carnot_cop_refrigerator", &["cycle"], &[("Tc", "270 K"), ("Th", "300 K")]),
    ("carnot_cop_heat_pump", &["cycle"], &[("Tc", "275 K"), ("Th", "295 K")]),
    ("otto_efficiency_air", &["cycle"], &[("r", "8 1")]),
    ("diesel_efficiency_air", &["cycle"], &[("r", "18 1"), ("rc", "2 1")]),
    ("brayton_efficiency_air", &["cycle"], &[("rp", "10 1")]),
    ("compressor_outlet_temperature_air", &["process"], &[("T1", "300 K"), ("P1", "100 kPa"), ("P2", "600 kPa"),
                                                          ("eta", "0.85 1")]),
    ("isothermal_work_ideal_gas", &["process"], &[("n", "1 mol"), ("T", "300 K"), ("V2", "0.02 m^3"), ("V1", "0.01 m^3")]),
    ("entropy_change_ideal_gas", &["process"], &[("cp", "1005 J/(kg*K)"), ("R", "287 J/(kg*K)"), ("T1", "300 K"),
                                                 ("T2", "600 K"), ("P1", "100 kPa"), ("P2", "500 kPa")]),
    ("humidity_ratio_vapour_pressure", &["field"], &[("pv", "2000 Pa"), ("P", "101325 Pa")]),
    ("moist_air_enthalpy", &["field"], &[("T", "25 °C"), ("W", "0.01 1")]),
];

fn given(ex: &[(&str, &str)]) -> Vec<(String, Input)> {
    ex.iter().map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect()
}

fn check_texts(f: &Figure) {
    if let Err(e) = ax_engine::figkit::check_figure(f) {
        panic!("{e}");
    }
}

#[test]
fn every_capability_draws_its_published_kinds() {
    let p = pack();
    let dump = std::env::var("FIG_DUMP").ok();
    let mut missing = vec![];
    for (key, kinds, ex) in KINDS {
        let g = given(ex);
        let a = ax_engine::solve(p, key, &g, None);
        assert!(a.solved(), "{key}: the worked example is not solved: {}", a.message);
        let figs = figures(p, key, &g, None, None);
        for kind in *kinds {
            match figs.iter().find(|f| f.kind == *kind) {
                Some(f) => {
                    assert_eq!(f.slug, format!("{}-{kind}", key.replace('_', "-")));
                    assert!(!f.svg.contains("NaN") && !f.svg.contains("inf\""), "{key} {kind}: a non-finite number");
                    check_texts(f);
                    let text = format!("{} {}", f.title, f.caption);
                    assert!(!text.contains('—'), "{key} {kind}: an em dash");
                    for w in text.split(|c: char| !c.is_alphanumeric()) {
                        assert!(!["we", "We", "our", "Our", "us"].contains(&w), "{key} {kind}: '{w}' in the text");
                    }
                    if let Some(d) = &dump {
                        std::fs::write(format!("{d}/{}.svg", f.slug), &f.svg).unwrap();
                    }
                }
                None => missing.push(format!("{key}: {kind}")),
            }
        }
        if let Some(i) = figs.iter().position(|f| f.slug.contains("-live-")) {
            assert!(figs[i..].iter().all(|f| f.slug.contains("-live-")), "{key}: generic figures before own ones");
        }
    }
    assert!(missing.is_empty(), "missing figures: {missing:?}");
}

/// The first number of a text multiplied by f ("12 m" -> "16.44 m"; the unit "1" is left alone).
fn scale_first(t: &str, f: f64) -> Option<String> {
    let cs: Vec<char> = t.chars().collect();
    let mut i = 0;
    while i < cs.len() && cs[i].is_whitespace() {
        i += 1;
    }
    let st = i;
    while i < cs.len() && (cs[i].is_ascii_digit() || cs[i] == '.' || cs[i] == '-' || cs[i] == '+'
        || ((cs[i] == 'e' || cs[i] == 'E') && i > st)) {
        i += 1;
    }
    let x: f64 = cs[st..i].iter().collect::<String>().parse().ok()?;
    if x == 0.0 {
        return None;
    }
    Some(format!("{}{}", x * f, cs[i..].iter().collect::<String>()))
}

#[test]
fn a_changed_input_changes_the_drawing() {
    let p = pack();
    let mut unchanged = vec![];
    for (key, kinds, ex) in KINDS {
        let g0 = given(ex);
        let base = figures(p, key, &g0, None, None);
        let mut tries: Vec<Vec<(String, Input)>> = vec![];
        for f in [1.1, 0.9, 1.37, 0.73] {
            let mut all = g0.clone();
            let mut any = false;
            for e in all.iter_mut() {
                if let Input::Text(t) = &e.1 {
                    if let Some(t2) = scale_first(t, f) {
                        e.1 = Input::Text(t2);
                        any = true;
                    }
                }
            }
            if any {
                tries.push(all);
            }
            for i in 0..g0.len() {
                if let Input::Text(t) = &g0[i].1 {
                    if let Some(t2) = scale_first(t, f) {
                        let mut g = g0.clone();
                        g[i].1 = Input::Text(t2);
                        tries.push(g);
                    }
                }
            }
        }
        let mut ok = false;
        for g in tries {
            let a = ax_engine::solve(p, key, &g, None);
            if !a.solved() {
                continue;
            }
            let figs = figures(p, key, &g, None, None);
            let differs = kinds.iter().all(|kind| match (base.iter().find(|f| f.kind == *kind), figs.iter().find(|f| f.kind == *kind)) {
                (Some(a), Some(b)) => a.svg != b.svg,
                _ => false,
            });
            if differs {
                ok = true;
                for f in figs.iter().filter(|f| !f.slug.contains("-live-")) {
                    assert!(!f.svg.contains("NaN"), "{key}: a non-finite number");
                    check_texts(f);
                }
                break;
            }
        }
        if !ok {
            unchanged.push(*key);
        }
    }
    assert!(unchanged.is_empty(), "drawing did not follow a changed input: {unchanged:?}");
}

#[test]
fn an_impossible_input_draws_nothing_wrong() {
    let p = pack();
    // a subsonic "shock", a "Carnot cycle" with its cold side hotter, a hot fin colder than nothing: no own figure
    for (key, kind, ex) in [
        ("normal_shock_mach_air", "shock", vec![("M1", "0.5 1")]),
        ("carnot_efficiency", "cycle", vec![("Tc", "700 K"), ("Th", "600 K")]),
        ("otto_efficiency_air", "cycle", vec![("r", "0.5 1")]),
        ("lmtd", "profile", vec![("Th_in", "300"), ("Th_out", "320"), ("Tc_in", "303.15"), ("Tc_out", "343.15"),
                                 ("arrangement", "counterflow")]),
    ] {
        let g: Vec<(String, Input)> = ex.iter().map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect();
        let figs = figures(p, key, &g, None, None);
        assert!(figs.iter().all(|f| f.kind != kind), "{key}: a {kind} drawn for impossible inputs");
    }
}

#[test]
fn a_formula_solved_for_another_quantity_keeps_its_figure() {
    // the gauge pressure asked the other way round: the depth from a pressure
    let p = pack();
    let g = given(&[("rho", "1000 kg/m^3"), ("P", "117.7 kPa")]);
    let a = ax_engine::solve(p, "hydrostatic_gauge_pressure", &g, Some("h"));
    assert!(a.solved());
    let figs = figures(p, "hydrostatic_gauge_pressure", &g, Some("h"), None);
    let f = figs.iter().find(|f| f.kind == "schematic").expect("the tank");
    check_texts(f);
}
