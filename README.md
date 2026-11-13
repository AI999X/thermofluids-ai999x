# thermofluids-ai999x: offline, verified thermofluids solver with step-by-step answers

[![tests](https://github.com/AI999X/thermofluids-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/thermofluids-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/thermofluids-ai999x)](https://pypi.org/project/thermofluids-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/thermofluids-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/thermofluids-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/thermofluids-ai999x/)** · **[Guides](https://huggingface.co/AI999X/thermofluids-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/thermofluids-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.6.0

Field: **Engineering** · Tasks: Question Answering

thermofluids-ai999x is an offline, deterministic thermofluids solver written in Rust: a desktop app, a command-line program, a browser module and a Python package. It answers thermofluids questions with verified results from 133 certified capabilities, gives 1057 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**thermofluids-ai999x** reads a question in plain English (or a typed call), picks a certified thermofluids capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions, or a refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** thermofluids-ai999x comes as the AI999X desktop app and as the command-line program `thermofluids` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Thermofluids desktop app** 1.5.0 | Windows, Linux. This model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-thermofluids-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-thermofluids-1.5.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.5.0 | Windows, Linux. Every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz) |
| **Command line** (`thermofluids`) | Windows, Linux. One program per model, offline, no Python | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz) |
| **In your browser** | any device. Nothing to install | [Compute on the model page](https://www.ai999x.com/models/thermofluids-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install thermofluids-ai999x` |

Checksums: [SHA256SUMS](https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS) · every version: [https://downloads.ai999x.com/desktop/latest/](https://downloads.ai999x.com/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Quick start

### Desktop app

Download **AI999X Thermofluids** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Thermofluids among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-thermofluids-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-thermofluids-1.5.0-linux-x86_64.tar.gz
./ai999x-thermofluids-1.5.0-linux-x86_64/ai999x-thermofluids
```

### Command line

Linux:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.5.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.5.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
thermofluids --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.5.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.5.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
thermofluids --version
```

Every command below was run as shown; the text under it is what the program printed.

**Ask in words**: Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```console
$ thermofluids ask "Water with density 998 kg/m^3 and dynamic viscosity 0.001 Pa*s flows at an average velocity of 1.5 m/s in a pipe of diameter 0.05 m. What is the Reynolds number?"
Read as: Reynolds number from density, velocity, diameter and viscosity
Reynolds number (Re) = 74850
  by: Reynolds number from density, velocity, diameter and viscosity  (Re = rho*v*D/mu)
  D = 0.05 m   [0.05 m]
  mu = 0.001 Pa*s   [0.001 Pa*s]
  rho = 998 kg/m^3   [998 kg/m^3]
  v = 1.5 m/s   [1.5 m/s]
  assumes: Newtonian fluid; the diameter as the characteristic length; the situation is: fluid flow
```

**What a capability needs**: The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ thermofluids info dew_point_temperature
Dew-point temperature from dry-bulb temperature and relative humidity
  p_v = RH p_ws(T), then p_ws(T_dew) = p_v solved by bisection to the last digit
  in   T        dry-bulb temperature [K]
  in   RH       relative humidity (fraction, or % written) []
  out  T_dew    dew-point temperature [K]
  out  p_v      partial pressure of the water vapour [Pa]
```

**Solve**: Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ thermofluids solve dew_point_temperature "T=25 °C" RH=0.6
T_dew = 289.8511319 K
p_v = 1901.529882 Pa
  by: Dew-point temperature from dry-bulb temperature and relative humidity  (p_v = RH p_ws(T), then p_ws(T_dew) = p_v solved by bisection to the last digit)
  T = 298.15 K
  RH = 0.6
  assumes: 0 < RH <= 1; -100 °C <= T <= 200 °C
```

**Many questions at once**: One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | thermofluids batch

`cases.jsonl`:

```json
{"key": "dew_point_temperature", "inputs": {"T": "25 °C", "RH": 0.6}}
{"key": "lmtd", "inputs": {"Th_in": "150 °C", "Th_out": "90 °C", "Tc_in": "30 °C", "Tc_out": "70 °C", "arrangement": "counterflow"}}
```

```console
$ thermofluids batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":289.85113185453645,"unit":"K","solved_for":"T_dew","si":[["T",298.15],["RH",0.6]],"values":[["T_dew",289.85113185453645,"K"],["p_v",1901.5298820861765,"Pa"]],"target":"T_dew","phrase":"dew-point temperature","assumptions":["0 < RH <= 1","-100 °C <= T <= 200 °C"],"name":"Dew-point temperature from dry-bulb temperature and relative humidity","equation":"p_v = RH p_ws(T), then p_ws(T_dew) = p_v solved by bisection to the last digit"}
{"outcome":"SOLVED","message":"","value":69.52118993564416,"unit":"K","solved_for":"lmtd","si":[["Th_in",423.15],["Th_out",363.15],["Tc_in",303.15],["Tc_out",343.15]],"values":[["lmtd",69.52118993564416,"K"],["dT1",80.0,"K"],["dT2",60.0,"K"]],"target":"lmtd","phrase":"log-mean temperature difference","assumptions":["both terminal temperature differences positive","the hot stream cools and the cold stream warms"],"name":"Log-mean temperature difference of a heat exchanger","equation":"(dT1 - dT2) / ln(dT1 / dT2) from the terminal differences of the chosen arrangement"}
```

### In your browser

Open [Thermofluids on ai999x.com](https://www.ai999x.com/models/thermofluids-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `thermofluids_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install thermofluids-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### Lift coefficient of a thin symmetric airfoil

Command line:

```console
$ thermofluids solve thin_airfoil_lift_coefficient "alpha=0.0872664626 rad"
section lift coefficient (CL) = 0.5483113556
  by: Lift coefficient of a thin symmetric airfoil  (CL = 2*pi*alpha)
  alpha = 0.0872664626 rad   [0.0872665 rad]
  assumes: thin symmetric airfoil; small angle of attack; inviscid incompressible flow; the situation is: thin airfoil
```

Python:

```python
import thermofluids_ai999x as m

a = m.solve("thin_airfoil_lift_coefficient", alpha="0.0872664626 rad")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/thermofluids-ai999x/#cap-thin_airfoil_lift_coefficient) and press **Compute in your browser**.

_What this means:_ This calculation determines the section lift coefficient for a thin symmetric airfoil. It uses the linear relationship between the lift coefficient and the angle of attack, assuming inviscid incompressible flow. The result indicates that at an angle of attack of 0.0872665 rad, the airfoil generates a lift coefficient of 0.548311. This simple model is useful for initial aerodynamic estimates where the wing is thin and the angle of attack is small.

### Average skin-friction coefficient of a laminar flat plate

Command line:

```console
$ thermofluids solve blasius_average_skin_friction "ReL=250000 1"
average skin-friction coefficient over the plate (cfm) = 0.00265645869
  by: Average skin-friction coefficient of a laminar flat plate  (cfm = 1.3282293448607851/sqrt(ReL))
  ReL = 250000   [250000 1]
  assumes: laminar boundary layer over the whole plate; zero pressure gradient; the situation is: flat plate, laminar
```

Python:

```python
import thermofluids_ai999x as m

a = m.solve("blasius_average_skin_friction", ReL="250000 1")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/thermofluids-ai999x/#cap-blasius_average_skin_friction) and press **Compute in your browser**.

_What this means:_ This calculation determines the average skin-friction coefficient over the entire surface of a flat plate, yielding a value of 0.00265646. It uses the Reynolds number based on the total plate length, which is 250000, to estimate the overall drag contribution under laminar flow conditions. This single dimensionless coefficient represents the total resistance experienced by the plate when the boundary layer remains smooth and attached throughout its length.

### Displacement thickness of a laminar flat-plate boundary layer

Command line:

```console
$ thermofluids solve blasius_displacement_thickness "x=0.5 m" "Rex=200000 1"
displacement thickness (ds) = 0.001923899089 m
  by: Displacement thickness of a laminar flat-plate boundary layer  (ds = 1.7207876575205*x/sqrt(Rex))
  x = 0.5 m   [0.5 m]
  Rex = 200000   [200000 1]
  assumes: steady laminar flow over a flat plate; zero pressure gradient; the situation is: flat plate, laminar
```

Python:

```python
import thermofluids_ai999x as m

a = m.solve("blasius_displacement_thickness", x="0.5 m", Rex="200000 1")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/thermofluids-ai999x/#cap-blasius_displacement_thickness) and press **Compute in your browser**.

_What this means:_ This calculation determines the displacement thickness, denoted as ds, which represents the distance by which the external streamline is shifted outward due to the slowing of fluid near the surface. The result, 0.0019239 m, is derived from the local Reynolds number, Rex, and the distance from the leading edge, x. This value quantifies how much the boundary layer effectively reduces the flow area in a steady laminar flow over a flat plate with zero pressure gradient.

### Local skin-friction coefficient of a laminar flat plate

Command line:

```console
$ thermofluids solve blasius_local_skin_friction "Rex=100000 1"
local skin-friction coefficient (cf) = 0.002100114992
  by: Local skin-friction coefficient of a laminar flat plate  (cf = 0.6641146724303926/sqrt(Rex))
  Rex = 100000   [100000 1]
  assumes: steady laminar flow over a flat plate; zero pressure gradient; the situation is: flat plate, laminar
```

Python:

```python
import thermofluids_ai999x as m

a = m.solve("blasius_local_skin_friction", Rex="100000 1")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/thermofluids-ai999x/#cap-blasius_local_skin_friction) and press **Compute in your browser**.

_What this means:_ This calculation determines the local skin-friction coefficient at a specific point on a laminar flat plate based on the local Reynolds number. The result, 0.00210011, represents the dimensionless shear stress exerted by the fluid on the surface at that location. This value is derived from the Blasius solution for steady, incompressible flow with zero pressure gradient. A lower coefficient indicates reduced frictional resistance at that particular distance along the plate.

## Documentation

- **Rust user guide**: the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/thermofluids-ai999x/blob/main/docs/thermofluids_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/thermofluids-ai999x/blob/main/docs/thermofluids_ai999x_rust_guide.md)
- **Python guide**: every capability with a Python example: [PDF](https://huggingface.co/AI999X/thermofluids-ai999x/blob/main/docs/thermofluids_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/thermofluids-ai999x/blob/main/docs/thermofluids_ai999x_capabilities.md)
- **Desktop app & command line**: downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page**: compute in your browser, every capability explained: https://www.ai999x.com/models/thermofluids-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified thermofluids capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes (SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR), never a silent guess. Every one of its 1057 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `thermofluids` command line for Windows and Linux, and WebAssembly on the model page, no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** fluid statics and dynamics, pipe flow, boundary layers, aerodynamics and compressible flow, heat conduction, convection and radiation, fins, heat exchangers, thermodynamic cycles, psychrometrics and combustion.

## Model overview

| Property | Value |
|---|---|
| Package | `thermofluids-ai999x` 1.6.0 (import `thermofluids_ai999x`) |
| Type | Deterministic neuro-symbolic thermofluids engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 27 certified derivations, 20 certified procedures, 86 certified relations |
| Knowledge pack | version 0.1.1 |
| Golden answers | 1057 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

```bash
pip install thermofluids-ai999x
```

```python
import thermofluids_ai999x as thermofluids

thermofluids.ask("Water with density 998 kg/m^3 and dynamic viscosity 0.001 Pa*s flows at an average velocity of 1.5 m/s in a pipe of diameter 0.05 m. What is the Reynolds number?").show()
# -> SOLVED — 74850 1  (reynolds_number)
thermofluids.ask("Oil of density 900 kg/m^3 and viscosity 0.1 Pa*s flows in laminar flow at a mean velocity of 0.5 m/s in a pipe of diameter 0.02 m. What is the friction factor?").show()
# -> SOLVED — 0.711111 1  (reynolds_number, laminar_friction_factor)
thermofluids.ask("An air-standard Otto cycle has a compression ratio of 8. What is its thermal efficiency?").show()
# -> SOLVED — 0.564725 1  (otto_efficiency_air)
thermofluids.ask("Dew point temperature: T = 25 \u00b0C, RH = 60%").show()
# -> SOLVED — T_dew = 289.851, p_v = 1901.53  (dew_point_temperature)
thermofluids.solve("lmtd", Th_in="150 \u00b0C", Th_out="90 \u00b0C", Tc_in="30 \u00b0C", Tc_out="70 \u00b0C", arrangement="counterflow").show()
# -> SOLVED — lmtd = 69.5212, dT1 = 80, dT2 = 60  (lmtd)
thermofluids.ask("Who created you?").message
# -> thermofluids-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

The Python package also installs a Python command, `thermofluids-ai999x "<question>"`; for the native program use `thermofluids` (above). The outputs above were produced by running this exact code on thermofluids-ai999x 1.6.0.

## Draw the answer

`thermofluids.plot(answer)` draws the answer's formula around its own result: the engine's numbers, as SVG (Jupyter shows it directly; `.figure()` gives a matplotlib figure with the `[plot]` extra, `.plotly()` a Plotly figure).

```python
a = thermofluids.ask("Water with density 998 kg/m^3 and dynamic viscosity 0.001 Pa*s flows at an average velocity of 1.5 m/s in a pipe of diameter 0.05 m. What is the Reynolds number?")
thermofluids.plot(a)
# -> Reynolds number from density, velocity, diameter and viscosity: Re against rho, 101 points, marked at 7.485e+04 1
```

## How every answer is verified

1. **Reading**: the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning**: only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing**: exact symbolic or high-precision numeric evaluation.
4. **Checking**: dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Python worked examples, with their steps

Each question was put to `thermofluids-ai999x`; the answer is the one it returned, with the steps it took.

**Reynolds number from density, velocity, diameter and viscosity** · _Pipe flow_

> Water with density 998 kg/m^3 and dynamic viscosity 0.001 Pa*s flows at an average velocity of 1.5 m/s in a pipe of diameter 0.05 m. What is the Reynolds number?

→ **Re = 74850 1**

<details><summary>Steps</summary>

- Reynolds number from density, velocity, diameter and viscosity: rho = 998 kg/m^3, v = 1.5 m/s, D = 0.05 m, mu = 0.001 Pa*s → Re = 74850 1

</details>

**Reynolds number from density, velocity, diameter and viscosity** · _Pipe flow_

> Oil of density 900 kg/m^3 and viscosity 0.1 Pa*s flows in laminar flow at a mean velocity of 0.5 m/s in a pipe of diameter 0.02 m. What is the friction factor?

→ **f = 0.7111111111 1**

<details><summary>Steps</summary>

- Reynolds number from density, velocity, diameter and viscosity: rho = 900 kg/m^3, v = 0.5 m/s, D = 0.02 m, mu = 0.1 Pa*s → Re = 90 1
- Darcy friction factor in laminar pipe flow: Re = 90 1 → f = 0.7111111111 1

</details>

**Thermal efficiency of the air-standard Otto cycle** · _Thermodynamic cycles_

> An air-standard Otto cycle has a compression ratio of 8. What is its thermal efficiency?

→ **eta = 0.5647247184 1**

<details><summary>Steps</summary>

- Thermal efficiency of the air-standard Otto cycle: r = 8 1 → eta = 0.5647247184 1

</details>

**Dew-point temperature from dry-bulb temperature and relative humidity** · _Psychrometrics_

> Dew point temperature: T = 25 °C, RH = 60%

→ **T_dew = 289.8511319 K, p_v = 1901.529882 Pa**

<details><summary>Steps</summary>

- Dew-point temperature from dry-bulb temperature and relative humidity: T = 25 °C, RH = 0.6 → T_dew = 289.8511319 K, p_v = 1901.529882 Pa

</details>

**Log-mean temperature difference of a heat exchanger** · _Heat exchangers_

`m.solve("lmtd", Th_in="150 °C", Th_out="90 °C", Tc_in="30 °C", Tc_out="70 °C", arrangement="counterflow")`

→ **lmtd = 69.52118994 K, dT1 = 80 K, dT2 = 60 K**

<details><summary>Steps</summary>

- Log-mean temperature difference of a heat exchanger: Th_in = 150 °C, Th_out = 90 °C, Tc_in = 30 °C, Tc_out = 70 °C, arrangement = counterflow → lmtd = 69.52118994 K, dT1 = 80 K, dT2 = 60 K

</details>

**Lift coefficient of a thin symmetric airfoil** · _Aerodynamics_

> Using thin airfoil theory, what is the lift coefficient of a symmetric airfoil at an angle of attack of 5 deg?

→ **CL = 0.5483113556 1**

<details><summary>Steps</summary>

- Lift coefficient of a thin symmetric airfoil: alpha = 0.0872664626 rad → CL = 0.5483113556 1

</details>

## Figures

Every point was computed by `thermofluids-ai999x`.

**Friction factor chart (Colebrook, turbulent flow)**

![Friction factor chart (Colebrook, turbulent flow), computed by thermofluids-ai999x](https://huggingface.co/AI999X/thermofluids-ai999x/resolve/main/figures/colebrook-friction-factor-chart.svg)

_Darcy friction factor of turbulent pipe flow against Reynolds number (logarithmic axis) for five relative roughnesses from smooth to ε/D = 0.05, each from the Colebrook equation solved by the package at up to 89 Reynolds numbers between 4000 and 10⁸. Marked: Re = 100000, ε/D = 0.0002, f = 0.0190054._

**Psychrometric chart: humidity ratio against temperature**

![Psychrometric chart: humidity ratio against temperature, computed by thermofluids-ai999x](https://huggingface.co/AI999X/thermofluids-ai999x/resolve/main/figures/humidity-ratio-from-relative-humidity-chart.svg)

_Psychrometric chart at 101325 Pa: humidity ratio against dry-bulb temperature at 20, 40, 60, 80 and 100 % relative humidity, computed by the package every 1 °C. Air at 30 °C and 40 % has a wet-bulb temperature of 20.0643 °C; marked: T = 30 °C, W = 10.6028 g/kg._

**Heat-exchanger effectiveness against NTU**

![Heat-exchanger effectiveness against NTU, computed by thermofluids-ai999x](https://huggingface.co/AI999X/thermofluids-ai999x/resolve/main/figures/hx-effectiveness-ntu-chart.svg)

_Heat-exchanger effectiveness against the number of transfer units for counterflow and parallel flow with heat-capacity rates 2000 and 4000 W/K (ratio 0.5), computed by the package at 60 values of UA. Marked: counterflow, UA = 3000 W/K, NTU = 1.5, ε = 0.690785, Q = 165788 W._

**Otto cycle on a pressure–volume diagram**

![Otto cycle on a pressure–volume diagram, computed by thermofluids-ai999x](https://huggingface.co/AI999X/thermofluids-ai999x/resolve/main/figures/otto-cycle-pv.svg)

_An air-standard Otto cycle drawn on pressure–volume axes: compression ratio 8, intake 1 bar and 300 K in 0.5 L, peak temperature 1800 K (stated). Each point is a solve of the package's isentropic and ideal-gas relations (amount of air 0.0200454 mol): T2 = 689.219 K, T4 = 783.496 K; the enclosed area is the net work. The package's Otto efficiency for r = 8 is 0.564725; marked: state 3, P = 48 bar._

**Wet-bulb temperature table**

![Wet-bulb temperature table, computed by thermofluids-ai999x](https://huggingface.co/AI999X/thermofluids-ai999x/resolve/main/figures/wet-bulb-temperature-heatmap.svg)

_Wet-bulb temperature at 101325 Pa for dry-bulb temperatures 10, 15, 20, 25, 30, 35, 40 °C (rows) and relative humidities 20, 40, 60, 80 and 100 % (columns), each cell computed by the package (°C, one decimal). At 100 % the wet bulb equals the dry bulb; at 40 °C and 20 % it is 22 °C._

## Capabilities

27 certified derivations, 20 certified procedures, 86 certified relations. A few of them:

- **Derivations:** derive bernoulli horizontal; derive blasius local skin friction; derive blasius momentum thickness; derive brayton efficiency air; derive buoyant force; derive continuity speed; derive darcy weisbach head loss; derive darcy weisbach pressure drop; derive diesel efficiency air; derive dynamic pressure; derive effectiveness phase change; derive humidity ratio vapour pressure; derive hydraulic power; derive hydrostatic force plane; derive minor loss head; derive nusselt heat transfer coefficient; derive otto efficiency air; derive overall heat rate; derive overall u plane wall; derive plane wall conduction; derive pump shaft power; derive radiation net exchange; derive radiation parallel plates; derive reynolds number …
- **Procedures:** Bernoulli's equation with a change of height; Boundary work of a constant-pressure (isobaric) process; Churchill-Bernstein correlation for a cylinder in cross flow; Churchill-Chu correlation for natural convection on a vertical plate; Darcy friction factor from the Colebrook equation; Dew-point temperature from dry-bulb temperature and relative humidity; Efficiency of a pin fin with an insulated tip; Entropy change for heat exchanged at a constant temperature; Entropy generated when heat flows between two reservoirs; Equivalence ratio of a fuel-air mixture; First law of thermodynamics: change of internal energy; Gnielinski correlation for turbulent pipe flow; Heat exchanger rating by the effectiveness-NTU method; Humidity ratio from temperature, relative humidity and pressure; Hydraulic press (Pascal's principle); Log-mean temperature difference of a heat exchanger; Saturation pressure of water vapour; Stoichiometric air-fuel ratio of a gaseous fuel; Thermodynamic wet-bulb temperature; Wet steam: quality and mixture properties from saturated values
- **Relations:** Absolute pressure at a depth in a liquid; Actual outlet temperature of an adiabatic air compressor; Adiabatic flame temperature with constant product specific heat; Average Nusselt number for laminar flow over a flat plate; Average skin-friction coefficient of a laminar flat plate; Bernoulli's equation along a horizontal streamline; Biot number; Blasius friction factor for turbulent flow in a smooth pipe; Buoyant force (Archimedes' principle); Carnot efficiency of a reversible heat engine; Coefficient of performance of a Carnot heat pump; Coefficient of performance of a Carnot refrigerator; Continuity: speed after a change of pipe diameter; Critical radius of insulation on a cylinder; Darcy friction factor in laminar pipe flow; Darcy-Weisbach head loss in a pipe; Darcy-Weisbach pressure drop in a pipe; Displacement thickness of a laminar flat-plate boundary layer; Dittus-Boelter correlation, fluid being cooled; Dittus-Boelter correlation, fluid being heated; Drag force from the drag coefficient; Dynamic pressure of a moving fluid; Effectiveness of a heat exchanger with one stream changing phase; Effectiveness of a parallel-flow heat exchanger …

`thermofluids_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified thermofluids capabilities, and any question whose stated numbers it cannot all use: it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (fluid statics and dynamics, pipe flow, boundary layers, aerodynamics and compressible flow, heat conduction, convection and radiation, fins, heat exchangers, thermodynamic cycles, psychrometrics and combustion); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused; use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{thermofluids_ai999x_1_6_0,
  title  = {thermofluids-ai999x: an offline, verified thermofluids solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.6.0},
  url    = {https://huggingface.co/AI999X/thermofluids-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/thermofluids-ai999x/discussions or visit https://ai999x.com.

## Support the project

thermofluids-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/thermofluids-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.0`; the [GitHub release](https://github.com/AI999X/thermofluids-ai999x/releases/tag/v1.6.0) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `thermofluids` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/thermofluids`) and its data (`rust/models/thermofluids/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p thermofluids-ai999x
./target/release/thermofluids --help
cargo test -p thermofluids-ai999x
```
