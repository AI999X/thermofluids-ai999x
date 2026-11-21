# Thermofluids (thermofluids-ai999x)

[![tests](https://github.com/AI999X/thermofluids-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/thermofluids-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/thermofluids-ai999x)](https://pypi.org/project/thermofluids-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/thermofluids-ai999x)

Thermofluids covers fluid statics and dynamics, pipe flow, boundary layers, aerodynamics, heat conduction, convection and radiation, heat exchangers, thermodynamic cycles, psychrometrics and combustion: 106 calculations in 22 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Thermofluids desktop app: Reynolds number from density, velocity, diameter and viscosity, with its worked example and live scene](https://www.ai999x.com/?ax_shot=thermofluids-ai999x/hero.webp)](https://www.ai999x.com/models/thermofluids-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-thermofluids-1.6.7-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-thermofluids-1.6.7-linux-x86_64.tar.gz) |
| **Command line** `thermofluids` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-cli-1.6.7-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-cli-1.6.7-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/thermofluids-ai999x/#capabilities) |
| **Python** 3.10+ | `pip install thermofluids-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## See it

From the Thermofluids desktop app: every scene and chart is drawn from the computed answer.

[![Thermofluids desktop app: a slider moves an input and the answer, the scene and the charts follow](https://www.ai999x.com/?ax_shot=thermofluids-ai999x/clip-loop.webp)](https://www.ai999x.com/?ax_shot=thermofluids-ai999x/clip.mp4)

*[Watch the full clip (MP4)](https://www.ai999x.com/?ax_shot=thermofluids-ai999x/clip.mp4)*

| | |
|---|---|
| [![Reynolds number from density, velocity, diameter and viscosity: the live scene in the Thermofluids desktop app](https://www.ai999x.com/?ax_shot=thermofluids-ai999x/scene-1.webp)](https://www.ai999x.com/models/thermofluids-ai999x/#cap-reynolds_number)<br>Reynolds number from density, velocity, diameter and viscosity | [![Thermal efficiency of the air-standard Otto cycle: the live scene in the Thermofluids desktop app](https://www.ai999x.com/?ax_shot=thermofluids-ai999x/scene-2.webp)](https://www.ai999x.com/models/thermofluids-ai999x/#cap-otto_efficiency_air)<br>Thermal efficiency of the air-standard Otto cycle |
| [![Dew-point temperature from dry-bulb temperature and relative humidity: the live scene in the Thermofluids desktop app](https://www.ai999x.com/?ax_shot=thermofluids-ai999x/scene-3.webp)](https://www.ai999x.com/models/thermofluids-ai999x/#cap-dew_point_temperature)<br>Dew-point temperature from dry-bulb temperature and relative humidity | [![Log-mean temperature difference of a heat exchanger: the live scene in the Thermofluids desktop app](https://www.ai999x.com/?ax_shot=thermofluids-ai999x/scene-4.webp)](https://www.ai999x.com/models/thermofluids-ai999x/#cap-lmtd)<br>Log-mean temperature difference of a heat exchanger |
| [![Lift coefficient of a thin symmetric airfoil: the live scene in the Thermofluids desktop app](https://www.ai999x.com/?ax_shot=thermofluids-ai999x/scene-5.webp)](https://www.ai999x.com/models/thermofluids-ai999x/#cap-thin_airfoil_lift_coefficient)<br>Lift coefficient of a thin symmetric airfoil | [![Average skin-friction coefficient of a laminar flat plate: the live scene in the Thermofluids desktop app](https://www.ai999x.com/?ax_shot=thermofluids-ai999x/scene-6.webp)](https://www.ai999x.com/models/thermofluids-ai999x/#cap-blasius_average_skin_friction)<br>Average skin-friction coefficient of a laminar flat plate |
| [![Adiabatic flame temperature with constant product specific heat: the live scene in the Thermofluids desktop app](https://www.ai999x.com/?ax_shot=thermofluids-ai999x/scene-7.webp)](https://www.ai999x.com/models/thermofluids-ai999x/#cap-adiabatic_flame_temperature_simple)<br>Adiabatic flame temperature with constant product specific heat | [![Isentropic stagnation pressure of air from the Mach number: the live scene in the Thermofluids desktop app](https://www.ai999x.com/?ax_shot=thermofluids-ai999x/scene-8.webp)](https://www.ai999x.com/models/thermofluids-ai999x/#cap-stagnation_pressure_air)<br>Isentropic stagnation pressure of air from the Mach number |

Charts:

|   |   |   |   |
|---|---|---|---|
| [![Reynolds number from density, velocity, diameter and viscosity: a chart in the Thermofluids desktop app](https://www.ai999x.com/?ax_shot=thermofluids-ai999x/chart-1.webp)](https://www.ai999x.com/models/thermofluids-ai999x/#cap-reynolds_number)<br>Reynolds number from density, velocity, diameter and viscosity | [![Thermal efficiency of the air-standard Otto cycle: a chart in the Thermofluids desktop app](https://www.ai999x.com/?ax_shot=thermofluids-ai999x/chart-2.webp)](https://www.ai999x.com/models/thermofluids-ai999x/#cap-otto_efficiency_air)<br>Thermal efficiency of the air-standard Otto cycle | [![Dew-point temperature from dry-bulb temperature and relative humidity: a chart in the Thermofluids desktop app](https://www.ai999x.com/?ax_shot=thermofluids-ai999x/chart-3.webp)](https://www.ai999x.com/models/thermofluids-ai999x/#cap-dew_point_temperature)<br>Dew-point temperature from dry-bulb temperature and relative humidity | [![Log-mean temperature difference of a heat exchanger: a chart in the Thermofluids desktop app](https://www.ai999x.com/?ax_shot=thermofluids-ai999x/chart-4.webp)](https://www.ai999x.com/models/thermofluids-ai999x/#cap-lmtd)<br>Log-mean temperature difference of a heat exchanger |

## Ask in words: 14 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import thermofluids_ai999x as thermofluids

a = thermofluids.ask(
    "Pin fin efficiency: h = 50 W/(m^2*K), k = 200 W/(m*K), D = 5 mm, "
    "L = 50 mm"
)
print(a.outcome, a.values)
# SOLVED {'efficiency': 0.8610571715805476, 'mL': 0.7071067811865476}

a = thermofluids.ask(
    "Bernoulli equation with height: P1 = 200 kPa, v1 = 2 m/s, h1 = 0 "
    "m, v2 = 6 m/s, h2 = 5 m, rho = 1000 kg/m^3, g = 9.81 m/s^2"
)
print(a.outcome, a.values)
# SOLVED {'P2': 134950.0}

a = thermofluids.ask("Dew point temperature: T = 25 °C, RH = 60%")
print(a.outcome, a.values)
# SOLVED {'T_dew': 289.85113185453645, 'p_v': 1901.5298820861765}

a = thermofluids.ask(
    "Isobaric work: P = 200 kPa, V1 = 0.010 m^3, V2 = 0.025 m^3"
)
print(a.outcome, a.values)
# SOLVED {'W': 3000.0000000000005, 'dV': 0.015000000000000001}

a = thermofluids.ask(
    "Churchill-Bernstein correlation: Re = 10000, Pr = 0.7"
)
print(a.outcome, a.values)
# SOLVED {'Nu': 53.32778867020997}

a = thermofluids.ask(
    "Colebrook friction factor: Re = 100000, eD = 0.0002"
)
print(a.outcome, a.values)
# SOLVED {'f': 0.01900543522195957}

a = thermofluids.ask(
    "Humidity ratio: T = 30 °C, RH = 50%, P = 101325 Pa"
)
print(a.outcome, a.values)
# SOLVED {'W': 0.01331020383863019, 'p_v': 2123.015121796302}

a = thermofluids.ask(
    "Entropy change of a reservoir: Q = 1000 J, T = 300 K"
)
print(a.outcome, a.values)
# SOLVED {'dS': 3.3333333333333335}

a = thermofluids.ask(
    "Hydraulic press: F1 = 100 N, A1 = 0.01 m^2, A2 = 0.5 m^2"
)
print(a.outcome, a.values)
# SOLVED {'F2': 5000.0, 'P': 10000.0, 'MA': 50.0, 'd_ratio': 0.02}

a = thermofluids.ask("Saturation vapour pressure of water: T = 25 °C")
print(a.outcome, a.values)
# SOLVED {'p_sat': 3169.2164701436277}

a = thermofluids.ask(
    "First law of thermodynamics: Q = 500 J, W = 200 J"
)
print(a.outcome, a.values)
# SOLVED {'dU': 300.0}

a = thermofluids.ask(
    "Gnielinski correlation: Re = 50000, Pr = 5, f = 0.021"
)
print(a.outcome, a.values)
# SOLVED {'Nu': 285.58931252195663}

a = thermofluids.ask(
    "Counterflow LMTD: Th_in = 150 °C, Th_out = 90 °C, Tc_in = 30 °C, "
    "Tc_out = 70 °C"
)
print(a.outcome, a.values)
# SOLVED {'lmtd': 69.52118993564416, 'dT1': 80.0, 'dT2': 60.0}

a = thermofluids.ask(
    "Wet bulb temperature: T = 30 °C, RH = 40%, P = 101325 Pa"
)
print(a.outcome, a.values)
# SOLVED {'T_wb': 293.2143470642154, 'W': 0.010602781187021928}
```

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/thermofluids-ai999x/blob/main/docs/thermofluids_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/thermofluids-ai999x/blob/main/docs/thermofluids_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/thermofluids-ai999x/blob/main/docs/thermofluids_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/thermofluids-ai999x/blob/main/docs/thermofluids_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/thermofluids-ai999x/blob/main/docs/thermofluids_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/thermofluids-ai999x/blob/main/docs/thermofluids_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/thermofluids-ai999x/blob/main/docs/thermofluids_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/thermofluids-ai999x/blob/main/docs/thermofluids_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/thermofluids-ai999x/blob/main/docs/thermofluids_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Thermofluids on ai999x.com](https://www.ai999x.com/models/thermofluids-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.7 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/thermofluids-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.7`; the [GitHub release](https://github.com/AI999X/thermofluids-ai999x/releases/tag/v1.6.7) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `thermofluids` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/thermofluids`) and its data (`rust/models/thermofluids/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p thermofluids-ai999x
./target/release/thermofluids --help
cargo test -p thermofluids-ai999x
```
