//! Thermofluids: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p thermofluids-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&thermofluids_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "thin_airfoil_lift_coefficient", "inputs": {"alpha": "0.0872664626 rad"}}));
    println!();
    show(json!({"key": "blasius_average_skin_friction", "inputs": {"ReL": "250000 1"}}));
    println!();
    show(json!({"key": "adiabatic_flame_temperature_simple", "inputs": {"T0": "298 K", "LHV": "50000000 J/kg", "AFR": "17.2 1", "cp": "1200 J/(kg*K)"}}));
    println!();
    show(json!({"key": "stagnation_pressure_air", "inputs": {"P": "50000 Pa", "M": "1.5 1"}}));
    println!();
    show(json!({"key": "critical_insulation_radius", "inputs": {"k": "0.05 W/(m*K)", "h": "10 W/(m^2*K)"}}));
    println!();
    show(json!({"key": "flat_plate_laminar_average_nusselt", "inputs": {"ReL": "100000 1", "Pr": "0.7 1"}}));
    println!();
    show(json!({"key": "drag_force", "inputs": {"Cd": "0.3 1", "rho": "1.2 kg/m^3", "v": "30 m/s", "A": "2.2 m^2"}}));
    println!();
    show(json!({"key": "pin_fin_efficiency", "inputs": {"h": "50 W/(m^2*K)", "k": "200 W/(m*K)", "D": "5 mm", "L": "50 mm"}}));
    println!();
    show(json!({"key": "bernoulli_horizontal", "inputs": {"P1": "200000 Pa", "rho": "1000 kg/m^3", "v1": "2 m/s", "v2": "6 m/s"}}));
    println!();
    show(json!({"key": "bernoulli_with_height", "inputs": {"P1": "200 kPa", "v1": "2 m/s", "h1": "0 m", "v2": "6 m/s", "h2": "5 m", "rho": "1000 kg/m^3", "g": "9.81 m/s^2"}}));
    println!();
    show(json!({"key": "absolute_pressure_depth", "inputs": {"P0": "101325 Pa", "rho": "1025 kg/m^3", "g": "9.80665 m/s^2", "h": "20 m"}}));
    println!();
    show(json!({"key": "compressor_outlet_temperature_air", "inputs": {"T1": "300 K", "P2": "600000 Pa", "P1": "100000 Pa", "eta": "0.85 1"}}));
    println!();
    show(json!({"key": "effectiveness_phase_change", "inputs": {"N": "2 1"}}));
    println!();
    show(json!({"key": "grashof_number", "inputs": {"g": "9.80665 m/s^2", "beta": "0.0033 1/K", "Ts": "330 K", "Tinf": "300 K", "L": "0.5 m", "nu": "1.6e-05 m^2/s"}}));
    println!();
    show(json!({"key": "froude_number", "inputs": {"v": "3 m/s", "g": "9.80665 m/s^2", "h": "0.5 m"}}));
}
