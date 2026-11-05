# thermofluids: the Thermofluids program, user guide

**thermofluids-ai999x 1.1.0**, Rust build: command line, desktop app and browser. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Thermofluids covers fluid statics and dynamics, pipe flow, boundary layers, aerodynamics, heat conduction, convection and radiation, heat exchangers, thermodynamic cycles, psychrometrics and combustion. This guide is about `thermofluids`, the model as a self-contained program for Windows and Linux: 99 capabilities in 20 topics, answered on your own computer with no network, no Python and nothing else to install. The same compiled model runs in the AI999X desktop app and in the browser on the model's page (https://www.ai999x.com/models/thermofluids-ai999x/); the Python package (`pip install thermofluids-ai999x`) has its own guide. Every command output printed here is what the program printed.

## Contents

- 1. Get it: desktop app, command line, browser, Python
- 2. Quick start
- 3. Writing inputs
- 4. Outcomes and exit codes
- 5. Scripts and agents: batch and JSON
- 6. Figures
- 7. In a web page or Node.js: WebAssembly
- 8. Every capability

## 1. Get it

Version 1.5.0 of the AI999X desktop app and of the command-line bundle. Checksums for every file: https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS; the newest version is always under https://downloads.ai999x.com/desktop/latest/. Full instructions: https://www.ai999x.com/desktop-cli/

| what | system | download |
|---|---|---|
| Desktop app: Thermofluids | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-thermofluids-1.5.0-windows-x86_64.zip |
| Desktop app: Thermofluids | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-thermofluids-1.5.0-linux-x86_64.tar.gz |
| Desktop app: all models | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-windows-x86_64.zip |
| Desktop app: all models | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz |
| Command line | Windows x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-windows-x86_64.zip |
| Command line | Linux x86-64 | https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz |

### Desktop app

Thermofluids in its own app: search, questions in words, input forms, live figures and explanations. Windows: Download ai999x-thermofluids-1.5.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-thermofluids.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-thermofluids-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-thermofluids-1.5.0-linux-x86_64.tar.gz
./ai999x-thermofluids-1.5.0-linux-x86_64/ai999x-thermofluids
```

It opens on this model: type a question or pick a capability, fill in the form and press Compute. The AI999X desktop app with every model works the same way (choose the model first). Windows: Download ai999x-1.5.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-1.5.0-linux-x86_64.tar.gz
./ai999x-1.5.0-linux-x86_64/ai999x
```

### Command line: Linux

The bundle holds one program per model; `thermofluids` is this one. The checksum line should print OK.

```shell
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.5.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.5.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
thermofluids --version
```

### Command line: Windows (PowerShell)

The fourth line should print True.

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

### In your browser

Open https://www.ai999x.com/models/thermofluids-ai999x/, choose a capability and press Compute in your browser: the same program runs on your device as WebAssembly; nothing is sent anywhere.

### Python

```shell
pip install thermofluids-ai999x
```

## 2. Quick start

### 2.1 Check the install

Prints the model, its version and that this is the Rust build.

```shell
$ thermofluids --version
thermofluids-ai999x 1.5.0 (Rust build, ax-engine 1.5.0)
```

### 2.2 Ask in words

Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```shell
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

### 2.3 Find a capability

Search the capabilities by what you want to know; the first column is the id to pass to info and solve.

```shell
$ thermofluids search "dew-point temperature from dry-bulb temperature and relative humidity"
dew_point_temperature                Dew-point temperature from dry-bulb temperature and relative humidity  (psychrometrics / dew point)
humidity_ratio_from_relative_humidity Humidity ratio from temperature, relative humidity and pressure  (psychrometrics / humidity ratio)
wet_bulb_temperature                 Thermodynamic wet-bulb temperature  (psychrometrics / wet bulb)
moist_air_specific_volume            Specific volume of moist air per kilogram of dry air  (psychrometrics)
moist_air_enthalpy                   Specific enthalpy of moist air  (psychrometrics)
humidity_ratio_vapour_pressure       Humidity ratio from the partial pressure of water vapour  (psychrometrics)
stagnation_temperature_mach          Stagnation temperature from the Mach number  (compressible flow)
isentropic_temperature_pressure_air  Temperature after an isentropic pressure change of air  (gas processes)
… 12 more
```

### 2.4 List the capabilities

Every capability the program answers: its id (what you pass to solve) and its name.

```shell
$ thermofluids list
hydrostatic_gauge_pressure               Gauge pressure at a depth in a liquid
absolute_pressure_depth                  Absolute pressure at a depth in a liquid
buoyant_force                            Buoyant force (Archimedes' principle)
continuity_speed                         Continuity: speed after a change of pipe diameter
bernoulli_horizontal                     Bernoulli's equation along a horizontal streamline
torricelli_efflux                        Speed of efflux from a hole (Torricelli's law)
dynamic_pressure                         Dynamic pressure of a moving fluid
reynolds_number                          Reynolds number from density, velocity, diameter and viscosity
… 91 more
```

### 2.5 What a capability needs

The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```shell
$ thermofluids info dew_point_temperature
Dew-point temperature from dry-bulb temperature and relative humidity
  p_v = RH p_ws(T), then p_ws(T_dew) = p_v solved by bisection to the last digit
  in   T        dry-bulb temperature [K]
  in   RH       relative humidity (fraction, or % written) []
  out  T_dew    dew-point temperature [K]
  out  p_v      partial pressure of the water vapour [Pa]
```

### 2.6 Solve

Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```shell
$ thermofluids solve dew_point_temperature "T=25 °C" RH=0.6
T_dew = 289.8511319 K
p_v = 1901.529882 Pa
  by: Dew-point temperature from dry-bulb temperature and relative humidity  (p_v = RH p_ws(T), then p_ws(T_dew) = p_v solved by bisection to the last digit)
  T = 298.15 K
  RH = 0.6
  assumes: 0 < RH <= 1; -100 °C <= T <= 200 °C
```

### 2.7 Machine-readable answer

--json prints the same answer as one line of JSON (values in SI): for scripts and other programs.

```shell
$ thermofluids solve dew_point_temperature "T=25 °C" RH=0.6 --json
{"outcome":"SOLVED","message":"","value":289.85113185453645,"unit":"K","solved_for":"T_dew","si":[["T",298.15],["RH",0.6]],"values":[["T_dew",289.85113185453645,"K"],["p_v",1901.5298820861763,"Pa"]],"target":"T_dew","phrase":"dew-point temperature","assumptions":["0 < RH <= 1","-100 °C <= T <= 200 °C"],"name":"Dew-point temperature from dry-bulb temperature and relative humidity","equation":"p_v = RH p_ws(T), then p_ws(T_dew) = p_v solved by bisection to the last digit"}
```

### 2.8 When an input is missing

No number is guessed: the answer names what is missing, shows a complete call, and the program exits with status 1.

```shell
$ thermofluids solve dew_point_temperature "T=25 °C"
NEEDS_INPUT: dew_point_temperature needs RH (relative humidity (fraction, or % written)). Example: solve('dew_point_temperature', T='298.15 K', RH=0.6).
```

### 2.9 Solve an equation for another quantity

An equation can be solved for any of its quantities: give the others and name the one you want with --target (here x).

```shell
$ thermofluids solve blasius_displacement_thickness "Rex=200000 1" "ds=0.0019239 m" --target x
distance from the leading edge (x) = 0.5000002369 m
  by: Displacement thickness of a laminar flat-plate boundary layer  (ds = 1.7207876575205*x/sqrt(Rex))
  Rex = 200000   [200000 1]
  ds = 0.0019239 m   [0.0019239 m]
  assumes: steady laminar flow over a flat plate; zero pressure gradient; the situation is: flat plate, laminar
```

### 2.10 Many questions at once

One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | thermofluids batch

`cases.jsonl`:

```json
{"key": "dew_point_temperature", "inputs": {"T": "25 °C", "RH": 0.6}}
{"key": "lmtd", "inputs": {"Th_in": "150 °C", "Th_out": "90 °C", "Tc_in": "30 °C", "Tc_out": "70 °C", "arrangement": "counterflow"}}
```

```shell
$ thermofluids batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":289.85113185453645,"unit":"K","solved_for":"T_dew","si":[["T",298.15],["RH",0.6]],"values":[["T_dew",289.85113185453645,"K"],["p_v",1901.5298820861763,"Pa"]],"target":"T_dew","phrase":"dew-point temperature","assumptions":["0 < RH <= 1","-100 °C <= T <= 200 °C"],"name":"Dew-point temperature from dry-bulb temperature and relative humidity","equation":"p_v = RH p_ws(T), then p_ws(T_dew) = p_v solved by bisection to the last digit"}
{"outcome":"SOLVED","message":"","value":69.52118993564416,"unit":"K","solved_for":"lmtd","si":[["Th_in",423.15],["Th_out",363.15],["Tc_in",303.15],["Tc_out",343.15]],"values":[["lmtd",69.52118993564416,"K"],["dT1",80.0,"K"],["dT2",60.0,"K"]],"target":"lmtd","phrase":"log-mean temperature difference","assumptions":["both terminal temperature differences positive","the hot stream cools and the cold stream warms"],"name":"Log-mean temperature difference of a heat exchanger","equation":"(dT1 - dT2) / ln(dT1 / dT2) from the terminal differences of the chosen arrangement"}
```

## 3. Writing inputs

Inputs are written `name=value`, one per argument; `thermofluids info <capability>` lists the names. Quote an argument that holds a space or brackets (bash, zsh and PowerShell all accept double quotes). The answer always states each input as it was read, converted to SI, so a misread unit is visible at once.

| kind | example | how it is read |
|---|---|---|
| a number with a unit | `"R=0.128 nm"` | any unit of the same kind; converted to SI |
| a number in SI | `R=1.28e-10` | no unit means the SI unit shown by info |
| a choice | `structure=FCC` | one of the values info lists |
| a list or vector | `"v=[1, 2, 3]" or "v=1, 2, 3"` | brackets keep it a list |
| a matrix | `"A=[[1, 2], [3, 4]]"` | one list per row |
| true / false | `inclusive=true` | for yes/no options |

An equation can be solved for any of its quantities: give the others and add `--target <symbol>`. Without `--target`, the program solves for the quantity the capability is named after.

## 4. Outcomes and exit codes

The program never guesses a number: when it cannot answer, it says why and exits with status 1, so a script can tell an answer from a refusal without reading the text.

| outcome | meaning | exit status |
|---|---|---|
| SOLVED | the answer: every output with its unit, the method, the inputs in SI, the conditions assumed | 0 |
| NEEDS_INPUT | an input is missing; the message names it and shows a complete call | 1 |
| INVALID_INPUT | a value or unit of the wrong kind, an unknown input name, or a value outside the conditions | 1 |
| AMBIGUOUS | the inputs allow two different answers; say which one is meant | 1 |
| OUT_OF_SCOPE | the model has no capability for this, or the result would not be meaningful | 1 |
| (usage) | a command the program does not know: it prints its usage | 2 |

## 5. Scripts and agents: batch and JSON

`thermofluids solve ... --json` prints one answer as one line of JSON. `thermofluids batch` reads one request per line on standard input and writes one JSON answer per line, in order, starting the program once for any number of requests. A request is `{"key": "<capability>", "inputs": {"name": "value", ...}, "target": null}`. In PowerShell, pipe the file in: `Get-Content cases.jsonl | thermofluids batch`. `thermofluids catalog` prints every capability with its inputs as JSON, to build forms or tool definitions.

| answer field | meaning |
|---|---|
| outcome | one of the outcomes above |
| message | why there is no answer (empty when SOLVED) |
| value, unit | the main result in SI units |
| values | every output: [symbol, value, unit] (procedures) |
| target | the quantity solved for |
| si | each numeric input converted to SI: [symbol, value] |
| assumptions | the conditions the answer relies on |
| name, equation | the capability and its equation or method |

```python
import json, subprocess

reqs = [{"key": "<capability>", "inputs": {"name": "value"}}]
out = subprocess.run(["thermofluids", "batch"], input="\n".join(map(json.dumps, reqs)),
                     capture_output=True, text=True, check=True).stdout
answers = [json.loads(line) for line in out.splitlines()]
```

## 6. Figures

`thermofluids figures <capability> name=value ... --out DIR` draws the live figures for those inputs and writes each as an SVG file into DIR (an existing folder); without `--out` the SVG text comes back as JSON. The desktop app and the model page draw the same figures as soon as you press Compute.

## 7. In a web page or Node.js: WebAssembly

The same model is a WebAssembly module, `thermofluids_ai999x.wasm` (https://www.ai999x.com/compute/rust/thermofluids_ai999x.wasm), with a small loader, `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js). It runs in a browser, a Web Worker or Node.js, with no server.

```html
<script src="ax-wasm.js"></script>
<script>
  AXWasm.load("thermofluids_ai999x.wasm").then(function (m) {
    var a = m.solve({key: "<capability>", inputs: {name: "value"}});
    console.log(a.outcome, a.values || a.value);
    console.log(m.catalog().capabilities.length + " capabilities");
  });
</script>
```

## 8. Every capability

By topic. For each: what it does, what it needs and gives, when it applies, and a command with what the program printed for it. `thermofluids info <id>` shows the same inputs and outputs in the terminal.

### Aerodynamics (1)

<a id="thin_airfoil_lift_coefficient"></a>

#### Lift coefficient of a thin symmetric airfoil, `thin_airfoil_lift_coefficient`

One equation links section lift coefficient (CL) and angle of attack (alpha). Give all but one of them and it is solved for the one left out, by default section lift coefficient (CL).

$$
C_{L} = 2 \pi \alpha
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `alpha` | in | angle of attack | rad |
| `CL` | out | section lift coefficient | - |

**Applies when:** thin symmetric airfoil; small angle of attack; inviscid incompressible flow; small angles of attack below stall.

```shell
$ thermofluids solve thin_airfoil_lift_coefficient "alpha=0.0872664626 rad"
section lift coefficient (CL) = 0.5483113556
  by: Lift coefficient of a thin symmetric airfoil  (CL = 2*pi*alpha)
  alpha = 0.0872664626 rad   [0.0872665 rad]
  assumes: thin symmetric airfoil; small angle of attack; inviscid incompressible flow; the situation is: thin airfoil
```

**What the worked example means:** This calculation determines the section lift coefficient for a thin symmetric airfoil. It uses the linear relationship between the lift coefficient and the angle of attack, assuming inviscid incompressible flow. The result indicates that at an angle of attack of 0.0872665 rad, the airfoil generates a lift coefficient of 0.548311. This simple model is useful for initial aerodynamic estimates where the wing is thin and the angle of attack is small.

### Boundary layers (4)

<a id="blasius_average_skin_friction"></a>

#### Average skin-friction coefficient of a laminar flat plate, `blasius_average_skin_friction`

One equation links average skin-friction coefficient over the plate (cfm) and Reynolds number based on the plate length (ReL). Give all but one of them and it is solved for the one left out, by default average skin-friction coefficient over the plate (cfm).

$$
\text{cfm} = \frac{1.3282293448607851}{\sqrt{\text{ReL}}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `ReL` | in | Reynolds number based on the plate length | - |
| `cfm` | out | average skin-friction coefficient over the plate | - |

**Applies when:** laminar boundary layer over the whole plate; zero pressure gradient; Re_L below about 500000.

```shell
$ thermofluids solve blasius_average_skin_friction "ReL=250000 1"
average skin-friction coefficient over the plate (cfm) = 0.00265645869
  by: Average skin-friction coefficient of a laminar flat plate  (cfm = 1.3282293448607851/sqrt(ReL))
  ReL = 250000   [250000 1]
  assumes: laminar boundary layer over the whole plate; zero pressure gradient; the situation is: flat plate, laminar
```

**What the worked example means:** This calculation determines the average skin-friction coefficient over the entire surface of a flat plate, yielding a value of 0.00265646. It uses the Reynolds number based on the total plate length, which is 250000, to estimate the overall drag contribution under laminar flow conditions. This single dimensionless coefficient represents the total resistance experienced by the plate when the boundary layer remains smooth and attached throughout its length.

<a id="blasius_displacement_thickness"></a>

#### Displacement thickness of a laminar flat-plate boundary layer, `blasius_displacement_thickness`

One equation links displacement thickness (ds), distance from the leading edge (x) and local Reynolds number U x / nu (Rex). Give all but one of them and it is solved for the one left out, by default displacement thickness (ds).

$$
d_{s} = \frac{1.7207876575205 x}{\sqrt{\text{Rex}}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | distance from the leading edge | m |
| `Rex` | in | local Reynolds number U x / nu | - |
| `ds` | out | displacement thickness | m |

**Applies when:** steady laminar flow over a flat plate; zero pressure gradient; Re_x below about 500000.

```shell
$ thermofluids solve blasius_displacement_thickness "x=0.5 m" "Rex=200000 1"
displacement thickness (ds) = 0.001923899089 m
  by: Displacement thickness of a laminar flat-plate boundary layer  (ds = 1.7207876575205*x/sqrt(Rex))
  x = 0.5 m   [0.5 m]
  Rex = 200000   [200000 1]
  assumes: steady laminar flow over a flat plate; zero pressure gradient; the situation is: flat plate, laminar
```

**What the worked example means:** This calculation determines the displacement thickness, denoted as ds, which represents the distance by which the external streamline is shifted outward due to the slowing of fluid near the surface. The result, 0.0019239 m, is derived from the local Reynolds number, Rex, and the distance from the leading edge, x. This value quantifies how much the boundary layer effectively reduces the flow area in a steady laminar flow over a flat plate with zero pressure gradient.

<a id="blasius_local_skin_friction"></a>

#### Local skin-friction coefficient of a laminar flat plate, `blasius_local_skin_friction`

One equation links local skin-friction coefficient (cf) and local Reynolds number U x / nu (Rex). Give all but one of them and it is solved for the one left out, by default local skin-friction coefficient (cf).

$$
c_{f} = \frac{0.6641146724303926}{\sqrt{\text{Rex}}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Rex` | in | local Reynolds number U x / nu | - |
| `cf` | out | local skin-friction coefficient | - |

**Applies when:** steady laminar flow over a flat plate; zero pressure gradient; Re_x below about 500000.

```shell
$ thermofluids solve blasius_local_skin_friction "Rex=100000 1"
local skin-friction coefficient (cf) = 0.002100114992
  by: Local skin-friction coefficient of a laminar flat plate  (cf = 0.6641146724303926/sqrt(Rex))
  Rex = 100000   [100000 1]
  assumes: steady laminar flow over a flat plate; zero pressure gradient; the situation is: flat plate, laminar
```

**What the worked example means:** This calculation determines the local skin-friction coefficient at a specific point on a laminar flat plate based on the local Reynolds number. The result, 0.00210011, represents the dimensionless shear stress exerted by the fluid on the surface at that location. This value is derived from the Blasius solution for steady, incompressible flow with zero pressure gradient. A lower coefficient indicates reduced frictional resistance at that particular distance along the plate.

<a id="blasius_momentum_thickness"></a>

#### Momentum thickness of a laminar flat-plate boundary layer, `blasius_momentum_thickness`

One equation links momentum thickness (th), distance from the leading edge (x) and local Reynolds number U x / nu (Rex). Give all but one of them and it is solved for the one left out, by default momentum thickness (th).

$$
t_{h} = \frac{0.6641146724303926 x}{\sqrt{\text{Rex}}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `x` | in | distance from the leading edge | m |
| `Rex` | in | local Reynolds number U x / nu | - |
| `th` | out | momentum thickness | m |

**Applies when:** steady laminar flow over a flat plate; zero pressure gradient; Re_x below about 500000.

```shell
$ thermofluids solve blasius_momentum_thickness "x=0.5 m" "Rex=200000 1"
momentum thickness (th) = 0.0007425027762 m
  by: Momentum thickness of a laminar flat-plate boundary layer  (th = 0.6641146724303926*x/sqrt(Rex))
  x = 0.5 m   [0.5 m]
  Rex = 200000   [200000 1]
  assumes: steady laminar flow over a flat plate; zero pressure gradient; the situation is: flat plate, laminar
```

**What the worked example means:** This calculation determines the momentum thickness of a laminar flat-plate boundary layer using the local Reynolds number and distance from the leading edge. The resulting momentum thickness, 0.000742503 m, represents the loss of momentum flux in the boundary layer relative to ideal inviscid flow. This value is directly related to the skin friction drag experienced by the plate surface.

### Combustion (3)

<a id="adiabatic_flame_temperature_simple"></a>

#### Adiabatic flame temperature with constant product specific heat, `adiabatic_flame_temperature_simple`

One equation links adiabatic flame temperature (Tad), temperature of the reactants (T0), lower heating value of the fuel (LHV), air-fuel ratio by mass (AFR) and mean specific heat of the products (cp). Give all but one of them and it is solved for the one left out, by default adiabatic flame temperature (Tad).

$$
\text{Tad} = T_{0} + \frac{\text{LHV}}{\left(1 + \text{AFR}\right) c_{p}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T0` | in | temperature of the reactants | K |
| `LHV` | in | lower heating value of the fuel | J/kg |
| `AFR` | in | air-fuel ratio by mass | - |
| `cp` | in | mean specific heat of the products | J/(kg*K) |
| `Tad` | out | adiabatic flame temperature | K |

**Applies when:** complete combustion; no dissociation; adiabatic, constant-pressure burning; constant mean cp of the products; first estimates of flame temperature for lean or stoichiometric mixtures.

```shell
$ thermofluids solve adiabatic_flame_temperature_simple "T0=298 K" "LHV=50000000 J/kg" "AFR=17.2 1" "cp=1200 J/(kg*K)"
adiabatic flame temperature (Tad) = 2587.377289 K
  by: Adiabatic flame temperature with constant product specific heat  (Tad = T0 + LHV/((1 + AFR)*cp))
  T0 = 298 K   [298 K]
  LHV = 50000000 J/kg   [5e+07 J/kg]
  AFR = 17.2   [17.2 1]
  cp = 1200 J/(kg*K)   [1200 J/(kg*K)]
  assumes: complete combustion; no dissociation; adiabatic, constant-pressure burning; constant mean cp of the products; the situation is: combustion
```

**What the worked example means:** This calculation determines the adiabatic flame temperature, which represents the theoretical maximum temperature achieved during complete combustion without heat loss. The result of 2587.38 K is derived from the reactant temperature 298 K, the fuel's lower heating value 5e+07 J/kg, the air-fuel ratio 17.2, and the mean specific heat of the products 1200 J/(kg*K). This estimate helps engineers assess thermal loads and verify if a mixture can sustain ignition under ideal conditions.

<a id="equivalence_ratio"></a>

#### Equivalence ratio of a fuel-air mixture: `equivalence_ratio`

Equivalence ratio and excess air of a gaseous fuel burned at a given air-fuel ratio by mass.

$$
\phi = \frac{AFR_s}{AFR}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `fuel` | in | the fuel | - |
| `afr` | in | actual air-fuel ratio by mass | - |
| `phi` | out | equivalence ratio (fuel-air ratio over stoichiometric) | - |
| `afr_stoich` | out | stoichiometric air-fuel ratio | - |
| `excess_air` | out | excess air as a fraction | - |

**Applies when:** afr > 0.

```shell
$ thermofluids solve equivalence_ratio fuel=methane afr=20
phi = 0.8560159571
afr_stoich = 17.12031914
excess_air = 0.168202522
  by: Equivalence ratio of a fuel-air mixture  (phi = AFR_s / AFR with the stoichiometric ratio from the atom balance)
  afr = 20
  fuel = methane
  assumes: afr > 0
```

**What the worked example means:** This calculation determines the equivalence ratio for a fuel-air mixture by comparing the stoichiometric air-fuel ratio, which is 17.1203, against the actual air-fuel ratio provided as input, 20. The resulting equivalence ratio is 0.856016. A value greater than one indicates a rich mixture with insufficient air, while a value less than one signifies a lean mixture with excess air. The corresponding excess air fraction is calculated as 0.168203, quantifying the amount of air supplied beyond what is chemically required for complete combustion.

<a id="stoichiometric_air_fuel_ratio"></a>

#### Stoichiometric air-fuel ratio of a gaseous fuel, `stoichiometric_air_fuel_ratio`

Mass of air per mass of fuel for complete combustion, with the moles of O2 and air per mole of fuel.

$$
AFR_s = \frac{(x + y/4)(M_{O_2} + 3.76 M_{N_2})}{x M_C + y M_H}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `fuel` | in | the fuel | - |
| `afr_stoich` | out | stoichiometric air-fuel ratio (mass) | - |
| `o2_moles` | out | moles of O2 per mole of fuel | - |
| `air_moles` | out | moles of air per mole of fuel | - |

**Applies when:** complete combustion to CO2 and H2O; air = O2 + 3.76 N2.

```shell
$ thermofluids solve stoichiometric_air_fuel_ratio fuel=methane
afr_stoich = 17.12031914
o2_moles = 2
air_moles = 9.52
  by: Stoichiometric air-fuel ratio of a gaseous fuel  (atom balance of CxHy + (x + y/4)(O2 + 3.76 N2), then the mass ratio with standard atomic weights)
  fuel = methane
  assumes: complete combustion to CO2 and H2O; air = O2 + 3.76 N2
```

**What the worked example means:** This calculation determines the theoretical minimum mass of air needed for the complete combustion of a gaseous hydrocarbon fuel. By balancing the atoms in the reaction between the fuel and oxygen, it establishes that 2 moles of oxygen are required to fully convert the carbon and hydrogen in one mole of fuel into carbon dioxide and water vapor. Since air is composed of approximately one part oxygen and 3.76 parts nitrogen by volume, this corresponds to 9.52 total moles of air. The final result, 17.1203, represents the ratio of the mass of this air supply to the mass of the fuel consumed, providing a baseline for engine efficiency and emissions analysis.

### Compressible flow (6)

<a id="stagnation_pressure_air"></a>

#### Isentropic stagnation pressure of air from the Mach number, `stagnation_pressure_air`

One equation links stagnation pressure (P0), static pressure (P) and Mach number (M). Give all but one of them and it is solved for the one left out, by default stagnation pressure (P0).

$$
P_{0} = P \left(1 + \frac{M^{2}}{5}\right)^{\frac{7}{2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P` | in | static pressure | Pa |
| `M` | in | Mach number | - |
| `P0` | out | stagnation pressure | Pa |

**Applies when:** air as an ideal gas with gamma = 1.4; isentropic deceleration; isentropic flow of air.

```shell
$ thermofluids solve stagnation_pressure_air "P=50000 Pa" "M=1.5 1"
stagnation pressure (P0) = 183551.5314 Pa
  by: Isentropic stagnation pressure of air from the Mach number  (P0 = P*(1 + M**2/5)**(7/2))
  P = 50000 Pa   [50000 Pa]
  M = 1.5   [1.5 1]
  assumes: air as an ideal gas with gamma = 1.4; isentropic deceleration; the situation is: compressible, air
```

**What the worked example means:** This calculation determines the stagnation pressure, which represents the total pressure potential of the airflow if it were brought to rest without friction or heat transfer. The result, 183552 Pa, is derived from the static pressure 50000 Pa and the Mach number 1.5. A higher stagnation pressure indicates greater capacity for work extraction in the moving airstream before any losses occur.

<a id="mach_number"></a>

#### Mach number: `mach_number`

One equation links Mach number (M), mean flow velocity (v) and local speed of sound (a). Give all but one of them and it is solved for the one left out, by default Mach number (M).

$$
M = \frac{v}{a}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `v` | in | mean flow velocity | m/s |
| `a` | in | local speed of sound | m/s |
| `M` | out | Mach number | - |

**Applies when:** v and a measured in the same frame and state; any flow or flight speed compared with the local speed of sound.

```shell
$ thermofluids solve mach_number "v=250 m/s" "a=300 m/s"
Mach number (M) = 0.8333333333
  by: Mach number  (M = v/a)
  v = 250 m/s   [250 m/s]
  a = 300 m/s   [300 m/s]
  assumes: v and a measured in the same frame and state; the situation is: compressible
```

**What the worked example means:** This calculation determines the Mach number, which compares the mean flow velocity 250 m/s to the local speed of sound 300 m/s. A value above one indicates supersonic conditions where compressibility effects are significant, while a lower value suggests subsonic flow. The result helps engineers assess how air behaves around objects moving at these speeds.

<a id="normal_shock_mach_air"></a>

#### Mach number behind a normal shock in air: `normal_shock_mach_air`

One equation links Mach number behind the shock (M2) and Mach number ahead of the shock (M1). Give all but one of them and it is solved for the one left out, by default Mach number behind the shock (M2).

$$
M_{2} = \sqrt{\frac{M_{1}^{2} + 5}{7 M_{1}^{2} - 1}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `M1` | in | Mach number ahead of the shock | - |
| `M2` | out | Mach number behind the shock | - |

**Applies when:** air as an ideal gas with gamma = 1.4; steady normal shock; adiabatic; a stationary normal shock with supersonic upstream flow.

```shell
$ thermofluids solve normal_shock_mach_air "M1=2 1"
Mach number behind the shock (M2) = 0.5773502692
  by: Mach number behind a normal shock in air  (M2 = sqrt((M1**2 + 5)/(7*M1**2 - 1)))
  M1 = 2   [2 1]
  assumes: air as an ideal gas with gamma = 1.4; steady normal shock; adiabatic; the situation is: shock, air
```

**What the worked example means:** This calculation determines the subsonic Mach number behind a normal shock wave in air, denoted as 0.57735, using the supersonic upstream Mach number, given as 2. The result indicates that the flow decelerates to a subsonic speed across the discontinuity, reflecting the irreversible entropy increase and pressure rise associated with the shock structure.

<a id="speed_of_sound_ideal_gas"></a>

#### Speed of sound in an ideal gas: `speed_of_sound_ideal_gas`

One equation links speed of sound (a), ratio of specific heats (gamma), specific gas constant (R) and absolute temperature of the gas (T). Give all but one of them and it is solved for the one left out, by default speed of sound (a).

$$
a = \sqrt{\gamma R T}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `gamma` | in | ratio of specific heats | - |
| `R` | in | specific gas constant | J/(kg*K) |
| `T` | in | absolute temperature of the gas | K |
| `a` | out | speed of sound | m/s |

**Applies when:** ideal gas; small-amplitude (acoustic) waves; gases at moderate pressure.

```shell
$ thermofluids solve speed_of_sound_ideal_gas "gamma=1.4 1" "R=287 J/(kg*K)" "T=300 K"
speed of sound (a) = 347.1887095 m/s
  by: Speed of sound in an ideal gas  (a = sqrt(gamma*R*T))
  gamma = 1.4   [1.4 1]
  R = 287 J/(kg*K)   [287 J/(kg*K)]
  T = 300 K   [300 K]
  assumes: ideal gas; small-amplitude (acoustic) waves; the situation is: compressible
```

**What the worked example means:** This calculation determines the speed at which small pressure disturbances travel through an ideal gas. The result, 347.189 m/s, depends on the ratio of specific heats, the specific gas constant, and the absolute temperature. A higher value indicates that acoustic waves propagate more quickly through the medium under these conditions.

<a id="stagnation_temperature_mach"></a>

#### Stagnation temperature from the Mach number, `stagnation_temperature_mach`

One equation links stagnation (total) temperature (T0), static temperature (T), ratio of specific heats (gamma) and Mach number (M). Give all but one of them and it is solved for the one left out, by default stagnation (total) temperature (T0).

$$
T_{0} = T \left(1 + \frac{\left(\gamma - 1\right) M^{2}}{2}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | static temperature | K |
| `gamma` | in | ratio of specific heats | - |
| `M` | in | Mach number | - |
| `T0` | out | stagnation (total) temperature | K |

**Applies when:** ideal gas with constant specific heats; adiabatic deceleration; any adiabatic flow of an ideal gas.

```shell
$ thermofluids solve stagnation_temperature_mach "T=220 K" "gamma=1.4 1" "M=2 1"
stagnation (total) temperature (T0) = 396 K
  by: Stagnation temperature from the Mach number  (T0 = T*(1 + (gamma - 1)*M**2/2))
  T = 220 K   [220 K]
  gamma = 1.4   [1.4 1]
  M = 2   [2 1]
  assumes: ideal gas with constant specific heats; adiabatic deceleration; the situation is: compressible
```

**What the worked example means:** This calculation finds the stagnation temperature, which represents the total thermal energy of the fluid when brought to rest adiabatically. It uses the static temperature 220 K and the Mach number 2 under the assumption of an ideal gas with constant specific heats. The resulting value 396 K indicates the maximum temperature the flow can reach without heat addition, serving as a critical reference for assessing engine performance and aerodynamic efficiency.

<a id="normal_shock_pressure_air"></a>

#### Static pressure behind a normal shock in air, `normal_shock_pressure_air`

One equation links static pressure behind the shock (P2), static pressure ahead of the shock (P1) and Mach number ahead of the shock (M1). Give all but one of them and it is solved for the one left out, by default static pressure behind the shock (P2).

$$
P_{2} = \frac{P_{1} \left(7 M_{1}^{2} - 1\right)}{6}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P1` | in | static pressure ahead of the shock | Pa |
| `M1` | in | Mach number ahead of the shock | - |
| `P2` | out | static pressure behind the shock | Pa |

**Applies when:** air as an ideal gas with gamma = 1.4; steady normal shock; adiabatic; a stationary normal shock with supersonic upstream flow.

```shell
$ thermofluids solve normal_shock_pressure_air "P1=100000 Pa" "M1=2 1"
static pressure behind the shock (P2) = 450000 Pa
  by: Static pressure behind a normal shock in air  (P2 = P1*(7*M1**2 - 1)/6)
  P1 = 100000 Pa   [100000 Pa]
  M1 = 2   [2 1]
  assumes: air as an ideal gas with gamma = 1.4; steady normal shock; adiabatic; the situation is: shock, air
```

**What the worked example means:** This calculation determines the static pressure immediately behind a normal shock wave in air. It uses the known static pressure ahead of the shock, 100000 Pa, and the upstream Mach number, 2, to find the resulting downstream pressure, 450000 Pa. This value represents the sudden rise in pressure that occurs when supersonic flow passes through the shock, which is critical for assessing structural loads on components like intakes and diffusers.

### Conduction (8)

<a id="critical_insulation_radius"></a>

#### Critical radius of insulation on a cylinder, `critical_insulation_radius`

One equation links critical outer radius of the insulation (r), thermal conductivity (k) and outside heat transfer coefficient (h). Give all but one of them and it is solved for the one left out, by default critical outer radius of the insulation (r).

$$
r = \frac{k}{h}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `k` | in | thermal conductivity | W/(m*K) |
| `h` | in | outside heat transfer coefficient | W/(m^2*K) |
| `r` | out | critical outer radius of the insulation | m |

**Applies when:** cylindrical insulation; constant outside heat transfer coefficient; no radiation; deciding whether insulating a small pipe or wire increases heat loss.

```shell
$ thermofluids solve critical_insulation_radius "k=0.05 W/(m*K)" "h=10 W/(m^2*K)"
critical outer radius of the insulation (r) = 0.005 m
  by: Critical radius of insulation on a cylinder  (r = k/h)
  k = 0.05 W/(m*K)   [0.05 W/(m*K)]
  h = 10 W/(m^2*K)   [10 W/(m^2*K)]
  assumes: cylindrical insulation; constant outside heat transfer coefficient; no radiation; the situation is: critical insulation
```

**What the worked example means:** This calculation determines the critical outer radius of insulation on a cylinder, denoted as 0.005 m. It is derived from the thermal conductivity of the material and the outside heat transfer coefficient. If the actual radius of the pipe or wire is smaller than this computed value, adding insulation will initially increase heat loss rather than reduce it. This result helps decide whether insulating small-diameter components is beneficial for reducing energy loss.

<a id="plane_wall_conduction"></a>

#### Fourier's law: heat conduction through a plane wall, `plane_wall_conduction`

One equation links rate of heat transfer (q), thermal conductivity (k), wall area (A), inner surface temperature (T1), outer surface temperature (T2) and wall thickness (L). Give all but one of them and it is solved for the one left out, by default rate of heat transfer (q).

$$
q = \frac{k A \left(T_{1} - T_{2}\right)}{L}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `k` | in | thermal conductivity | W/(m*K) |
| `A` | in | wall area | m^2 |
| `T1` | in | inner surface temperature | K |
| `T2` | in | outer surface temperature | K |
| `L` | in | wall thickness | m |
| `q` | out | rate of heat transfer | W |

**Applies when:** steady one-dimensional conduction; constant thermal conductivity; no heat generation; a flat wall, slab or window with known surface temperatures.

```shell
$ thermofluids solve plane_wall_conduction "k=0.7 W/(m*K)" "A=10 m^2" "T1=293.15 K" "T2=278.15 K" "L=0.2 m"
rate of heat transfer (q) = 525 W
  by: Fourier's law: heat conduction through a plane wall  (q = k*A*(T1 - T2)/L)
  k = 0.7 W/(m*K)   [0.7 W/(m*K)]
  A = 10 m^2   [10 m^2]
  T1 = 293.15 K   [293.15 K]
  T2 = 278.15 K   [278.15 K]
  L = 0.2 m   [0.2 m]
  assumes: steady one-dimensional conduction; constant thermal conductivity; no heat generation; the situation is: conduction, plane wall
```

**What the worked example means:** This calculation determines the steady-state heat transfer rate through a plane wall using Fourier's law. The result, 525 W, depends on the wall area 10 m^2, the thickness 0.2 m, and the temperature difference between the inner surface 293.15 K and outer surface 278.15 K. It assumes constant thermal conductivity 0.7 W/(m*K) and no internal heat generation, making it suitable for analyzing insulation or building envelopes under stable conditions.

<a id="overall_heat_rate"></a>

#### Heat transfer rate from the overall heat transfer coefficient, `overall_heat_rate`

One equation links rate of heat transfer (q), overall heat transfer coefficient (U), wall area (A), inside fluid temperature (Ti) and outside fluid temperature (To). Give all but one of them and it is solved for the one left out, by default rate of heat transfer (q).

$$
q = U A \left(T_{i} - T_{o}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `U` | in | overall heat transfer coefficient | W/(m^2*K) |
| `A` | in | wall area | m^2 |
| `Ti` | in | inside fluid temperature | K |
| `To` | in | outside fluid temperature | K |
| `q` | out | rate of heat transfer | W |

**Applies when:** steady heat flow; U based on the stated area; building envelopes and walls between two fluids.

```shell
$ thermofluids solve overall_heat_rate "U=0.5 W/(m^2*K)" "A=20 m^2" "Ti=294.15 K" "To=268.15 K"
rate of heat transfer (q) = 260 W
  by: Heat transfer rate from the overall heat transfer coefficient  (q = U*A*(Ti - To))
  U = 0.5 W/(m^2*K)   [0.5 W/(m^2*K)]
  A = 20 m^2   [20 m^2]
  Ti = 294.15 K   [294.15 K]
  To = 268.15 K   [268.15 K]
  assumes: steady heat flow; U based on the stated area; the situation is: overall
```

**What the worked example means:** This calculation determines the total thermal power passing through a surface. It uses the overall heat transfer coefficient, which accounts for conduction and convection resistances, multiplied by the wall area and the temperature difference between the inside and outside fluids. The resulting heat transfer rate of 260 W indicates how much energy is lost or gained per unit time under steady conditions.

<a id="wall_generation_max_temperature"></a>

#### Maximum temperature of a plane wall with uniform heat generation, `wall_generation_max_temperature`

One equation links temperature at the mid-plane (Tmax), surface temperature (Ts), heat generated per unit volume (q), half-thickness of the wall (L) and thermal conductivity (k). Give all but one of them and it is solved for the one left out, by default temperature at the mid-plane (Tmax).

$$
T_{\text{max}} = T_{s} + \frac{q L^{2}}{2 k}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Ts` | in | surface temperature | K |
| `q` | in | heat generated per unit volume | W/m^3 |
| `L` | in | half-thickness of the wall | m |
| `k` | in | thermal conductivity | W/(m*K) |
| `Tmax` | out | temperature at the mid-plane | K |

**Applies when:** steady one-dimensional conduction; both faces at the same temperature; uniform generation; electrically heated slabs and fuel plates.

```shell
$ thermofluids solve wall_generation_max_temperature "Ts=350 K" "q=1000000 W/m^3" "L=0.01 m" "k=20 W/(m*K)"
temperature at the mid-plane (Tmax) = 352.5 K
  by: Maximum temperature of a plane wall with uniform heat generation  (Tmax = Ts + q*L**2/(2*k))
  Ts = 350 K   [350 K]
  q = 1000000 W/m^3   [1e+06 W/m^3]
  L = 0.01 m   [0.01 m]
  k = 20 W/(m*K)   [20 W/(m*K)]
  assumes: steady one-dimensional conduction; both faces at the same temperature; uniform generation; the situation is: heat generation
```

**What the worked example means:** This calculation determines the peak temperature within a plane wall that generates heat uniformly. The result, 352.5 K, represents the temperature at the centerline, derived from the surface temperature 350 K, the volumetric heat generation rate 1e+06 W/m^3, the half-thickness 0.01 m, and the thermal conductivity 20 W/(m*K). This value indicates how much hotter the interior becomes compared to the cooled surfaces due to internal energy production.

<a id="overall_u_plane_wall"></a>

#### Overall heat transfer coefficient of a plane wall with convection on both sides, `overall_u_plane_wall`

One equation links overall heat transfer coefficient (U), inside heat transfer coefficient (h1), wall thickness (L), thermal conductivity (k) and outside heat transfer coefficient (h2). Give all but one of them and it is solved for the one left out, by default overall heat transfer coefficient (U).

$$
U = \frac{1}{\frac{1}{h_{1}} + \frac{L}{k} + \frac{1}{h_{2}}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `h1` | in | inside heat transfer coefficient | W/(m^2*K) |
| `L` | in | wall thickness | m |
| `k` | in | thermal conductivity | W/(m*K) |
| `h2` | in | outside heat transfer coefficient | W/(m^2*K) |
| `U` | out | overall heat transfer coefficient | W/(m^2*K) |

**Applies when:** steady one-dimensional heat flow; a single plane layer; no radiation; walls and windows between two fluids.

```shell
$ thermofluids solve overall_u_plane_wall "h1=10 W/(m^2*K)" "L=0.2 m" "k=0.7 W/(m*K)" "h2=25 W/(m^2*K)"
overall heat transfer coefficient (U) = 2.348993289 W/(m^2*K)
  by: Overall heat transfer coefficient of a plane wall with convection on both sides  (U = 1/(1/h1 + L/k + 1/h2))
  h1 = 10 W/(m^2*K)   [10 W/(m^2*K)]
  L = 0.2 m   [0.2 m]
  k = 0.7 W/(m*K)   [0.7 W/(m*K)]
  h2 = 25 W/(m^2*K)   [25 W/(m^2*K)]
  assumes: steady one-dimensional heat flow; a single plane layer; no radiation; the situation is: overall
```

**What the worked example means:** This calculation determines the overall heat transfer coefficient U for a plane wall with convection on both sides. The result, 2.34899 W/(m^2*K), represents the total conductance of the system, combining the resistance from the inside fluid film, the solid wall material, and the outside fluid film. A higher value indicates that heat passes through the barrier more easily, while a lower value signifies better insulation performance.

<a id="cylinder_conduction"></a>

#### Radial heat conduction through a cylindrical wall, `cylinder_conduction`

One equation links rate of heat transfer (q), thermal conductivity (k), length of the cylinder (L), inner surface temperature (T1), outer surface temperature (T2), inner radius (r1) and outer radius (r2). Give all but one of them and it is solved for the one left out, by default rate of heat transfer (q).

$$
q = \frac{2 \pi k L \left(T_{1} - T_{2}\right)}{\ln\left(\frac{r_{2}}{r_{1}}\right)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `k` | in | thermal conductivity | W/(m*K) |
| `L` | in | length of the cylinder | m |
| `T1` | in | inner surface temperature | K |
| `T2` | in | outer surface temperature | K |
| `r1` | in | inner radius | m |
| `r2` | in | outer radius | m |
| `q` | out | rate of heat transfer | W |

**Applies when:** steady radial conduction; constant thermal conductivity; no heat generation; pipe walls and pipe insulation.

```shell
$ thermofluids solve cylinder_conduction "k=45 W/(m*K)" "L=5 m" "T1=393.15 K" "T2=392.15 K" "r1=0.05 m" "r2=0.06 m"
rate of heat transfer (q) = 7753.974456 W
  by: Radial heat conduction through a cylindrical wall  (q = 2*pi*k*L*(T1 - T2)/log(r2/r1))
  k = 45 W/(m*K)   [45 W/(m*K)]
  L = 5 m   [5 m]
  T1 = 393.15 K   [393.15 K]
  T2 = 392.15 K   [392.15 K]
  r1 = 0.05 m   [0.05 m]
  r2 = 0.06 m   [0.06 m]
  assumes: steady radial conduction; constant thermal conductivity; no heat generation; the situation is: conduction, cylindrical
```

**What the worked example means:** This calculation determines the steady-state radial heat flow through a cylindrical wall, such as a pipe or insulated cable. The result of 7753.97 W represents the total power transferred across the wall thickness. This value depends on the thermal conductivity of the material, the length of the cylinder, and the logarithmic ratio of the outer to inner radii, which accounts for the changing surface area as heat moves outward from the center.

<a id="sphere_conduction"></a>

#### Radial heat conduction through a spherical shell, `sphere_conduction`

One equation links rate of heat transfer (q), thermal conductivity (k), inner surface temperature (T1), outer surface temperature (T2), inner radius (r1) and outer radius (r2). Give all but one of them and it is solved for the one left out, by default rate of heat transfer (q).

$$
q = \frac{4 \pi k \left(T_{1} - T_{2}\right)}{\frac{1}{r_{1}} - \frac{1}{r_{2}}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `k` | in | thermal conductivity | W/(m*K) |
| `T1` | in | inner surface temperature | K |
| `T2` | in | outer surface temperature | K |
| `r1` | in | inner radius | m |
| `r2` | in | outer radius | m |
| `q` | out | rate of heat transfer | W |

**Applies when:** steady radial conduction; constant thermal conductivity; spherical tanks and their insulation.

```shell
$ thermofluids solve sphere_conduction "k=0.04 W/(m*K)" "T1=400 K" "T2=300 K" "r1=0.1 m" "r2=0.15 m"
rate of heat transfer (q) = 15.07964474 W
  by: Radial heat conduction through a spherical shell  (q = 4*pi*k*(T1 - T2)/(1/r1 - 1/r2))
  k = 0.04 W/(m*K)   [0.04 W/(m*K)]
  T1 = 400 K   [400 K]
  T2 = 300 K   [300 K]
  r1 = 0.1 m   [0.1 m]
  r2 = 0.15 m   [0.15 m]
  assumes: steady radial conduction; constant thermal conductivity; the situation is: conduction, spherical
```

**What the worked example means:** This calculation determines the steady-state radial heat conduction through a spherical shell. The result, 15.0796 W, represents the total power transferred from the inner surface at temperature 400 K to the outer surface at temperature 300 K. This value depends on the material's thermal conductivity, 0.04 W/(m*K), and the geometric resistance defined by the inner radius 0.1 m and outer radius 0.15 m. A higher temperature difference or greater conductivity increases the heat flow, while larger radii generally reduce the thermal resistance in this specific configuration.

<a id="thermal_diffusivity"></a>

#### Thermal diffusivity: `thermal_diffusivity`

One equation links thermal diffusivity (alpha), thermal conductivity (k), density of the material (rho) and specific heat capacity (cp). Give all but one of them and it is solved for the one left out, by default thermal diffusivity (alpha).

$$
\alpha = \frac{k}{\rho c_{p}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `k` | in | thermal conductivity | W/(m*K) |
| `rho` | in | density of the material | kg/m^3 |
| `cp` | in | specific heat capacity | J/(kg*K) |
| `alpha` | out | thermal diffusivity | m^2/s |

**Applies when:** constant properties; transient conduction in solids and fluids.

```shell
$ thermofluids solve thermal_diffusivity "k=400 W/(m*K)" "rho=8900 kg/m^3" "cp=385 J/(kg*K)"
thermal diffusivity (alpha) = 0.0001167371954 m^2/s
  by: Thermal diffusivity  (alpha = k/(rho*cp))
  k = 400 W/(m*K)   [400 W/(m*K)]
  rho = 8900 kg/m^3   [8900 kg/m^3]
  cp = 385 J/(kg*K)   [385 J/(kg*K)]
  assumes: constant properties; the situation is: diffusivity
```

**What the worked example means:** This calculation determines the thermal diffusivity, represented by alpha, using the formula where alpha equals k divided by the product of rho and cp. The result, 0.000116737 m^2/s, indicates how rapidly temperature changes propagate through the material during transient conduction. It reflects the ratio of the material's ability to conduct heat, defined by its thermal conductivity k with value 400 W/(m*K), to its capacity to store thermal energy, determined by density rho at 8900 kg/m^3 and specific heat capacity cp at 385 J/(kg*K). A higher value suggests that heat diffuses quickly relative to the amount of energy stored in the material.

### Convection (10)

<a id="flat_plate_laminar_average_nusselt"></a>

#### Average Nusselt number for laminar flow over a flat plate, `flat_plate_laminar_average_nusselt`

One equation links average Nusselt number over the plate (Nu), Reynolds number based on the plate length (ReL) and Prandtl number of the fluid (Pr). Give all but one of them and it is solved for the one left out, by default average Nusselt number over the plate (Nu).

$$
\mathrm{Nu} = 0.664 \text{ReL}^{\frac{1}{2}} \mathrm{Pr}^{\frac{1}{3}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `ReL` | in | Reynolds number based on the plate length | - |
| `Pr` | in | Prandtl number of the fluid | - |
| `Nu` | out | average Nusselt number over the plate | - |

**Applies when:** laminar boundary layer over the whole plate; isothermal plate; Re_L < 500000 and 0.6 < Pr < 10.

```shell
$ thermofluids solve flat_plate_laminar_average_nusselt "ReL=100000 1" "Pr=0.7 1"
average Nusselt number over the plate (Nu) = 186.4378529
  by: Average Nusselt number for laminar flow over a flat plate  (Nu = 0.664*ReL**(1/2)*Pr**(1/3))
  ReL = 100000   [100000 1]
  Pr = 0.7   [0.7 1]
  assumes: laminar boundary layer over the whole plate; isothermal plate; the situation is: flat plate, laminar
```

**What the worked example means:** This calculation determines the average Nusselt number, which represents the ratio of convective to conductive heat transfer across the boundary layer. It is derived from the Reynolds number based on the plate length, 100000, and the Prandtl number of the fluid, 0.7. A higher value indicates more effective convective cooling or heating relative to pure conduction.

<a id="churchill_bernstein_cylinder"></a>

#### Churchill-Bernstein correlation for a cylinder in cross flow, `churchill_bernstein_cylinder`

Average Nusselt number of a circular cylinder in cross flow over the whole Reynolds range.

$$
Nu = 0.3 + \frac{0.62\,Re^{1/2}Pr^{1/3}}{[1 + (0.4/Pr)^{2/3}]^{1/4}}\left[1 + \left(\frac{Re}{282000}\right)^{5/8}\right]^{4/5}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Re` | in | Reynolds number based on the diameter | - |
| `Pr` | in | Prandtl number | - |
| `Nu` | out | average Nusselt number based on the diameter | - |

**Applies when:** Re * Pr >= 0.2.

```shell
$ thermofluids solve churchill_bernstein_cylinder Re=10000 Pr=0.7
Nu = 53.32778867
  by: Churchill-Bernstein correlation for a cylinder in cross flow  (Nu = 0.3 + 0.62 Re^(1/2) Pr^(1/3) / (1 + (0.4/Pr)^(2/3))^(1/4) (1 + (Re/282000)^(5/8))^(4/5))
  Re = 10000
  Pr = 0.7
  assumes: Re * Pr >= 0.2
```

**What the worked example means:** This calculation determines the average heat transfer performance of a circular cylinder in cross flow using the Churchill-Bernstein correlation. By inputting the Reynolds number based on diameter, which is 10000, and the fluid's Prandtl number, which is 0.7, the model computes the dimensionless average Nusselt number. The resulting value of 53.3278 represents the ratio of convective to conductive heat transfer across the boundary layer surrounding the cylinder, valid for the entire range from laminar to turbulent flow regimes.

<a id="churchill_chu_vertical_plate"></a>

#### Churchill-Chu correlation for natural convection on a vertical plate, `churchill_chu_vertical_plate`

Average Nusselt number of free convection from an isothermal vertical plate, laminar and turbulent.

$$
Nu = \left(0.825 + \frac{0.387\,Ra^{1/6}}{[1 + (0.492/Pr)^{9/16}]^{8/27}}\right)^2
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Ra` | in | Rayleigh number based on the plate height | - |
| `Pr` | in | Prandtl number | - |
| `Nu` | out | average Nusselt number | - |

**Applies when:** 0 < Ra <= 1e13.

```shell
$ thermofluids solve churchill_chu_vertical_plate Ra=1000000000 Pr=0.71
Nu = 122.8565349
  by: Churchill-Chu correlation for natural convection on a vertical plate  (Nu = (0.825 + 0.387 Ra^(1/6) / (1 + (0.492/Pr)^(9/16))^(8/27))^2)
  Ra = 1000000000
  Pr = 0.71
  assumes: 0 < Ra <= 1e13
```

**What the worked example means:** This calculation determines the average heat transfer rate from an isothermal vertical plate using the Churchill-Chu correlation. By inputting a Rayleigh number of 1e+09 and a Prandtl number of 0.71, the method yields an average Nusselt number of 122.857. This result represents the ratio of convective to conductive heat transfer across the boundary layer, valid for both laminar and turbulent natural convection regimes within the specified range.

<a id="dittus_boelter_cooling"></a>

#### Dittus-Boelter correlation, fluid being cooled, `dittus_boelter_cooling`

One equation links Nusselt number (Nu), Reynolds number (Re) and Prandtl number of the fluid (Pr). Give all but one of them and it is solved for the one left out, by default Nusselt number (Nu).

$$
\mathrm{Nu} = 0.023 \mathrm{Re}^{\frac{4}{5}} \mathrm{Pr}^{\frac{3}{10}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Re` | in | Reynolds number | - |
| `Pr` | in | Prandtl number of the fluid | - |
| `Nu` | out | Nusselt number | - |

**Applies when:** fully developed turbulent flow in a smooth circular tube; properties at the bulk temperature; Re > 10000 and 0.6 < Pr < 160.

```shell
$ thermofluids solve dittus_boelter_cooling "Re=50000 1" "Pr=5 1"
Nusselt number (Nu) = 214.0892402
  by: Dittus-Boelter correlation, fluid being cooled  (Nu = 0.023*Re**(4/5)*Pr**(3/10))
  Re = 50000   [50000 1]
  Pr = 5   [5 1]
  assumes: fully developed turbulent flow in a smooth circular tube; properties at the bulk temperature; the situation is: dittus boelter, cooled fluid
```

**What the worked example means:** The Dittus-Boelter correlation calculates the Nusselt number as 214.089 using the Reynolds number 50000 and the Prandtl number 5. This result quantifies the ratio of convective to conductive heat transfer across the boundary layer when the fluid is being cooled. A higher value signifies more effective heat removal from the fluid to the pipe wall under turbulent flow conditions.

<a id="dittus_boelter_heating"></a>

#### Dittus-Boelter correlation, fluid being heated, `dittus_boelter_heating`

One equation links Nusselt number (Nu), Reynolds number (Re) and Prandtl number of the fluid (Pr). Give all but one of them and it is solved for the one left out, by default Nusselt number (Nu).

$$
\mathrm{Nu} = 0.023 \mathrm{Re}^{\frac{4}{5}} \mathrm{Pr}^{\frac{2}{5}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Re` | in | Reynolds number | - |
| `Pr` | in | Prandtl number of the fluid | - |
| `Nu` | out | Nusselt number | - |

**Applies when:** fully developed turbulent flow in a smooth circular tube; properties at the bulk temperature; Re > 10000 and 0.6 < Pr < 160.

```shell
$ thermofluids solve dittus_boelter_heating "Re=50000 1" "Pr=5 1"
Nusselt number (Nu) = 251.473277
  by: Dittus-Boelter correlation, fluid being heated  (Nu = 0.023*Re**(4/5)*Pr**(2/5))
  Re = 50000   [50000 1]
  Pr = 5   [5 1]
  assumes: fully developed turbulent flow in a smooth circular tube; properties at the bulk temperature; the situation is: dittus boelter, heated fluid
```

**What the worked example means:** This result determines the Nusselt number, which characterizes the ratio of convective to conductive heat transfer across the boundary. It was computed using the Dittus-Boelter correlation for a fluid being heated in fully developed turbulent flow within a smooth circular tube. The value 251.473 suggests that convection dominates over pure conduction, driven by the Reynolds number 50000 and Prandtl number 5. This estimate assumes properties are evaluated at the bulk temperature and applies only when the flow is turbulent with moderate property variations.

<a id="gnielinski_nusselt"></a>

#### Gnielinski correlation for turbulent pipe flow, `gnielinski_nusselt`

Nusselt number of fully developed turbulent or transitional flow in a tube.

$$
Nu = \frac{(f/8)(Re - 1000)Pr}{1 + 12.7\sqrt{f/8}\,(Pr^{2/3} - 1)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Re` | in | Reynolds number (3000 to 5e6) | - |
| `Pr` | in | Prandtl number (0.5 to 2000) | - |
| `f` | in | Darcy friction factor | - |
| `Nu` | out | Nusselt number based on the diameter | - |

**Applies when:** 3000 <= Re <= 5e6; 0.5 <= Pr <= 2000.

```shell
$ thermofluids solve gnielinski_nusselt Re=50000 Pr=5 f=0.021
Nu = 285.5893125
  by: Gnielinski correlation for turbulent pipe flow  (Nu = (f/8)(Re - 1000)Pr / (1 + 12.7 sqrt(f/8)(Pr^(2/3) - 1)))
  Re = 50000
  Pr = 5
  f = 0.021
  assumes: 3000 <= Re <= 5e6; 0.5 <= Pr <= 2000
```

**What the worked example means:** This calculation determines the Nusselt number, which quantifies the ratio of convective to conductive heat transfer across the boundary. It uses the Gnielinski correlation, combining the Darcy friction factor 0.021, the Reynolds number 50000, and the Prandtl number 5. The resulting value of 285.589 suggests that as fluid turbulence or thermal diffusivity changes, the efficiency of heat exchange within the pipe adjusts accordingly, provided the flow remains in the turbulent or transitional regime.

<a id="nusselt_heat_transfer_coefficient"></a>

#### Heat transfer coefficient from the Nusselt number, `nusselt_heat_transfer_coefficient`

One equation links convection heat transfer coefficient (h), average Nusselt number (Nu), thermal conductivity (k) and characteristic length (diameter or plate length) (Lc). Give all but one of them and it is solved for the one left out, by default convection heat transfer coefficient (h).

$$
h = \frac{\mathrm{Nu} k}{L_{c}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Nu` | in | average Nusselt number | - |
| `k` | in | thermal conductivity | W/(m*K) |
| `Lc` | in | characteristic length (diameter or plate length) | m |
| `h` | out | convection heat transfer coefficient | W/(m^2*K) |

**Applies when:** Nu based on the same characteristic length; k of the fluid; turning a convection correlation's Nusselt number into h.

```shell
$ thermofluids solve nusselt_heat_transfer_coefficient "Nu=120 1" "k=0.6 W/(m*K)" "Lc=0.05 m"
convection heat transfer coefficient (h) = 1440 W/(m^2*K)
  by: Heat transfer coefficient from the Nusselt number  (h = Nu*k/Lc)
  Nu = 120   [120 1]
  k = 0.6 W/(m*K)   [0.6 W/(m*K)]
  Lc = 0.05 m   [0.05 m]
  assumes: Nu based on the same characteristic length; k of the fluid; the situation is: nusselt
```

**What the worked example means:** This calculation determines the convective heat transfer coefficient by combining the dimensionless average Nusselt number with the thermal conductivity of the fluid and a specific characteristic length. The result, 1440 W/(m^2*K), represents the effectiveness of heat transfer between the surface and the moving fluid. By using the fluid's conductivity rather than the solid's, this value correctly translates correlation data into practical engineering units for analyzing convection problems.

<a id="flat_plate_laminar_local_nusselt"></a>

#### Local Nusselt number for laminar flow over a flat plate, `flat_plate_laminar_local_nusselt`

One equation links local Nusselt number (Nux), local Reynolds number U x / nu (Rex) and Prandtl number of the fluid (Pr). Give all but one of them and it is solved for the one left out, by default local Nusselt number (Nux).

$$
\text{Nux} = 0.332 \text{Rex}^{\frac{1}{2}} \mathrm{Pr}^{\frac{1}{3}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Rex` | in | local Reynolds number U x / nu | - |
| `Pr` | in | Prandtl number of the fluid | - |
| `Nux` | out | local Nusselt number | - |

**Applies when:** laminar boundary layer; isothermal plate; Re_x < 500000 and 0.6 < Pr < 10.

```shell
$ thermofluids solve flat_plate_laminar_local_nusselt "Rex=100000 1" "Pr=0.7 1"
local Nusselt number (Nux) = 93.21892644
  by: Local Nusselt number for laminar flow over a flat plate  (Nux = 0.332*Rex**(1/2)*Pr**(1/3))
  Rex = 100000   [100000 1]
  Pr = 0.7   [0.7 1]
  assumes: laminar boundary layer; isothermal plate; the situation is: flat plate, laminar
```

**What the worked example means:** This calculation determines the local Nusselt number, which quantifies the convective heat transfer rate relative to conduction at a specific distance from the leading edge of a flat plate. By combining the local Reynolds number and the fluid's Prandtl number using the laminar boundary layer correlation, the result 93.2189 reflects how effectively heat is transferred between the isothermal surface and the flowing fluid under these conditions.

<a id="newton_cooling"></a>

#### Newton's law of cooling (convection heat transfer), `newton_cooling`

One equation links rate of heat transfer (q), convection heat transfer coefficient (h), surface area in contact with the fluid (A), surface temperature (Ts) and temperature of the fluid far from the surface (Tinf). Give all but one of them and it is solved for the one left out, by default rate of heat transfer (q).

$$
q = h A \left(T_{s} - \text{Tinf}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `h` | in | convection heat transfer coefficient | W/(m^2*K) |
| `A` | in | surface area in contact with the fluid | m^2 |
| `Ts` | in | surface temperature | K |
| `Tinf` | in | temperature of the fluid far from the surface | K |
| `q` | out | rate of heat transfer | W |

**Applies when:** uniform surface temperature; h averaged over the surface; heat exchange between a surface and a moving or still fluid.

```shell
$ thermofluids solve newton_cooling "h=25 W/(m^2*K)" "A=0.5 m^2" "Ts=353.15 K" "Tinf=293.15 K"
rate of heat transfer (q) = 750 W
  by: Newton's law of cooling (convection heat transfer)  (q = h*A*(Ts - Tinf))
  h = 25 W/(m^2*K)   [25 W/(m^2*K)]
  A = 0.5 m^2   [0.5 m^2]
  Ts = 353.15 K   [353.15 K]
  Tinf = 293.15 K   [293.15 K]
  assumes: uniform surface temperature; h averaged over the surface; the situation is: convection
```

**What the worked example means:** This calculation determines the convective heat transfer rate between a surface and a fluid. It uses the convection coefficient, surface area, and the temperature difference to find that the heat flow is 750 W. This value represents how much thermal energy moves from the solid to the surrounding fluid per unit of time.

<a id="prandtl_number"></a>

#### Prandtl number: `prandtl_number`

One equation links Prandtl number of the fluid (Pr), specific heat of the fluid (cp), dynamic viscosity of the fluid (mu) and thermal conductivity (k). Give all but one of them and it is solved for the one left out, by default Prandtl number of the fluid (Pr).

$$
\mathrm{Pr} = \frac{c_{p} \mu}{k}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `cp` | in | specific heat of the fluid | J/(kg*K) |
| `mu` | in | dynamic viscosity of the fluid | Pa*s |
| `k` | in | thermal conductivity | W/(m*K) |
| `Pr` | out | Prandtl number of the fluid | - |

**Applies when:** properties at the film temperature; any Newtonian fluid.

```shell
$ thermofluids solve prandtl_number "cp=4180 J/(kg*K)" "mu=0.001 Pa*s" "k=0.6 W/(m*K)"
Prandtl number of the fluid (Pr) = 6.966666667
  by: Prandtl number  (Pr = cp*mu/k)
  cp = 4180 J/(kg*K)   [4180 J/(kg*K)]
  mu = 0.001 Pa*s   [0.001 Pa*s]
  k = 0.6 W/(m*K)   [0.6 W/(m*K)]
  assumes: properties at the film temperature; the situation is: prandtl
```

**What the worked example means:** This calculation determines the Prandtl number, a dimensionless ratio that compares momentum diffusivity to thermal diffusivity. It is derived by multiplying the fluid's specific heat 4180 J/(kg*K) by its dynamic viscosity 0.001 Pa*s and dividing the result by the thermal conductivity 0.6 W/(m*K). The final value of 6.96667 indicates how quickly velocity changes propagate through the fluid relative to temperature changes, which helps predict the development of thermal boundary layers.

### External flow (4)

<a id="drag_force"></a>

#### Drag force from the drag coefficient: `drag_force`

One equation links drag force (F), drag coefficient (Cd), density of the fluid (rho), mean flow velocity (v) and frontal (reference) area (A). Give all but one of them and it is solved for the one left out, by default drag force (F).

$$
F = \frac{C_{d} \rho v^{2} A}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Cd` | in | drag coefficient | - |
| `rho` | in | density of the fluid | kg/m^3 |
| `v` | in | mean flow velocity | m/s |
| `A` | in | frontal (reference) area | m^2 |
| `F` | out | drag force | N |

**Applies when:** drag coefficient based on the stated reference area; incompressible flow; a body moving through a fluid with a known drag coefficient.

```shell
$ thermofluids solve drag_force "Cd=0.3 1" "rho=1.2 kg/m^3" "v=30 m/s" "A=2.2 m^2"
drag force (F) = 356.4 N
  by: Drag force from the drag coefficient  (F = Cd*rho*v**2*A/2)
  Cd = 0.3   [0.3 1]
  rho = 1.2 kg/m^3   [1.2 kg/m^3]
  v = 30 m/s   [30 m/s]
  A = 2.2 m^2   [2.2 m^2]
  assumes: drag coefficient based on the stated reference area; incompressible flow; the situation is: drag
```

**What the worked example means:** This calculation determines the resistive force acting on a body moving through a fluid. The result of 356.4 N represents the total drag, derived by combining the fluid density 1.2 kg/m^3, the mean flow velocity 30 m/s, the frontal reference area 2.2 m^2, and the dimensionless drag coefficient 0.3. This value indicates the magnitude of the force opposing motion, which is critical for assessing energy requirements and structural loads in engineering applications.

<a id="lift_force"></a>

#### Lift force from the lift coefficient: `lift_force`

One equation links lift force (F), lift coefficient (CL), density of the fluid (rho), mean flow velocity (v) and wing planform area (A). Give all but one of them and it is solved for the one left out, by default lift force (F).

$$
F = \frac{C_{L} \rho v^{2} A}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `CL` | in | lift coefficient | - |
| `rho` | in | density of the fluid | kg/m^3 |
| `v` | in | mean flow velocity | m/s |
| `A` | in | wing planform area | m^2 |
| `F` | out | lift force | N |

**Applies when:** lift coefficient based on the planform area; incompressible flow; a wing or airfoil with a known lift coefficient.

```shell
$ thermofluids solve lift_force "CL=1.2 1" "rho=1.1 kg/m^3" "v=70 m/s" "A=20 m^2"
lift force (F) = 64680 N
  by: Lift force from the lift coefficient  (F = CL*rho*v**2*A/2)
  CL = 1.2   [1.2 1]
  rho = 1.1 kg/m^3   [1.1 kg/m^3]
  v = 70 m/s   [70 m/s]
  A = 20 m^2   [20 m^2]
  assumes: lift coefficient based on the planform area; incompressible flow; the situation is: lift
```

**What the worked example means:** This calculation determines the aerodynamic lift force acting perpendicular to the flow direction. It uses the formula involving the lift coefficient, fluid density, square of the mean flow velocity, and wing planform area. The resulting force of 64680 N indicates how much upward support the surface generates under these specific incompressible flow conditions.

<a id="stokes_drag"></a>

#### Stokes drag on a small sphere: `stokes_drag`

One equation links drag force on the sphere (F), dynamic viscosity of the fluid (mu), sphere diameter (D) and mean flow velocity (v). Give all but one of them and it is solved for the one left out, by default drag force on the sphere (F).

$$
F = 3 \pi \mu D v
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `mu` | in | dynamic viscosity of the fluid | Pa*s |
| `D` | in | sphere diameter | m |
| `v` | in | mean flow velocity | m/s |
| `F` | out | drag force on the sphere | N |

**Applies when:** creeping flow (Re < 1); rigid sphere far from walls; a small particle or droplet moving slowly through a viscous fluid.

```shell
$ thermofluids solve stokes_drag "mu=0.001 Pa*s" "D=0.0001 m" "v=0.002 m/s"
drag force on the sphere (F) = 1.884955592e-09 N
  by: Stokes drag on a small sphere  (F = 3*pi*mu*D*v)
  mu = 0.001 Pa*s   [0.001 Pa*s]
  D = 0.0001 m   [0.0001 m]
  v = 0.002 m/s   [0.002 m/s]
  assumes: creeping flow (Re < 1); rigid sphere far from walls; the situation is: stokes
```

**What the worked example means:** This calculation determines the viscous drag force acting on a small spherical particle moving through a fluid. The result, 1.88496e-09 N, is derived from the dynamic viscosity of the fluid, 0.001 Pa*s, the diameter of the sphere, 0.0001 m, and its relative velocity, 0.002 m/s. This linear relationship applies when inertial forces are negligible compared to viscous forces, typical in creeping flow conditions where the Reynolds number is less than one.

<a id="stokes_terminal_velocity"></a>

#### Terminal settling velocity of a small sphere (Stokes), `stokes_terminal_velocity`

One equation links terminal settling velocity (v), Acceleration (g), sphere diameter (D), density of the particle (rho_p), density of the fluid (rho) and dynamic viscosity of the fluid (mu). Give all but one of them and it is solved for the one left out, by default terminal settling velocity (v).

$$
v = \frac{g D^{2} \left(\rho_{p} - \rho\right)}{18 \mu}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `g` | in | Acceleration (optional) | m/s^2 |
| `D` | in | sphere diameter | m |
| `rho_p` | in | density of the particle | kg/m^3 |
| `rho` | in | density of the fluid | kg/m^3 |
| `mu` | in | dynamic viscosity of the fluid | Pa*s |
| `v` | out | terminal settling velocity | m/s |

**Applies when:** creeping flow (Re < 1); particle denser than the fluid; rigid sphere; fine particles or droplets settling in a viscous fluid.

```shell
$ thermofluids solve stokes_terminal_velocity "g=9.80665 m/s^2" "D=0.0001 m" "rho_p=2650 kg/m^3" "rho=1000 kg/m^3" "mu=0.001 Pa*s"
terminal settling velocity (v) = 0.008989429167 m/s
  by: Terminal settling velocity of a small sphere (Stokes)  (v = g*D**2*(rho_p - rho)/(18*mu))
  g = 9.80665 m/s^2   [9.80665 m/s^2]
  D = 0.0001 m   [0.0001 m]
  rho_p = 2650 kg/m^3   [2650 kg/m^3]
  rho = 1000 kg/m^3   [1000 kg/m^3]
  mu = 0.001 Pa*s   [0.001 Pa*s]
  assumes: creeping flow (Re < 1); particle denser than the fluid; rigid sphere; the situation is: stokes
```

**What the worked example means:** This calculation determines the steady speed at which a small rigid sphere falls through a viscous fluid, known as the terminal settling velocity. The result, 0.00898943 m/s, depends on the gravitational acceleration, the square of the particle diameter, the difference between the particle and fluid densities, and the fluid's dynamic viscosity. A higher value indicates faster settling, which is typical for larger or denser particles in less viscous fluids.

### Fins (2)

<a id="pin_fin_efficiency"></a>

#### Efficiency of a pin fin with an insulated tip, `pin_fin_efficiency`

Fin efficiency tanh(mL)/(mL) of a cylindrical pin fin with m = sqrt(4h/(kD)).

$$
\eta_f = \frac{\tanh(mL)}{mL},\quad m = \sqrt{\frac{4h}{kD}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `h` | in | heat transfer coefficient | W/(m^2*K) |
| `k` | in | fin thermal conductivity | W/(m*K) |
| `D` | in | pin diameter | m |
| `L` | in | fin length | m |
| `efficiency` | out | fin efficiency | - |
| `mL` | out | fin parameter m L | - |

**Applies when:** h, k, D, L > 0; one-dimensional conduction, uniform h, insulated tip.

```shell
$ thermofluids solve pin_fin_efficiency "h=50 W/(m^2*K)" "k=200 W/(m*K)" "D=5 mm" "L=50 mm"
efficiency = 0.8610571716
mL = 0.7071067812
  by: Efficiency of a pin fin with an insulated tip  (eta = tanh(mL)/(mL), m = sqrt(4h/(kD)))
  h = 50 W/(m^2*K)
  k = 200 W/(m*K)
  D = 0.005 m
  L = 0.05 m
  assumes: h, k, D, L > 0; one-dimensional conduction, uniform h, insulated tip
```

**What the worked example means:** This calculation determines the efficiency of a cylindrical pin fin with an insulated tip. It uses the heat transfer coefficient, thermal conductivity, diameter, and length to find the fin parameter m L, which is 0.707107. The resulting efficiency, 0.861057, indicates how well the fin performs compared to an ideal fin with uniform temperature.

<a id="infinite_fin_heat_rate"></a>

#### Heat transfer rate of a very long fin: `infinite_fin_heat_rate`

One equation links rate of heat transfer (q), convection heat transfer coefficient (h), perimeter of the fin's cross section (P), thermal conductivity (k), cross-sectional area of the fin (Ac), temperature at the fin base (Tb) and temperature of the fluid far from the surface (Tinf). Give all but one of them and it is solved for the one left out, by default rate of heat transfer (q).

$$
q = \sqrt{h P k A_{c}} \left(T_{b} - \text{Tinf}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `h` | in | convection heat transfer coefficient | W/(m^2*K) |
| `P` | in | perimeter of the fin's cross section | m |
| `k` | in | thermal conductivity | W/(m*K) |
| `Ac` | in | cross-sectional area of the fin | m^2 |
| `Tb` | in | temperature at the fin base | K |
| `Tinf` | in | temperature of the fluid far from the surface | K |
| `q` | out | rate of heat transfer | W |

**Applies when:** one-dimensional conduction along the fin; uniform h; constant cross section; very long fin; m L above about 2.65.

```shell
$ thermofluids solve infinite_fin_heat_rate "h=25 W/(m^2*K)" "P=0.0314 m" "k=200 W/(m*K)" "Ac=7.85e-05 m^2" "Tb=373.15 K" "Tinf=298.15 K"
rate of heat transfer (q) = 8.326182348 W
  by: Heat transfer rate of a very long fin  (q = sqrt(h*P*k*Ac)*(Tb - Tinf))
  h = 25 W/(m^2*K)   [25 W/(m^2*K)]
  P = 0.0314 m   [0.0314 m]
  k = 200 W/(m*K)   [200 W/(m*K)]
  Ac = 7.85e-05 m^2   [7.85e-05 m^2]
  Tb = 373.15 K   [373.15 K]
  Tinf = 298.15 K   [298.15 K]
  assumes: one-dimensional conduction along the fin; uniform h; constant cross section; very long fin; the situation is: fin, long fin
```

**What the worked example means:** This calculation determines the heat transfer rate of a very long fin using the formula for infinite length. The result, 8.32618 W, represents the maximum dissipation capacity given the base temperature 373.15 K and fluid temperature 298.15 K. It assumes one-dimensional conduction with uniform convection coefficient 25 W/(m^2*K), perimeter 0.0314 m, conductivity 200 W/(m*K), and cross-sectional area 7.85e-05 m^2.

### Fluid dynamics (4)

<a id="bernoulli_horizontal"></a>

#### Bernoulli's equation along a horizontal streamline, `bernoulli_horizontal`

One equation links pressure at the outlet (P2), pressure at the inlet (P1), density of the fluid (rho), speed at the inlet (v1) and speed at the outlet (v2). Give all but one of them and it is solved for the one left out, by default pressure at the outlet (P2).

$$
P_{2} = P_{1} + \frac{\rho \left(v_{1}^{2} - v_{2}^{2}\right)}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P1` | in | pressure at the inlet | Pa |
| `rho` | in | density of the fluid | kg/m^3 |
| `v1` | in | speed at the inlet | m/s |
| `v2` | in | speed at the outlet | m/s |
| `P2` | out | pressure at the outlet | Pa |

**Applies when:** steady, incompressible, inviscid flow along a streamline; no height change; frictionless flow between two points at the same height.

```shell
$ thermofluids solve bernoulli_horizontal "P1=200000 Pa" "rho=1000 kg/m^3" "v1=2 m/s" "v2=6 m/s"
pressure at the outlet (P2) = 184000 Pa
  by: Bernoulli's equation along a horizontal streamline  (P2 = P1 + rho*(v1**2 - v2**2)/2)
  P1 = 200000 Pa   [200000 Pa]
  rho = 1000 kg/m^3   [1000 kg/m^3]
  v1 = 2 m/s   [2 m/s]
  v2 = 6 m/s   [6 m/s]
  assumes: steady, incompressible, inviscid flow along a streamline; no height change; the situation is: bernoulli, horizontal
```

**What the worked example means:** This calculation finds the static pressure at the outlet, denoted as 184000 Pa, by applying Bernoulli's principle to a horizontal streamline. It starts with the known inlet pressure 200000 Pa and adjusts it based on the change in kinetic energy caused by the difference between the inlet speed 2 m/s and the outlet speed 6 m/s. The fluid density 1000 kg/m^3 scales this dynamic pressure term, ensuring that the result reflects the conversion of velocity head into pressure head under frictionless conditions.

<a id="continuity_speed"></a>

#### Continuity: speed after a change of pipe diameter, `continuity_speed`

One equation links mean velocity at the outlet (v2), mean velocity at the inlet (v1), diameter at the inlet (D1) and diameter at the outlet (D2). Give all but one of them and it is solved for the one left out, by default mean velocity at the outlet (v2).

$$
v_{2} = v_{1} \left(\frac{D_{1}}{D_{2}}\right)^{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `v1` | in | mean velocity at the inlet | m/s |
| `D1` | in | diameter at the inlet | m |
| `D2` | in | diameter at the outlet | m |
| `v2` | out | mean velocity at the outlet | m/s |

**Applies when:** incompressible steady flow; circular cross sections; uniform velocity over each section; liquid flowing through a reducer, nozzle or expansion.

```shell
$ thermofluids solve continuity_speed "v1=2 m/s" "D1=0.1 m" "D2=0.05 m"
mean velocity at the outlet (v2) = 8 m/s
  by: Continuity: speed after a change of pipe diameter  (v2 = v1*(D1/D2)**2)
  v1 = 2 m/s   [2 m/s]
  D1 = 0.1 m   [0.1 m]
  D2 = 0.05 m   [0.05 m]
  assumes: incompressible steady flow; circular cross sections; uniform velocity over each section; the situation is: continuity
```

**What the worked example means:** This calculation determines the new fluid speed after passing through a section with a different pipe diameter. Using the principle of continuity for incompressible flow, the outlet velocity 8 m/s is derived from the known inlet velocity 2 m/s and the ratio of the diameters 0.1 m and 0.05 m. A reduction in diameter causes the fluid to accelerate to maintain mass conservation, while an expansion causes it to decelerate.

<a id="dynamic_pressure"></a>

#### Dynamic pressure of a moving fluid: `dynamic_pressure`

One equation links dynamic pressure (q), density of the fluid (rho) and mean flow velocity (v). Give all but one of them and it is solved for the one left out, by default dynamic pressure (q).

$$
q = \frac{\rho v^{2}}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rho` | in | density of the fluid | kg/m^3 |
| `v` | in | mean flow velocity | m/s |
| `q` | out | dynamic pressure | Pa |

**Applies when:** incompressible flow (low Mach number); a pitot-static measurement or the kinetic pressure of a stream.

```shell
$ thermofluids solve dynamic_pressure "rho=1.225 kg/m^3" "v=40 m/s"
dynamic pressure (q) = 980 Pa
  by: Dynamic pressure of a moving fluid  (q = rho*v**2/2)
  rho = 1.225 kg/m^3   [1.225 kg/m^3]
  v = 40 m/s   [40 m/s]
  assumes: incompressible flow (low Mach number); the situation is: dynamic pressure
```

**What the worked example means:** This calculation determines the dynamic pressure, which represents the kinetic energy per unit volume of a moving fluid. Using the fluid density 1.225 kg/m^3 and the mean flow velocity 40 m/s, the resulting dynamic pressure is 980 Pa. This value indicates the pressure increase that would occur if the fluid were brought to rest isentropically, a key concept in aerodynamic measurements.

<a id="torricelli_efflux"></a>

#### Speed of efflux from a hole (Torricelli's law), `torricelli_efflux`

One equation links speed of the jet leaving the hole (v), Acceleration (g) and depth of the hole below the free surface (h). Give all but one of them and it is solved for the one left out, by default speed of the jet leaving the hole (v).

$$
v = \sqrt{2 g h}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `g` | in | Acceleration (optional) | m/s^2 |
| `h` | in | depth of the hole below the free surface | m |
| `v` | out | speed of the jet leaving the hole | m/s |

**Applies when:** large tank (surface speed negligible); no losses at the hole; open to the atmosphere; liquid draining from a small hole in a large open tank.

```shell
$ thermofluids solve torricelli_efflux "g=9.80665 m/s^2" "h=2 m"
speed of the jet leaving the hole (v) = 6.263114241 m/s
  by: Speed of efflux from a hole (Torricelli's law)  (v = sqrt(2*g*h))
  g = 9.80665 m/s^2   [9.80665 m/s^2]
  h = 2 m   [2 m]
  assumes: large tank (surface speed negligible); no losses at the hole; open to the atmosphere; the situation is: torricelli
```

**What the worked example means:** This calculation determines the exit velocity of a liquid draining from a tank through a small hole. It uses the depth of the hole below the free surface, given as 2 m, and the acceleration due to gravity, 9.80665 m/s^2, to find that the speed of the jet leaving the hole is 6.26311 m/s. This result represents the theoretical maximum speed assuming no energy losses and a large tank where the surface movement is negligible.

### Fluid statics (4)

<a id="absolute_pressure_depth"></a>

#### Absolute pressure at a depth in a liquid: `absolute_pressure_depth`

One equation links absolute pressure at the depth (P), pressure at the free surface (P0), density of the fluid (rho), Acceleration (g) and depth below the free surface (h). Give all but one of them and it is solved for the one left out, by default absolute pressure at the depth (P).

$$
P = P_{0} + \rho g h
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P0` | in | pressure at the free surface | Pa |
| `rho` | in | density of the fluid | kg/m^3 |
| `g` | in | Acceleration (optional) | m/s^2 |
| `h` | in | depth below the free surface | m |
| `P` | out | absolute pressure at the depth | Pa |

**Applies when:** incompressible liquid at rest; uniform gravity; a point at a depth below a free surface with a known surface pressure.

```shell
$ thermofluids solve absolute_pressure_depth "P0=101325 Pa" "rho=1025 kg/m^3" "g=9.80665 m/s^2" "h=20 m"
absolute pressure at the depth (P) = 302361.325 Pa
  by: Absolute pressure at a depth in a liquid  (P = P0 + rho*g*h)
  P0 = 101325 Pa   [101325 Pa]
  rho = 1025 kg/m^3   [1025 kg/m^3]
  g = 9.80665 m/s^2   [9.80665 m/s^2]
  h = 20 m   [20 m]
  assumes: incompressible liquid at rest; uniform gravity; the situation is: hydrostatic
```

**What the worked example means:** This calculation determines the total pressure acting on a point beneath a liquid surface by combining the surface pressure with the hydrostatic contribution from the fluid's weight. Using the formula P = P0 + rho*g*h, where P0 represents the pressure at the free surface, rho is the density of the fluid, g is the acceleration due to gravity, and h is the depth below the free surface, the model computes the resulting absolute pressure. For the given inputs, the calculated absolute pressure is 302361 Pa, which reflects the cumulative stress state required for applications such as diving calculations or sealed vessel design.

<a id="buoyant_force"></a>

#### Buoyant force (Archimedes' principle): `buoyant_force`

One equation links buoyant force (F), density of the fluid (rho), Acceleration (g) and volume of fluid displaced (V). Give all but one of them and it is solved for the one left out, by default buoyant force (F).

$$
F = \rho g V
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rho` | in | density of the fluid | kg/m^3 |
| `g` | in | Acceleration (optional) | m/s^2 |
| `V` | in | volume of fluid displaced | m^3 |
| `F` | out | buoyant force | N |

**Applies when:** fluid at rest; uniform gravity; uniform fluid density; a body wholly or partly immersed in a fluid at rest.

```shell
$ thermofluids solve buoyant_force "rho=1000 kg/m^3" "g=9.80665 m/s^2" "V=0.02 m^3"
buoyant force (F) = 196.133 N
  by: Buoyant force (Archimedes' principle)  (F = rho*g*V)
  rho = 1000 kg/m^3   [1000 kg/m^3]
  g = 9.80665 m/s^2   [9.80665 m/s^2]
  V = 0.02 m^3   [0.02 m^3]
  assumes: fluid at rest; uniform gravity; uniform fluid density; the situation is: buoyancy
```

**What the worked example means:** This calculation determines the upward buoyant force acting on an immersed object using Archimedes' principle. The result, 196.133 N, represents the weight of the fluid displaced by the body. It depends directly on the fluid's density, 1000 kg/m^3, the local acceleration due to gravity, 9.80665 m/s^2, and the volume of fluid displaced, 0.02 m^3. This force explains why objects float or sink in a stationary fluid.

<a id="hydrostatic_gauge_pressure"></a>

#### Gauge pressure at a depth in a liquid: `hydrostatic_gauge_pressure`

One equation links gauge pressure at the depth (P), density of the fluid (rho), Acceleration (g) and depth below the free surface (h). Give all but one of them and it is solved for the one left out, by default gauge pressure at the depth (P).

$$
P = \rho g h
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rho` | in | density of the fluid | kg/m^3 |
| `g` | in | Acceleration (optional) | m/s^2 |
| `h` | in | depth below the free surface | m |
| `P` | out | gauge pressure at the depth | Pa |

**Applies when:** incompressible liquid at rest; uniform gravity; a point at a depth below the free surface of a liquid at rest.

```shell
$ thermofluids solve hydrostatic_gauge_pressure "rho=1000 kg/m^3" "g=9.80665 m/s^2" "h=12 m"
gauge pressure at the depth (P) = 117679.8 Pa
  by: Gauge pressure at a depth in a liquid  (P = rho*g*h)
  rho = 1000 kg/m^3   [1000 kg/m^3]
  g = 9.80665 m/s^2   [9.80665 m/s^2]
  h = 12 m   [12 m]
  assumes: incompressible liquid at rest; uniform gravity; the situation is: hydrostatic
```

**What the worked example means:** This calculation determines the gauge pressure at a specific point within a liquid, which represents the pressure increase due solely to the weight of the fluid above that point, excluding atmospheric pressure. The result of 117680 Pa is derived by multiplying the fluid's density of 1000 kg/m^3, the acceleration due to gravity of 9.80665 m/s^2, and the vertical depth below the free surface of 12 m. This value indicates how much the local pressure exceeds the ambient atmospheric pressure at the surface, a critical factor for assessing structural loads in tanks or diving depths.

<a id="hydrostatic_force_plane"></a>

#### Hydrostatic force on a submerged plane surface, `hydrostatic_force_plane`

One equation links resultant hydrostatic force on the surface (F), density of the fluid (rho), Acceleration (g), depth of the surface's centroid (hc) and area of the surface (A). Give all but one of them and it is solved for the one left out, by default resultant hydrostatic force on the surface (F).

$$
F = \rho g h_{c} A
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rho` | in | density of the fluid | kg/m^3 |
| `g` | in | Acceleration (optional) | m/s^2 |
| `hc` | in | depth of the surface's centroid | m |
| `A` | in | area of the surface | m^2 |
| `F` | out | resultant hydrostatic force on the surface | N |

**Applies when:** liquid at rest; gauge pressure (atmosphere acts on both sides); a plane gate, wall or plate wholly submerged in a liquid.

```shell
$ thermofluids solve hydrostatic_force_plane "rho=1000 kg/m^3" "g=9.80665 m/s^2" "hc=3 m" "A=2 m^2"
resultant hydrostatic force on the surface (F) = 58839.9 N
  by: Hydrostatic force on a submerged plane surface  (F = rho*g*hc*A)
  rho = 1000 kg/m^3   [1000 kg/m^3]
  g = 9.80665 m/s^2   [9.80665 m/s^2]
  hc = 3 m   [3 m]
  A = 2 m^2   [2 m^2]
  assumes: liquid at rest; gauge pressure (atmosphere acts on both sides); the situation is: submerged surface
```

**What the worked example means:** This calculation determines the total force exerted by a stationary liquid on a flat surface that is fully submerged. The result, 58839.9 N, represents the magnitude of this force in newtons. It is derived from the fluid density, gravitational acceleration, the depth of the surface centroid, and the surface area. This value helps engineers assess structural loads on gates or walls.

### Gas processes (7)

<a id="compressor_outlet_temperature_air"></a>

#### Actual outlet temperature of an adiabatic air compressor, `compressor_outlet_temperature_air`

One equation links final temperature (T2), initial temperature (T1), final pressure (P2), initial pressure (P1) and isentropic efficiency of the compressor (eta). Give all but one of them and it is solved for the one left out, by default final temperature (T2).

$$
T_{2} = T_{1} \left(1 + \frac{\left(\frac{P_{2}}{P_{1}}\right)^{\frac{2}{7}} - 1}{\eta}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T1` | in | initial temperature | K |
| `P2` | in | final pressure | Pa |
| `P1` | in | initial pressure | Pa |
| `eta` | in | isentropic efficiency of the compressor | - |
| `T2` | out | final temperature | K |

**Applies when:** air as an ideal gas with gamma = 1.4; adiabatic compressor; real compressors with a known isentropic efficiency.

```shell
$ thermofluids solve compressor_outlet_temperature_air "T1=300 K" "P2=600000 Pa" "P1=100000 Pa" "eta=0.85 1"
final temperature (T2) = 535.9448615 K
  by: Actual outlet temperature of an adiabatic air compressor  (T2 = T1*(1 + ((P2/P1)**(2/7) - 1)/eta))
  T1 = 300 K   [300 K]
  P2 = 600000 Pa   [600000 Pa]
  P1 = 100000 Pa   [100000 Pa]
  eta = 0.85   [0.85 1]
  assumes: air as an ideal gas with gamma = 1.4; adiabatic compressor; the situation is: compressor, air
```

**What the worked example means:** This calculation determines the final temperature of air exiting an adiabatic compressor, denoted as 535.945 K. It uses the initial inlet temperature 300 K, the pressure ratio derived from the final pressure 600000 Pa and initial pressure 100000 Pa, and the machine's isentropic efficiency 0.85. The result reflects how real-world inefficiencies cause the gas to heat up more than it would in an ideal, lossless process.

<a id="entropy_change_incompressible"></a>

#### Entropy change of an incompressible substance, `entropy_change_incompressible`

One equation links total entropy change (dS), mass (m), specific heat (c), final temperature (T2) and initial temperature (T1). Give all but one of them and it is solved for the one left out, by default total entropy change (dS).

$$
d_{S} = m c \ln\left(\frac{T_{2}}{T_{1}}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass | kg |
| `c` | in | specific heat | J/(kg*K) |
| `T2` | in | final temperature | K |
| `T1` | in | initial temperature | K |
| `dS` | out | total entropy change | J/K |

**Applies when:** incompressible substance with constant specific heat; liquids and solids heated or cooled.

```shell
$ thermofluids solve entropy_change_incompressible "m=2 kg" "c=4180 J/(kg*K)" "T2=353.15 K" "T1=293.15 K"
total entropy change (dS) = 1556.70283 J/K
  by: Entropy change of an incompressible substance  (dS = m*c*log(T2/T1))
  m = 2 kg   [2 kg]
  c = 4180 J/(kg*K)   [4180 J/(kg*K)]
  T2 = 353.15 K   [353.15 K]
  T1 = 293.15 K   [293.15 K]
  assumes: incompressible substance with constant specific heat; the situation is: entropy, incompressible substance
```

**What the worked example means:** This calculation determines the total entropy change for an incompressible substance, such as a liquid or solid, undergoing a temperature shift. It uses the mass of the substance, its constant specific heat capacity, and the ratio of the final absolute temperature to the initial absolute temperature. The result, 1556.7 J/K, represents the net increase or decrease in disorder within the material during this thermal process.

<a id="ideal_gas_law"></a>

#### Ideal gas law: `ideal_gas_law`

One equation links absolute pressure of the gas (P), volume of the gas (V), amount of gas (n), Gas constant (R) and absolute temperature (T). Give all but one of them and it is solved for the one left out, by default .

$$
P V = n R T
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P` | in | absolute pressure of the gas | Pa |
| `V` | in | volume of the gas | m^3 |
| `n` | in | amount of gas | mol |
| `R` | in | Gas constant (optional) | J/(mol*K) |
| `T` | in | absolute temperature | K |

**Applies when:** ideal gas; absolute pressure and temperature; gases at moderate pressure.

```shell
$ thermofluids solve ideal_gas_law "V=0.05 m^3" "n=2 mol" "R=8.314462618 J/(mol*K)" "T=300 K"
absolute pressure of the gas (P) = 99773.55142 Pa
  by: Ideal gas law  (P*V = n*R*T)
  V = 0.05 m^3   [0.05 m^3]
  n = 2 mol   [2 mol]
  R = 8.314462618 J/(mol*K)   [8.31446 J/(mol*K)]
  T = 300 K   [300 K]
  assumes: ideal gas; absolute pressure and temperature; the situation is: ideal gas
```

**What the worked example means:** The model calculates the absolute pressure of the gas using the ideal gas law. Given the volume, amount of substance, and temperature, the result is 99773.6 Pa. This value represents the force exerted by the gas per unit area under the assumption that the gas behaves ideally.

<a id="entropy_change_ideal_gas"></a>

#### Specific entropy change of an ideal gas: `entropy_change_ideal_gas`

One equation links change in specific entropy (ds), specific heat at constant pressure (cp), specific gas constant (R), initial temperature (T1), final temperature (T2), initial pressure (P1) and final pressure (P2). Give all but one of them and it is solved for the one left out, by default change in specific entropy (ds).

$$
d_{s} = c_{p} \ln\left(\frac{T_{2}}{T_{1}}\right) - R \ln\left(\frac{P_{2}}{P_{1}}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `cp` | in | specific heat at constant pressure | J/(kg*K) |
| `R` | in | specific gas constant | J/(kg*K) |
| `T1` | in | initial temperature | K |
| `T2` | in | final temperature | K |
| `P1` | in | initial pressure | Pa |
| `P2` | in | final pressure | Pa |
| `ds` | out | change in specific entropy | J/(kg*K) |

**Applies when:** ideal gas with constant specific heats; any process between two equilibrium states.

```shell
$ thermofluids solve entropy_change_ideal_gas "cp=1005 J/(kg*K)" "R=287 J/(kg*K)" "T1=300 K" "T2=600 K" "P1=100000 Pa" "P2=500000 Pa"
change in specific entropy (ds) = 234.7042356 J/(kg*K)
  by: Specific entropy change of an ideal gas  (ds = cp*log(T2/T1) - R*log(P2/P1))
  cp = 1005 J/(kg*K)   [1005 J/(kg*K)]
  R = 287 J/(kg*K)   [287 J/(kg*K)]
  T1 = 300 K   [300 K]
  T2 = 600 K   [600 K]
  P1 = 100000 Pa   [100000 Pa]
  P2 = 500000 Pa   [500000 Pa]
  assumes: ideal gas with constant specific heats; the situation is: entropy, ideal gas
```

**What the worked example means:** This calculation determines the change in specific entropy for an ideal gas undergoing a process between two equilibrium states. The result, 234.704 J/(kg*K), reflects the balance between thermal effects driven by the ratio of final to initial temperature and mechanical effects driven by the ratio of final to initial pressure. It assumes constant specific heats and ideal gas behavior to quantify how disorder or energy dispersal changes during the transition.

<a id="isentropic_temperature_pressure_air"></a>

#### Temperature after an isentropic pressure change of air, `isentropic_temperature_pressure_air`

One equation links final temperature (T2), initial temperature (T1), final pressure (P2) and initial pressure (P1). Give all but one of them and it is solved for the one left out, by default final temperature (T2).

$$
T_{2} = T_{1} \left(\frac{P_{2}}{P_{1}}\right)^{\frac{2}{7}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T1` | in | initial temperature | K |
| `P2` | in | final pressure | Pa |
| `P1` | in | initial pressure | Pa |
| `T2` | out | final temperature | K |

**Applies when:** air as an ideal gas with gamma = 1.4; reversible adiabatic process; ideal compressors, turbines and nozzles with air.

```shell
$ thermofluids solve isentropic_temperature_pressure_air "T1=300 K" "P2=800000 Pa" "P1=100000 Pa"
final temperature (T2) = 543.4341986 K
  by: Temperature after an isentropic pressure change of air  (T2 = T1*(P2/P1)**(2/7))
  T1 = 300 K   [300 K]
  P2 = 800000 Pa   [800000 Pa]
  P1 = 100000 Pa   [100000 Pa]
  assumes: air as an ideal gas with gamma = 1.4; reversible adiabatic process; the situation is: adiabatic process, air
```

**What the worked example means:** This calculation determines the final temperature of air after an isentropic pressure change. It uses the initial temperature 300 K and the ratio of final to initial pressures, where the final pressure is 800000 Pa and the initial pressure is 100000 Pa. The result indicates the thermodynamic state of the air assuming ideal gas behavior and a reversible adiabatic process.

<a id="isentropic_temperature_volume_air"></a>

#### Temperature after an isentropic volume change of air, `isentropic_temperature_volume_air`

One equation links final temperature (T2), initial temperature (T1), initial volume (V1) and final volume (V2). Give all but one of them and it is solved for the one left out, by default final temperature (T2).

$$
T_{2} = T_{1} \left(\frac{V_{1}}{V_{2}}\right)^{\frac{2}{5}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T1` | in | initial temperature | K |
| `V1` | in | initial volume | m^3 |
| `V2` | in | final volume | m^3 |
| `T2` | out | final temperature | K |

**Applies when:** air as an ideal gas with gamma = 1.4; reversible adiabatic process; ideal piston-cylinder compression or expansion of air.

```shell
$ thermofluids solve isentropic_temperature_volume_air "T1=300 K" "V1=0.001 m^3" "V2=0.000125 m^3"
final temperature (T2) = 689.219013 K
  by: Temperature after an isentropic volume change of air  (T2 = T1*(V1/V2)**(2/5))
  T1 = 300 K   [300 K]
  V1 = 0.001 m^3   [0.001 m^3]
  V2 = 0.000125 m^3   [0.000125 m^3]
  assumes: air as an ideal gas with gamma = 1.4; reversible adiabatic process; the situation is: adiabatic process, air
```

**What the worked example means:** This calculation determines the final temperature 689.219 K of air after a reversible adiabatic change in volume. It uses the initial temperature 300 K, initial volume 0.001 m^3, and final volume 0.000125 m^3. The result represents the thermal state of the gas assuming ideal behavior and no heat transfer or friction during the compression or expansion process.

<a id="isothermal_work_ideal_gas"></a>

#### Work done by an ideal gas in a reversible isothermal process, `isothermal_work_ideal_gas`

One equation links work done by the gas (W), amount of gas (n), Gas constant (R), constant absolute temperature (T), final volume (V2) and initial volume (V1). Give all but one of them and it is solved for the one left out, by default work done by the gas (W).

$$
W = n R T \ln\left(\frac{V_{2}}{V_{1}}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | amount of gas | mol |
| `R` | in | Gas constant (optional) | J/(mol*K) |
| `T` | in | constant absolute temperature | K |
| `V2` | in | final volume | m^3 |
| `V1` | in | initial volume | m^3 |
| `W` | out | work done by the gas | J |

**Applies when:** ideal gas; reversible (quasi-static) process; slow expansion or compression at constant temperature.

```shell
$ thermofluids solve isothermal_work_ideal_gas "n=1 mol" "R=8.314462618 J/(mol*K)" "T=300 K" "V2=0.02 m^3" "V1=0.01 m^3"
work done by the gas (W) = 1728.943896 J
  by: Work done by an ideal gas in a reversible isothermal process  (W = n*R*T*log(V2/V1))
  n = 1 mol   [1 mol]
  R = 8.314462618 J/(mol*K)   [8.31446 J/(mol*K)]
  T = 300 K   [300 K]
  V2 = 0.02 m^3   [0.02 m^3]
  V1 = 0.01 m^3   [0.01 m^3]
  assumes: ideal gas; reversible (quasi-static) process; the situation is: isothermal
```

**What the worked example means:** This calculation determines the work performed by an ideal gas during a reversible, isothermal process using the formula W equals n times R times T times the natural logarithm of V2 divided by V1. The result, 1728.94 J, represents the energy transferred as work when the amount of gas, 1 mol, expands or compresses at a constant absolute temperature, 300 K, changing its volume from 0.01 m^3 to 0.02 m^3. This value highlights how the work depends on the ratio of final to initial volumes rather than just pressure differences.

### Heat exchangers (7)

<a id="effectiveness_phase_change"></a>

#### Effectiveness of a heat exchanger with one stream changing phase, `effectiveness_phase_change`

One equation links heat exchanger effectiveness (eps) and number of transfer units UA/Cmin (N). Give all but one of them and it is solved for the one left out, by default heat exchanger effectiveness (eps).

$$
\varepsilon = 1 - e^{-N}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `N` | in | number of transfer units UA/Cmin | - |
| `eps` | out | heat exchanger effectiveness | - |

**Applies when:** one stream at constant temperature (Cr = 0); condensers and evaporators, any flow arrangement.

```shell
$ thermofluids solve effectiveness_phase_change "N=2 1"
heat exchanger effectiveness (eps) = 0.8646647168
  by: Effectiveness of a heat exchanger with one stream changing phase  (eps = 1 - exp(-N))
  N = 2   [2 1]
  assumes: one stream at constant temperature (Cr = 0); the situation is: heat exchanger, phase change
```

**What the worked example means:** This calculation determines the performance of a heat exchanger where one fluid undergoes phase change, such as in a condenser or evaporator. By using the formula eps = 1 - exp(-N), the effectiveness 0.864665 is derived directly from the number of transfer units 2. This result indicates how close the actual heat transfer is to the maximum theoretically possible amount for this specific configuration.

<a id="effectiveness_parallel_flow"></a>

#### Effectiveness of a parallel-flow heat exchanger, `effectiveness_parallel_flow`

One equation links heat exchanger effectiveness (eps), number of transfer units UA/Cmin (N) and capacity rate ratio Cmin/Cmax (Cr). Give all but one of them and it is solved for the one left out, by default heat exchanger effectiveness (eps).

$$
\varepsilon = \frac{1 - e^{-N \left(1 + C_{r}\right)}}{1 + C_{r}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `N` | in | number of transfer units UA/Cmin | - |
| `Cr` | in | capacity rate ratio Cmin/Cmax | - |
| `eps` | out | heat exchanger effectiveness | - |

**Applies when:** steady flow, constant properties; pure parallel-flow (co-current) exchangers.

```shell
$ thermofluids solve effectiveness_parallel_flow "N=1 1" "Cr=0.5 1"
heat exchanger effectiveness (eps) = 0.5179132266
  by: Effectiveness of a parallel-flow heat exchanger  (eps = (1 - exp(-N*(1 + Cr)))/(1 + Cr))
  N = 1   [1 1]
  Cr = 0.5   [0.5 1]
  assumes: steady flow, constant properties; the situation is: heat exchanger, parallel flow
```

In the worked example, N = 1 1, Cr = 0.5 1 go through the equation above and give eps = 0.5179132266 1. It applies only when steady flow, constant properties; pure parallel-flow (co-current) exchangers: outside that it says what is wrong instead of returning a number.

<a id="sensible_heat_rate"></a>

#### Heat added to a flowing stream (steady-flow energy balance), `sensible_heat_rate`

One equation links rate of heat transfer (Q), mass flow rate of the stream (mdot), specific heat of the fluid (cp), outlet temperature (Tout) and inlet temperature (Tin). Give all but one of them and it is solved for the one left out, by default rate of heat transfer (Q).

$$
Q = \text{mdot} c_{p} \left(T_{\text{out}} - T_{\text{in}}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `mdot` | in | mass flow rate of the stream | kg/s |
| `cp` | in | specific heat of the fluid | J/(kg*K) |
| `Tout` | in | outlet temperature | K |
| `Tin` | in | inlet temperature | K |
| `Q` | out | rate of heat transfer | W |

**Applies when:** steady flow; constant cp; no phase change; kinetic and potential energy changes negligible; heaters, coolers and each side of a heat exchanger.

```shell
$ thermofluids solve sensible_heat_rate "mdot=0.5 kg/s" "cp=4180 J/(kg*K)" "Tout=333.15 K" "Tin=293.15 K"
rate of heat transfer (Q) = 83600 W
  by: Heat added to a flowing stream (steady-flow energy balance)  (Q = mdot*cp*(Tout - Tin))
  mdot = 0.5 kg/s   [0.5 kg/s]
  cp = 4180 J/(kg*K)   [4180 J/(kg*K)]
  Tout = 333.15 K   [333.15 K]
  Tin = 293.15 K   [293.15 K]
  assumes: steady flow; constant cp; no phase change; kinetic and potential energy changes negligible; the situation is: stream heating
```

**What the worked example means:** This calculation determines the thermal power exchanged with a flowing fluid stream. It uses the steady-flow energy balance, assuming constant specific heat and negligible kinetic and potential energy changes. The result, 83600 W, represents the net heat added or removed based on the mass flow rate 0.5 kg/s, the specific heat 4180 J/(kg*K), and the difference between outlet temperature 333.15 K and inlet temperature 293.15 K. This value is critical for sizing heaters, coolers, and heat exchangers in systems where no phase change occurs.

<a id="hx_effectiveness_ntu"></a>

#### Heat exchanger rating by the effectiveness-NTU method, `hx_effectiveness_ntu`

Heat duty, outlet temperatures, effectiveness and NTU of a counterflow or parallel-flow exchanger.

$$
Q = \varepsilon(NTU, C_r)\,C_{min}(T_{h,i} - T_{c,i})
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C_hot` | in | heat capacity rate of the hot stream (m cp) | W/K |
| `C_cold` | in | heat capacity rate of the cold stream (m cp) | W/K |
| `UA` | in | overall conductance U A | W/K |
| `Th_in` | in | hot inlet temperature | K |
| `Tc_in` | in | cold inlet temperature | K |
| `arrangement` | in | flow arrangement | - |
| `Q` | out | heat transfer rate | W |
| `Th_out` | out | hot outlet temperature | K |
| `Tc_out` | out | cold outlet temperature | K |
| `effectiveness` | out | effectiveness | - |
| `NTU` | out | number of transfer units | - |

**Applies when:** positive capacity rates and UA; hot inlet hotter than cold inlet.

```shell
$ thermofluids solve hx_effectiveness_ntu "C_hot=2000 W/K" "C_cold=4000 W/K" "UA=3000 W/K" "Th_in=150 °C" "Tc_in=30 °C" arrangement=counterflow
Q = 165788.498 W
Th_out = 340.255751 K
Tc_out = 344.5971245 K
effectiveness = 0.6907854082
NTU = 1.5
  by: Heat exchanger rating by the effectiveness-NTU method  (NTU = UA/Cmin, Cr = Cmin/Cmax, effectiveness of the arrangement, Q = eps Cmin (Th,in - Tc,in), energy balances)
  C_hot = 2000 W/K
  C_cold = 4000 W/K
  UA = 3000 W/K
  Th_in = 423.15 K
  Tc_in = 303.15 K
  arrangement = counterflow
  assumes: positive capacity rates and UA; hot inlet hotter than cold inlet
```

**What the worked example means:** This calculation determines the performance of a heat exchanger using the effectiveness-NTU method. Given the capacity rates, overall conductance, and inlet temperatures, the model computes the number of transfer units as 1.5 and the effectiveness as 0.690785. The resulting heat transfer rate is 165788 W, which leads to hot and cold outlet temperatures of 340.256 K and 344.597 K respectively.

<a id="heat_rate_effectiveness"></a>

#### Heat transfer rate from the effectiveness: `heat_rate_effectiveness`

One equation links rate of heat transfer (Q), heat exchanger effectiveness (eps), smaller of the two heat capacity rates (m cp) (Cmin), inlet temperature of the hot fluid (Thi) and inlet temperature of the cold fluid (Tci). Give all but one of them and it is solved for the one left out, by default rate of heat transfer (Q).

$$
Q = \varepsilon C_{\text{min}} \left(\text{Thi} - \text{Tci}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `eps` | in | heat exchanger effectiveness | - |
| `Cmin` | in | smaller of the two heat capacity rates (m cp) | W/K |
| `Thi` | in | inlet temperature of the hot fluid | K |
| `Tci` | in | inlet temperature of the cold fluid | K |
| `Q` | out | rate of heat transfer | W |

**Applies when:** steady flow; no heat loss to the surroundings; rating an exchanger whose effectiveness is known.

```shell
$ thermofluids solve heat_rate_effectiveness "eps=0.6 1" "Cmin=2000 W/K" "Thi=423.15 K" "Tci=303.15 K"
rate of heat transfer (Q) = 144000 W
  by: Heat transfer rate from the effectiveness  (Q = eps*Cmin*(Thi - Tci))
  eps = 0.6   [0.6 1]
  Cmin = 2000 W/K   [2000 W/K]
  Thi = 423.15 K   [423.15 K]
  Tci = 303.15 K   [303.15 K]
  assumes: steady flow; no heat loss to the surroundings; the situation is: heat exchanger
```

**What the worked example means:** This calculation determines the actual heat transfer rate, denoted as Q, which equals 144000 W. It is derived from the product of the heat exchanger effectiveness, eps (0.6), the minimum heat capacity rate, Cmin (2000 W/K), and the difference between the hot fluid inlet temperature, Thi (423.15 K), and the cold fluid inlet temperature, Tci (303.15 K). This result represents the practical energy exchange power assuming ideal conditions where no heat is lost to the surroundings.

<a id="lmtd"></a>

#### Log-mean temperature difference of a heat exchanger, `lmtd`

The LMTD of a counterflow or parallel-flow exchanger from the four terminal temperatures.

$$
\Delta T_{lm} = \frac{\Delta T_1 - \Delta T_2}{\ln(\Delta T_1/\Delta T_2)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Th_in` | in | hot stream inlet temperature | K |
| `Th_out` | in | hot stream outlet temperature | K |
| `Tc_in` | in | cold stream inlet temperature | K |
| `Tc_out` | in | cold stream outlet temperature | K |
| `arrangement` | in | flow arrangement | - |
| `lmtd` | out | log-mean temperature difference | K |
| `dT1` | out | temperature difference at the hot-inlet end | K |
| `dT2` | out | temperature difference at the other end | K |

**Applies when:** both terminal temperature differences positive; the hot stream cools and the cold stream warms.

```shell
$ thermofluids solve lmtd "Th_in=150 °C" "Th_out=90 °C" "Tc_in=30 °C" "Tc_out=70 °C" arrangement=counterflow
lmtd = 69.52118994 K
dT1 = 80 K
dT2 = 60 K
  by: Log-mean temperature difference of a heat exchanger  ((dT1 - dT2) / ln(dT1 / dT2) from the terminal differences of the chosen arrangement)
  Th_in = 423.15 K
  Th_out = 363.15 K
  Tc_in = 303.15 K
  Tc_out = 343.15 K
  arrangement = counterflow
  assumes: both terminal temperature differences positive; the hot stream cools and the cold stream warms
```

**What the worked example means:** This calculation determines the average temperature driving force for heat transfer in a heat exchanger. It uses the four terminal temperatures to find the difference at each end, yielding values of 80 K and 60 K. The final result, 69.5212 K, represents the effective mean temperature difference that engineers use to size the equipment correctly under steady-state conditions.

<a id="ntu_definition"></a>

#### Number of transfer units of a heat exchanger, `ntu_definition`

One equation links number of transfer units UA/Cmin (N), overall heat transfer coefficient (U), heat transfer area (A) and smaller of the two heat capacity rates (m cp) (Cmin). Give all but one of them and it is solved for the one left out, by default number of transfer units UA/Cmin (N).

$$
N = \frac{U A}{C_{\text{min}}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `U` | in | overall heat transfer coefficient | W/(m^2*K) |
| `A` | in | heat transfer area | m^2 |
| `Cmin` | in | smaller of the two heat capacity rates (m cp) | W/K |
| `N` | out | number of transfer units UA/Cmin | - |

**Applies when:** U uniform over the area; the effectiveness-NTU method.

```shell
$ thermofluids solve ntu_definition "U=500 W/(m^2*K)" "A=10 m^2" "Cmin=2000 W/K"
number of transfer units UA/Cmin (N) = 2.5
  by: Number of transfer units of a heat exchanger  (N = U*A/Cmin)
  U = 500 W/(m^2*K)   [500 W/(m^2*K)]
  A = 10 m^2   [10 m^2]
  Cmin = 2000 W/K   [2000 W/K]
  assumes: U uniform over the area; the situation is: heat exchanger
```

**What the worked example means:** This calculation determines the number of transfer units, a dimensionless parameter that quantifies the thermal size of a heat exchanger. It is derived by dividing the product of the overall heat transfer coefficient and the heat transfer area by the smaller of the two heat capacity rates. The resulting value of 2.5 represents the ratio of the total conductance capability to the minimum fluid capacity rate, which helps assess the potential effectiveness of the heat exchange process under the assumption that the coefficient is uniform across the surface.

### Natural convection (2)

<a id="grashof_number"></a>

#### Grashof number: `grashof_number`

One equation links Grashof number (Gr), Acceleration (g), volumetric thermal expansion coefficient of the fluid (beta), surface temperature (Ts), temperature of the fluid far from the surface (Tinf), height of the plate (characteristic length) (L) and kinematic viscosity of the fluid (nu). Give all but one of them and it is solved for the one left out, by default Grashof number (Gr).

$$
\mathrm{Gr} = \frac{g \beta \left(T_{s} - \text{Tinf}\right) L^{3}}{\nu^{2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `g` | in | Acceleration (optional) | m/s^2 |
| `beta` | in | volumetric thermal expansion coefficient of the fluid | 1/K |
| `Ts` | in | surface temperature | K |
| `Tinf` | in | temperature of the fluid far from the surface | K |
| `L` | in | height of the plate (characteristic length) | m |
| `nu` | in | kinematic viscosity of the fluid | m^2/s |
| `Gr` | out | Grashof number | - |

**Applies when:** Boussinesq approximation; surface hotter than the fluid; buoyancy-driven flow next to a heated or cooled surface.

```shell
$ thermofluids solve grashof_number "g=9.80665 m/s^2" "beta=0.0033 1/K" "Ts=330 K" "Tinf=300 K" "L=0.5 m" "nu=1.6e-05 m^2/s"
Grashof number (Gr) = 474051928.7
  by: Grashof number  (Gr = g*beta*(Ts - Tinf)*L**3/nu**2)
  g = 9.80665 m/s^2   [9.80665 m/s^2]
  beta = 0.0033 1/K   [0.0033 1/K]
  Ts = 330 K   [330 K]
  Tinf = 300 K   [300 K]
  L = 0.5 m   [0.5 m]
  nu = 1.6e-05 m^2/s   [1.6e-05 m^2/s]
  assumes: Boussinesq approximation; surface hotter than the fluid; the situation is: natural convection
```

**What the worked example means:** This calculation determines the Grashof number, denoted as 4.74052e+08, using the gravitational acceleration 9.80665 m/s^2, the volumetric thermal expansion coefficient 0.0033 1/K, the surface temperature 330 K, the fluid far-field temperature 300 K, the characteristic length 0.5 m, and the kinematic viscosity 1.6e-05 m^2/s. The resulting value quantifies the relative strength of buoyancy forces compared to viscous forces in natural convection. A higher value suggests that buoyancy effects dominate over viscous resistance, driving stronger fluid motion due to temperature differences.

<a id="rayleigh_number"></a>

#### Rayleigh number: `rayleigh_number`

One equation links Rayleigh number (Ra), Grashof number (Gr) and Prandtl number of the fluid (Pr). Give all but one of them and it is solved for the one left out, by default Rayleigh number (Ra).

$$
\mathrm{Ra} = \mathrm{Gr} \mathrm{Pr}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Gr` | in | Grashof number | - |
| `Pr` | in | Prandtl number of the fluid | - |
| `Ra` | out | Rayleigh number | - |

**Applies when:** properties at the film temperature; natural convection correlations.

```shell
$ thermofluids solve rayleigh_number "Gr=100000000 1" "Pr=0.71 1"
Rayleigh number (Ra) = 71000000
  by: Rayleigh number  (Ra = Gr*Pr)
  Gr = 100000000   [1e+08 1]
  Pr = 0.71   [0.71 1]
  assumes: properties at the film temperature; the situation is: rayleigh
```

**What the worked example means:** This calculation determines the Rayleigh number, which characterizes natural convection. It is derived by multiplying the Grashof number, representing buoyancy forces against viscous forces, by the Prandtl number, which relates momentum diffusivity to thermal diffusivity. The resulting value of 7.1e+07 helps select appropriate correlations for heat transfer in buoyancy-driven flows, assuming fluid properties are evaluated at the film temperature.

### Open channel (1)

<a id="froude_number"></a>

#### Froude number of an open-channel flow: `froude_number`

One equation links Froude number (Fr), mean flow velocity (v), Acceleration (g) and flow depth (h). Give all but one of them and it is solved for the one left out, by default Froude number (Fr).

$$
\mathrm{Fr} = \frac{v}{\sqrt{g h}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `v` | in | mean flow velocity | m/s |
| `g` | in | Acceleration (optional) | m/s^2 |
| `h` | in | flow depth | m |
| `Fr` | out | Froude number | - |

**Applies when:** rectangular channel (hydraulic depth = flow depth); free-surface flow in a channel or river.

```shell
$ thermofluids solve froude_number "v=3 m/s" "g=9.80665 m/s^2" "h=0.5 m"
Froude number (Fr) = 1.354802267
  by: Froude number of an open-channel flow  (Fr = v/sqrt(g*h))
  v = 3 m/s   [3 m/s]
  g = 9.80665 m/s^2   [9.80665 m/s^2]
  h = 0.5 m   [0.5 m]
  assumes: rectangular channel (hydraulic depth = flow depth); the situation is: open channel
```

**What the worked example means:** This calculation determines the Froude number, a dimensionless parameter comparing inertial and gravitational forces in open-channel flow. It is derived from the mean flow velocity 3 m/s and the flow depth 0.5 m. The resulting value of 1.3548 helps classify the flow regime as subcritical or supercritical, which influences how disturbances propagate through the channel.

### Pipe flow (12)

<a id="blasius_friction_factor"></a>

#### Blasius friction factor for turbulent flow in a smooth pipe, `blasius_friction_factor`

One equation links Darcy friction factor (f) and Reynolds number (Re). Give all but one of them and it is solved for the one left out, by default Darcy friction factor (f).

$$
f = 0.3164 \mathrm{Re}^{\frac{-1}{4}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Re` | in | Reynolds number | - |
| `f` | out | Darcy friction factor | - |

**Applies when:** fully developed turbulent flow in a hydraulically smooth circular pipe; Darcy friction factor; 4000 < Re < 100000 in a smooth pipe.

```shell
$ thermofluids solve blasius_friction_factor "Re=50000 1"
Darcy friction factor (f) = 0.02115894325
  by: Blasius friction factor for turbulent flow in a smooth pipe  (f = 0.3164*Re**(-1/4))
  Re = 50000   [50000 1]
  assumes: fully developed turbulent flow in a hydraulically smooth circular pipe; Darcy friction factor; the situation is: pipe, turbulent, smooth pipe
```

**What the worked example means:** This calculation determines the Darcy friction factor, which represents the resistance to flow in a smooth pipe. It uses the Blasius correlation, relating the friction factor to the Reynolds number that characterizes the flow regime. The resulting value of 0.0211589 indicates how much pressure is lost due to wall shear stress for a flow with a Reynolds number of 50000. This metric is essential for sizing pumps and predicting energy losses in turbulent hydraulic systems.

<a id="colebrook_friction_factor"></a>

#### Darcy friction factor from the Colebrook equation, `colebrook_friction_factor`

Solve the implicit Colebrook-White equation for the Darcy friction factor of turbulent pipe flow.

$$
\frac{1}{\sqrt{f}} = -2\log_{10}\left(\frac{\varepsilon/D}{3.7} + \frac{2.51}{Re\sqrt{f}}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Re` | in | Reynolds number (turbulent, at least 4000) | - |
| `eD` | in | relative roughness (roughness / diameter) | - |
| `f` | out | Darcy friction factor | - |

**Applies when:** Re >= 4000; 0 <= eD <= 0.05.

```shell
$ thermofluids solve colebrook_friction_factor Re=100000 eD=0.0002
f = 0.01900543522
  by: Darcy friction factor from the Colebrook equation  (Newton iteration on 1/sqrt(f) from a Swamee-Jain start, to machine precision)
  Re = 100000
  eD = 0.0002
  assumes: Re >= 4000; 0 <= eD <= 0.05
```

**What the worked example means:** This calculation determines the Darcy friction factor for turbulent pipe flow by solving the implicit Colebrook-White equation. Using the input Reynolds number 100000 and relative roughness 0.0002, the method iterates to find the precise resistance coefficient, resulting in a friction factor of 0.0190054. This value quantifies the energy loss due to fluid friction within the pipe, which is essential for accurate pressure drop predictions in engineering systems.

<a id="laminar_friction_factor"></a>

#### Darcy friction factor in laminar pipe flow, `laminar_friction_factor`

One equation links Darcy friction factor (f) and Reynolds number (Re). Give all but one of them and it is solved for the one left out, by default Darcy friction factor (f).

$$
f = \frac{64}{\mathrm{Re}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Re` | in | Reynolds number | - |
| `f` | out | Darcy friction factor | - |

**Applies when:** fully developed laminar flow in a circular pipe; Darcy (not Fanning) friction factor; Re below about 2300 in a circular pipe.

```shell
$ thermofluids solve laminar_friction_factor "Re=1200 1"
Darcy friction factor (f) = 0.05333333333
  by: Darcy friction factor in laminar pipe flow  (f = 64/Re)
  Re = 1200   [1200 1]
  assumes: fully developed laminar flow in a circular pipe; Darcy (not Fanning) friction factor; the situation is: pipe, laminar
```

**What the worked example means:** This calculation determines the Darcy friction factor, which quantifies resistance to flow in a pipe. For fully developed laminar flow in a circular pipe, the friction factor depends inversely on the Reynolds number. Given a Reynolds number of 1200, the resulting Darcy friction factor is 0.0533333. This value helps engineers estimate pressure drops in low-speed or high-viscosity fluid systems.

<a id="darcy_weisbach_head_loss"></a>

#### Darcy-Weisbach head loss in a pipe: `darcy_weisbach_head_loss`

One equation links head loss to friction (hL), Darcy friction factor (f), length of the pipe (L), mean flow velocity (v), Acceleration (g) and inner diameter of the pipe (D). Give all but one of them and it is solved for the one left out, by default head loss to friction (hL).

$$
h_{L} = \frac{f L v^{2}}{2 g D}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `f` | in | Darcy friction factor | - |
| `L` | in | length of the pipe | m |
| `v` | in | mean flow velocity | m/s |
| `g` | in | Acceleration (optional) | m/s^2 |
| `D` | in | inner diameter of the pipe | m |
| `hL` | out | head loss to friction | m |

**Applies when:** fully developed flow; Darcy friction factor; constant diameter; frictional loss along a straight pipe of constant diameter.

```shell
$ thermofluids solve darcy_weisbach_head_loss "f=0.02 1" "L=100 m" "v=2 m/s" "g=9.80665 m/s^2" "D=0.1 m"
head loss to friction (hL) = 4.078864852 m
  by: Darcy-Weisbach head loss in a pipe  (hL = f*L*v**2/(2*g*D))
  f = 0.02   [0.02 1]
  L = 100 m   [100 m]
  v = 2 m/s   [2 m/s]
  g = 9.80665 m/s^2   [9.80665 m/s^2]
  D = 0.1 m   [0.1 m]
  assumes: fully developed flow; Darcy friction factor; constant diameter; the situation is: pipe
```

**What the worked example means:** This calculation determines the energy loss due to friction in a straight pipe of constant diameter, expressed as head loss. Using the Darcy friction factor, pipe length, mean flow velocity, inner diameter, and gravitational acceleration, the model computes that the head loss to friction equals 4.07886 m. This result indicates how much hydraulic energy is dissipated as heat over the specified section of the piping system.

<a id="darcy_weisbach_pressure_drop"></a>

#### Darcy-Weisbach pressure drop in a pipe: `darcy_weisbach_pressure_drop`

One equation links frictional pressure drop (dP), Darcy friction factor (f), length of the pipe (L), density of the fluid (rho), mean flow velocity (v) and inner diameter of the pipe (D). Give all but one of them and it is solved for the one left out, by default frictional pressure drop (dP).

$$
d_{P} = \frac{f L \rho v^{2}}{2 D}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `f` | in | Darcy friction factor | - |
| `L` | in | length of the pipe | m |
| `rho` | in | density of the fluid | kg/m^3 |
| `v` | in | mean flow velocity | m/s |
| `D` | in | inner diameter of the pipe | m |
| `dP` | out | frictional pressure drop | Pa |

**Applies when:** fully developed flow; Darcy friction factor; constant diameter; frictional pressure loss along a straight pipe of constant diameter.

```shell
$ thermofluids solve darcy_weisbach_pressure_drop "f=0.03 1" "L=30 m" "rho=870 kg/m^3" "v=1.5 m/s" "D=0.05 m"
frictional pressure drop (dP) = 17617.5 Pa
  by: Darcy-Weisbach pressure drop in a pipe  (dP = f*L*rho*v**2/(2*D))
  f = 0.03   [0.03 1]
  L = 30 m   [30 m]
  rho = 870 kg/m^3   [870 kg/m^3]
  v = 1.5 m/s   [1.5 m/s]
  D = 0.05 m   [0.05 m]
  assumes: fully developed flow; Darcy friction factor; constant diameter; the situation is: pipe
```

**What the worked example means:** This calculation determines the frictional pressure drop along a straight pipe of constant diameter. The result, 17617.5 Pa, represents the loss in force per unit area due to fluid viscosity and wall roughness. It is derived from the product of the dimensionless Darcy friction factor, the pipe length, the fluid density, and the square of the mean flow velocity, divided by twice the inner diameter. This value indicates the additional pressure required at the inlet to maintain the specified flow rate against resistive forces within the system.

<a id="haaland_friction_factor"></a>

#### Haaland explicit friction factor for turbulent pipe flow, `haaland_friction_factor`

One equation links Darcy friction factor (f), absolute roughness of the pipe wall (eps), inner diameter of the pipe (D) and Reynolds number (Re). Give all but one of them and it is solved for the one left out, by default Darcy friction factor (f).

$$
f = \frac{1}{\left(\frac{1.8 \ln\left(\left(\frac{\varepsilon}{3.7 D}\right)^{\frac{111}{100}} + \frac{6.9}{\mathrm{Re}}\right)}{\ln\left(10\right)}\right)^{2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `eps` | in | absolute roughness of the pipe wall | m |
| `D` | in | inner diameter of the pipe | m |
| `Re` | in | Reynolds number | - |
| `f` | out | Darcy friction factor | - |

**Applies when:** fully developed turbulent flow in a circular pipe; Darcy friction factor; 4000 < Re < 1e8 and eps/D < 0.05.

```shell
$ thermofluids solve haaland_friction_factor "eps=0.00015 m" "D=0.05 m" "Re=80000 1"
Darcy friction factor (f) = 0.02766318556
  by: Haaland explicit friction factor for turbulent pipe flow  (f = 1/(1.8*log((eps/(3.7*D))**(111/100) + 6.9/Re)/log(10))**2)
  eps = 0.00015 m   [0.00015 m]
  D = 0.05 m   [0.05 m]
  Re = 80000   [80000 1]
  assumes: fully developed turbulent flow in a circular pipe; Darcy friction factor; the situation is: pipe, haaland
```

**What the worked example means:** This calculation determines the Darcy friction factor for turbulent flow in a circular pipe using the Haaland explicit equation. The result of 0.0276632 indicates the resistance to fluid flow based on the input values: an inner diameter of 0.05 m, an absolute wall roughness of 0.00015 m, and a Reynolds number of 80000. This non-iterative method provides a quick estimate suitable for fully developed turbulent conditions where the Reynolds number exceeds four thousand.

<a id="hagen_poiseuille"></a>

#### Hagen-Poiseuille flow rate in laminar pipe flow, `hagen_poiseuille`

One equation links volume flow rate (Q), inner diameter of the pipe (D), pressure drop along the pipe (dP), dynamic viscosity of the fluid (mu) and length of the pipe (L). Give all but one of them and it is solved for the one left out, by default volume flow rate (Q).

$$
Q = \frac{\pi D^{4} d_{P}}{128 \mu L}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D` | in | inner diameter of the pipe | m |
| `dP` | in | pressure drop along the pipe | Pa |
| `mu` | in | dynamic viscosity of the fluid | Pa*s |
| `L` | in | length of the pipe | m |
| `Q` | out | volume flow rate | m^3/s |

**Applies when:** fully developed laminar flow; Newtonian fluid; circular pipe; Re below about 2300.

```shell
$ thermofluids solve hagen_poiseuille "D=0.01 m" "dP=20000 Pa" "mu=0.05 Pa*s" "L=10 m"
volume flow rate (Q) = 9.817477042e-06 m^3/s
  by: Hagen-Poiseuille flow rate in laminar pipe flow  (Q = pi*D**4*dP/(128*mu*L))
  D = 0.01 m   [0.01 m]
  dP = 20000 Pa   [20000 Pa]
  mu = 0.05 Pa*s   [0.05 Pa*s]
  L = 10 m   [10 m]
  assumes: fully developed laminar flow; Newtonian fluid; circular pipe; the situation is: pipe, laminar
```

**What the worked example means:** This calculation determines the volumetric flow rate of a Newtonian fluid moving through a straight circular pipe. The result, 9.81748e-06 m^3/s, reflects the balance between the driving pressure difference and the viscous resistance imposed by the fluid's dynamic viscosity and the pipe geometry. This relationship holds strictly for fully developed laminar flow where inertial effects are negligible compared to viscous forces.

<a id="kinematic_viscosity"></a>

#### Kinematic viscosity from dynamic viscosity and density, `kinematic_viscosity`

One equation links kinematic viscosity (nu), dynamic viscosity of the fluid (mu) and density of the fluid (rho). Give all but one of them and it is solved for the one left out, by default kinematic viscosity (nu).

$$
\nu = \frac{\mu}{\rho}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `mu` | in | dynamic viscosity of the fluid | Pa*s |
| `rho` | in | density of the fluid | kg/m^3 |
| `nu` | out | kinematic viscosity | m^2/s |

**Applies when:** Newtonian fluid; converting between dynamic and kinematic viscosity.

```shell
$ thermofluids solve kinematic_viscosity "mu=0.002 Pa*s" "rho=800 kg/m^3"
kinematic viscosity (nu) = 2.5e-06 m^2/s
  by: Kinematic viscosity from dynamic viscosity and density  (nu = mu/rho)
  mu = 0.002 Pa*s   [0.002 Pa*s]
  rho = 800 kg/m^3   [800 kg/m^3]
  assumes: Newtonian fluid; the situation is: kinematic
```

**What the worked example means:** This calculation determines the kinematic viscosity, which represents the fluid's resistance to flow under gravity or inertia independent of its mass. It is computed by dividing the dynamic viscosity 0.002 Pa*s by the density 800 kg/m^3. The resulting value 2.5e-06 m^2/s indicates how easily the fluid moves relative to its own weight, a key factor in analyzing flow similarity and calculating Reynolds numbers.

<a id="minor_loss_head"></a>

#### Minor head loss in a fitting: `minor_loss_head`

One equation links head loss across the fitting (hL), loss coefficient of the fitting (K), mean flow velocity (v) and Acceleration (g). Give all but one of them and it is solved for the one left out, by default head loss across the fitting (hL).

$$
h_{L} = \frac{K v^{2}}{2 g}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `K` | in | loss coefficient of the fitting | - |
| `v` | in | mean flow velocity | m/s |
| `g` | in | Acceleration (optional) | m/s^2 |
| `hL` | out | head loss across the fitting | m |

**Applies when:** fully turbulent flow (K independent of Re); K based on the mean velocity in the pipe; valves, elbows, entrances, exits and expansions.

```shell
$ thermofluids solve minor_loss_head "K=10 1" "v=3 m/s" "g=9.80665 m/s^2"
head loss across the fitting (hL) = 4.588722958 m
  by: Minor head loss in a fitting  (hL = K*v**2/(2*g))
  K = 10   [10 1]
  v = 3 m/s   [3 m/s]
  g = 9.80665 m/s^2   [9.80665 m/s^2]
  assumes: fully turbulent flow (K independent of Re); K based on the mean velocity in the pipe; the situation is: fitting
```

**What the worked example means:** This calculation determines the minor head loss, or energy dissipation, caused by a specific fitting in a pipe. It uses the loss coefficient K of 10, the mean flow velocity v of 3 m/s m/s, and standard gravity g of 9.80665 m/s^2 m/s squared. The result, hL of 4.58872 m m, represents the height of fluid column equivalent to the energy lost due to turbulence and friction at that component.

<a id="reynolds_number"></a>

#### Reynolds number from density, velocity, diameter and viscosity, `reynolds_number`

One equation links Reynolds number (Re), density of the fluid (rho), mean flow velocity (v), pipe or body diameter (D) and dynamic viscosity of the fluid (mu). Give all but one of them and it is solved for the one left out, by default Reynolds number (Re).

$$
\mathrm{Re} = \frac{\rho v D}{\mu}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rho` | in | density of the fluid | kg/m^3 |
| `v` | in | mean flow velocity | m/s |
| `D` | in | pipe or body diameter | m |
| `mu` | in | dynamic viscosity of the fluid | Pa*s |
| `Re` | out | Reynolds number | - |

**Applies when:** Newtonian fluid; the diameter as the characteristic length; flow in a circular pipe or past a sphere or cylinder of that diameter.

```shell
$ thermofluids solve reynolds_number "rho=998 kg/m^3" "v=1.5 m/s" "D=0.05 m" "mu=0.001 Pa*s"
Reynolds number (Re) = 74850
  by: Reynolds number from density, velocity, diameter and viscosity  (Re = rho*v*D/mu)
  rho = 998 kg/m^3   [998 kg/m^3]
  v = 1.5 m/s   [1.5 m/s]
  D = 0.05 m   [0.05 m]
  mu = 0.001 Pa*s   [0.001 Pa*s]
  assumes: Newtonian fluid; the diameter as the characteristic length; the situation is: fluid flow
```

**What the worked example means:** This calculation determines the Reynolds number, which compares inertial forces to viscous forces in a fluid flow. Using the fluid density, mean velocity, characteristic diameter, and dynamic viscosity, the result is 74850. This dimensionless value helps predict whether the flow will be laminar or turbulent under the given conditions.

<a id="reynolds_number_kinematic"></a>

#### Reynolds number from velocity, diameter and kinematic viscosity, `reynolds_number_kinematic`

One equation links Reynolds number (Re), mean flow velocity (v), pipe or body diameter (D) and kinematic viscosity of the fluid (nu). Give all but one of them and it is solved for the one left out, by default Reynolds number (Re).

$$
\mathrm{Re} = \frac{v D}{\nu}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `v` | in | mean flow velocity | m/s |
| `D` | in | pipe or body diameter | m |
| `nu` | in | kinematic viscosity of the fluid | m^2/s |
| `Re` | out | Reynolds number | - |

**Applies when:** Newtonian fluid; the diameter as the characteristic length; flow in a circular pipe or past a sphere or cylinder of that diameter.

```shell
$ thermofluids solve reynolds_number_kinematic "v=10 m/s" "D=0.03 m" "nu=1.5e-05 m^2/s"
Reynolds number (Re) = 20000
  by: Reynolds number from velocity, diameter and kinematic viscosity  (Re = v*D/nu)
  v = 10 m/s   [10 m/s]
  D = 0.03 m   [0.03 m]
  nu = 1.5e-05 m^2/s   [1.5e-05 m^2/s]
  assumes: Newtonian fluid; the diameter as the characteristic length; the situation is: fluid flow
```

**What the worked example means:** This calculation determines the Reynolds number, a dimensionless quantity that predicts flow patterns by comparing inertial forces to viscous forces. Using the mean flow velocity, pipe or body diameter, and kinematic viscosity, the result is 20000. Practitioners use this value to distinguish between laminar and turbulent flow regimes in Newtonian fluids.

<a id="swamee_jain_friction_factor"></a>

#### Swamee-Jain explicit friction factor for turbulent pipe flow, `swamee_jain_friction_factor`

One equation links Darcy friction factor (f), absolute roughness of the pipe wall (eps), inner diameter of the pipe (D) and Reynolds number (Re). Give all but one of them and it is solved for the one left out, by default Darcy friction factor (f).

$$
f = \frac{0.25}{\left(\frac{\ln\left(\frac{\varepsilon}{3.7 D} + \left(\frac{6.97}{\mathrm{Re}}\right)^{\frac{9}{10}}\right)}{\ln\left(10\right)}\right)^{2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `eps` | in | absolute roughness of the pipe wall | m |
| `D` | in | inner diameter of the pipe | m |
| `Re` | in | Reynolds number | - |
| `f` | out | Darcy friction factor | - |

**Applies when:** fully developed turbulent flow in a circular pipe; Darcy friction factor; 5000 < Re < 1e8 and 1e-6 < eps/D < 0.01.

```shell
$ thermofluids solve swamee_jain_friction_factor "eps=4.5e-05 m" "D=0.1 m" "Re=100000 1"
Darcy friction factor (f) = 0.02019568646
  by: Swamee-Jain explicit friction factor for turbulent pipe flow  (f = 0.25/(log(eps/(3.7*D) + (6.97/Re)**(9/10))/log(10))**2)
  eps = 4.5e-05 m   [4.5e-05 m]
  D = 0.1 m   [0.1 m]
  Re = 100000   [100000 1]
  assumes: fully developed turbulent flow in a circular pipe; Darcy friction factor; the situation is: pipe, swamee jain
```

**What the worked example means:** This calculation determines the Darcy friction factor, 0.0201957, for turbulent flow in a circular pipe. The result is derived from an explicit approximation of the Colebrook equation, using the Reynolds number 100000, pipe diameter 0.1 m, and absolute roughness 4.5e-05 m. A lower value indicates less resistance to flow, while a higher value signifies greater energy loss due to friction along the pipe walls.

### Psychrometrics (7)

<a id="dew_point_temperature"></a>

#### Dew-point temperature from dry-bulb temperature and relative humidity, `dew_point_temperature`

The temperature at which moist air of a given relative humidity becomes saturated on cooling.

$$
p_{ws}(T_{dp}) = \phi\,p_{ws}(T)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | dry-bulb temperature | K |
| `RH` | in | relative humidity (fraction, or % written) | - |
| `T_dew` | out | dew-point temperature | K |
| `p_v` | out | partial pressure of the water vapour | Pa |

**Applies when:** 0 < RH <= 1; -100 °C <= T <= 200 °C.

```shell
$ thermofluids solve dew_point_temperature "T=25 °C" RH=0.6
T_dew = 289.8511319 K
p_v = 1901.529882 Pa
  by: Dew-point temperature from dry-bulb temperature and relative humidity  (p_v = RH p_ws(T), then p_ws(T_dew) = p_v solved by bisection to the last digit)
  T = 298.15 K
  RH = 0.6
  assumes: 0 < RH <= 1; -100 °C <= T <= 200 °C
```

**What the worked example means:** This calculation determines the dew-point temperature, which is 289.851 K, for moist air at a dry-bulb temperature of 25 °C and a relative humidity of 0.6. It also computes the partial pressure of the water vapour as 1901.53 Pa. The dew point represents the temperature to which the air must be cooled at constant pressure to become saturated, indicating the absolute moisture content independent of the current air temperature.

<a id="humidity_ratio_from_relative_humidity"></a>

#### Humidity ratio from temperature, relative humidity and pressure, `humidity_ratio_from_relative_humidity`

Mass of water vapour per kilogram of dry air in moist air of a given relative humidity.

$$
W = 0.621945\,\frac{\phi p_{ws}}{P - \phi p_{ws}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | dry-bulb temperature | K |
| `RH` | in | relative humidity (fraction, or % written) | - |
| `P` | in | total (barometric) pressure | Pa |
| `W` | out | humidity ratio (kg/kg dry air) | - |
| `p_v` | out | partial pressure of the water vapour | Pa |

**Applies when:** 0 < RH <= 1; P greater than the vapour pressure.

```shell
$ thermofluids solve humidity_ratio_from_relative_humidity "T=30 °C" RH=0.5 "P=101325 Pa"
W = 0.01331020384
p_v = 2123.015122 Pa
  by: Humidity ratio from temperature, relative humidity and pressure  (p_v = RH p_ws(T); W = 0.621945 p_v / (P - p_v))
  T = 303.15 K
  RH = 0.5
  P = 101325 Pa
  assumes: 0 < RH <= 1; P greater than the vapour pressure
```

**What the worked example means:** This calculation determines the mass of water vapour per kilogram of dry air, known as the humidity ratio. It uses the dry-bulb temperature, relative humidity, and total pressure to find the partial pressure of water vapour. The resulting humidity ratio is 0.0133102, which helps assess moisture content for heating, ventilation, and air conditioning systems.

<a id="humidity_ratio_vapour_pressure"></a>

#### Humidity ratio from the partial pressure of water vapour, `humidity_ratio_vapour_pressure`

One equation links humidity ratio (kg water vapour per kg dry air) (W), partial pressure of water vapour (pv) and total (barometric) pressure of the moist air (P). Give all but one of them and it is solved for the one left out, by default humidity ratio (kg water vapour per kg dry air) (W).

$$
W = \frac{0.621945 p_{v}}{P - p_{v}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `pv` | in | partial pressure of water vapour | Pa |
| `P` | in | total (barometric) pressure of the moist air | Pa |
| `W` | out | humidity ratio (kg water vapour per kg dry air) | - |

**Applies when:** moist air as an ideal-gas mixture; ratio of molar masses 0.621945; atmospheric air-conditioning and drying calculations.

```shell
$ thermofluids solve humidity_ratio_vapour_pressure "pv=2000 Pa" "P=101325 Pa"
humidity ratio (kg water vapour per kg dry air) (W) = 0.01252343317
  by: Humidity ratio from the partial pressure of water vapour  (W = 0.621945*pv/(P - pv))
  pv = 2000 Pa   [2000 Pa]
  P = 101325 Pa   [101325 Pa]
  assumes: moist air as an ideal-gas mixture; ratio of molar masses 0.621945; the situation is: moist air
```

**What the worked example means:** This calculation determines the humidity ratio, which represents the mass of water vapour per unit mass of dry air. It uses the partial pressure of water vapour, given as 2000 Pa, and the total barometric pressure of the moist air, given as 101325 Pa. The resulting value, 0.0125234, indicates how much moisture is present in the air mixture under ideal gas assumptions, a key parameter for air-conditioning and drying processes.

<a id="saturation_vapour_pressure"></a>

#### Saturation pressure of water vapour: `saturation_vapour_pressure`

The saturation pressure of water vapour over liquid water (above 0.01 °C) or ice, -100 °C to 200 °C.

$$
\ln p_{ws} = C_8/T + C_9 + C_{10}T + C_{11}T^2 + C_{12}T^3 + C_{13}\ln T
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | temperature | K |
| `p_sat` | out | saturation pressure of water vapour | Pa |

**Applies when:** -100 °C <= T <= 200 °C.

```shell
$ thermofluids solve saturation_vapour_pressure "T=25 °C"
p_sat = 3169.21647 Pa
  by: Saturation pressure of water vapour  (ASHRAE (Hyland-Wexler) correlation for ln p_ws)
  T = 298.15 K
  assumes: -100 °C <= T <= 200 °C
```

**What the worked example means:** This calculation determines the equilibrium pressure of water vapour using the ASHRAE Hyland-Wexler correlation. Given an input temperature of 25 °C, the model computes the corresponding saturation pressure, which results in 3169.22 Pa. This value represents the pressure at which water vapour coexists with liquid or solid water phases under thermodynamic equilibrium.

<a id="moist_air_enthalpy"></a>

#### Specific enthalpy of moist air: `moist_air_enthalpy`

One equation links enthalpy per kilogram of dry air (h), dry-bulb temperature (T), humidity ratio (kg water vapour per kg dry air) (W), Specific heat (cpa), Specific heat (cpv), Latent heat (hg) and Temperature (T0). Give all but one of them and it is solved for the one left out, by default enthalpy per kilogram of dry air (h).

$$
h = \text{cpa} \left(T - T_{0}\right) + W \left(h_{g} + \text{cpv} \left(T - T_{0}\right)\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | dry-bulb temperature | K |
| `W` | in | humidity ratio (kg water vapour per kg dry air) | - |
| `cpa` | in | Specific heat (optional) | J/(kg*K) |
| `cpv` | in | Specific heat (optional) | J/(kg*K) |
| `hg` | in | Latent heat (optional) | J/kg |
| `T0` | in | Temperature (optional) | K |
| `h` | out | enthalpy per kilogram of dry air | J/kg |

**Applies when:** reference state: dry air and liquid water at 0 °C; constant specific heats; air-conditioning processes between about -40 and 60 °C.

```shell
$ thermofluids solve moist_air_enthalpy "T=298.15 K" "W=0.01 1" "cpa=1006 J/(kg*K)" "cpv=1860 J/(kg*K)" "hg=2501000 J/kg" "T0=273.15 K"
enthalpy per kilogram of dry air (h) = 50625 J/kg
  by: Specific enthalpy of moist air  (h = cpa*(T - T0) + W*(hg + cpv*(T - T0)))
  T = 298.15 K   [298.15 K]
  W = 0.01   [0.01 1]
  cpa = 1006 J/(kg*K)   [1006 J/(kg*K)]
  cpv = 1860 J/(kg*K)   [1860 J/(kg*K)]
  hg = 2501000 J/kg   [2.501e+06 J/kg]
  T0 = 273.15 K   [273.15 K]
  assumes: reference state: dry air and liquid water at 0 °C; constant specific heats; the situation is: moist air
```

**What the worked example means:** This calculation determines the specific enthalpy of moist air, expressed as 50625 J/kg. The result is derived from the dry-bulb temperature 298.15 K, the humidity ratio 0.01, and material properties including the specific heat of dry air 1006 J/(kg*K), the specific heat of water vapor 1860 J/(kg*K), and the latent heat of vaporization 2.501e+06 J/kg, all referenced against a baseline temperature 273.15 K. This value represents the total energy content per kilogram of dry air, combining both sensible heat due to temperature differences and latent heat associated with the moisture present. It is a critical parameter for assessing cooling loads and heating requirements in air-conditioning systems.

<a id="moist_air_specific_volume"></a>

#### Specific volume of moist air per kilogram of dry air, `moist_air_specific_volume`

One equation links specific volume per kilogram of dry air (v), dry-bulb temperature (T), humidity ratio (kg water vapour per kg dry air) (W), total (barometric) pressure of the moist air (P) and Specific gas constant (Ra). Give all but one of them and it is solved for the one left out, by default specific volume per kilogram of dry air (v).

$$
v = \frac{\mathrm{Ra} T \left(1 + 1.607858 W\right)}{P}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | dry-bulb temperature | K |
| `W` | in | humidity ratio (kg water vapour per kg dry air) | - |
| `P` | in | total (barometric) pressure of the moist air | Pa |
| `Ra` | in | Specific gas constant (optional) | J/(kg*K) |
| `v` | out | specific volume per kilogram of dry air | m^3/kg |

**Applies when:** moist air as an ideal-gas mixture; fan and duct sizing in air conditioning.

```shell
$ thermofluids solve moist_air_specific_volume "T=298.15 K" "W=0.01 1" "P=101325 Pa" "Ra=287.042 J/(kg*K)"
specific volume per kilogram of dry air (v) = 0.8582048108 m^3/kg
  by: Specific volume of moist air per kilogram of dry air  (v = Ra*T*(1 + 1.607858*W)/P)
  T = 298.15 K   [298.15 K]
  W = 0.01   [0.01 1]
  P = 101325 Pa   [101325 Pa]
  Ra = 287.042 J/(kg*K)   [287.042 J/(kg*K)]
  assumes: moist air as an ideal-gas mixture; the situation is: moist air
```

**What the worked example means:** This calculation determines the specific volume of moist air, which represents the total volume occupied by one kilogram of dry air plus its associated water vapor. Using the ideal gas law for a mixture, the result depends on the dry-bulb temperature, humidity ratio, and total pressure. The computed value of 0.858205 m^3/kg is critical for sizing fans and ducts in air conditioning systems, as it converts mass flow rates into volumetric flow rates needed for equipment selection.

<a id="wet_bulb_temperature"></a>

#### Thermodynamic wet-bulb temperature: `wet_bulb_temperature`

The wet-bulb temperature of moist air from its dry-bulb temperature, relative humidity and pressure.

$$
W = \frac{(2501 - 2.326\,t^*)W_s^* - 1.006(t - t^*)}{2501 + 1.86\,t - 4.186\,t^*}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | dry-bulb temperature | K |
| `RH` | in | relative humidity (fraction, or % written) | - |
| `P` | in | total (barometric) pressure | Pa |
| `T_wb` | out | thermodynamic wet-bulb temperature | K |
| `W` | out | humidity ratio | - |

**Applies when:** 0 < RH <= 1; 0 °C <= T <= 50 °C for the liquid-water branch.

```shell
$ thermofluids solve wet_bulb_temperature "T=30 °C" RH=0.4 "P=101325 Pa"
T_wb = 293.2143471 K
W = 0.01060278119
  by: Thermodynamic wet-bulb temperature  (the ASHRAE psychrometric equation W(T, T_wb, P) = W solved for T_wb by bisection)
  T = 303.15 K
  RH = 0.4
  P = 101325 Pa
  assumes: 0 < RH <= 1; 0 °C <= T <= 50 °C for the liquid-water branch
```

**What the worked example means:** This calculation determines the thermodynamic wet-bulb temperature of moist air using its dry-bulb temperature, relative humidity, and total pressure. The result shows that for a dry-bulb temperature of 30 °C, relative humidity of 0.4, and pressure of 101325 Pa, the wet-bulb temperature is 293.214 K and the humidity ratio is 0.0106028. These values indicate the cooling potential via evaporation and are critical for assessing human thermal comfort in heating, ventilation, and air conditioning systems.

### Pumps (2)

<a id="hydraulic_power"></a>

#### Hydraulic power delivered to a fluid: `hydraulic_power`

One equation links power delivered to the fluid (P), density of the fluid (rho), Acceleration (g), volume flow rate (Q) and head added by the pump (H). Give all but one of them and it is solved for the one left out, by default power delivered to the fluid (P).

$$
P = \rho g Q H
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rho` | in | density of the fluid | kg/m^3 |
| `g` | in | Acceleration (optional) | m/s^2 |
| `Q` | in | volume flow rate | m^3/s |
| `H` | in | head added by the pump | m |
| `P` | out | power delivered to the fluid | W |

**Applies when:** incompressible liquid; steady flow; a pump adding head H to a liquid stream.

```shell
$ thermofluids solve hydraulic_power "rho=1000 kg/m^3" "g=9.80665 m/s^2" "Q=0.05 m^3/s" "H=20 m"
power delivered to the fluid (P) = 9806.65 W
  by: Hydraulic power delivered to a fluid  (P = rho*g*Q*H)
  rho = 1000 kg/m^3   [1000 kg/m^3]
  g = 9.80665 m/s^2   [9.80665 m/s^2]
  Q = 0.05 m^3/s   [0.05 m^3/s]
  H = 20 m   [20 m]
  assumes: incompressible liquid; steady flow; the situation is: pump
```

**What the worked example means:** This calculation determines the theoretical power imparted to a fluid by a pump, represented as 9806.65 W. It is derived from the fluid density 1000 kg/m^3, gravitational acceleration 9.80665 m/s^2, volume flow rate 0.05 m^3/s, and the head added by the pump 20 m. This result serves as a baseline for understanding the minimum energy required to move the liquid against the specified head.

<a id="pump_shaft_power"></a>

#### Shaft power required by a pump: `pump_shaft_power`

One equation links shaft power required (P), density of the fluid (rho), Acceleration (g), volume flow rate (Q), head added by the pump (H) and pump efficiency (a fraction) (eta). Give all but one of them and it is solved for the one left out, by default shaft power required (P).

$$
P = \frac{\rho g Q H}{\eta}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rho` | in | density of the fluid | kg/m^3 |
| `g` | in | Acceleration (optional) | m/s^2 |
| `Q` | in | volume flow rate | m^3/s |
| `H` | in | head added by the pump | m |
| `eta` | in | pump efficiency (a fraction) | - |
| `P` | out | shaft power required | W |

**Applies when:** incompressible liquid; steady flow; efficiency given as a fraction; sizing the drive of a pump.

```shell
$ thermofluids solve pump_shaft_power "rho=1000 kg/m^3" "g=9.80665 m/s^2" "Q=0.03 m^3/s" "H=25 m" "eta=0.8 1"
shaft power required (P) = 9193.734375 W
  by: Shaft power required by a pump  (P = rho*g*Q*H/eta)
  rho = 1000 kg/m^3   [1000 kg/m^3]
  g = 9.80665 m/s^2   [9.80665 m/s^2]
  Q = 0.03 m^3/s   [0.03 m^3/s]
  H = 25 m   [25 m]
  eta = 0.8   [0.8 1]
  assumes: incompressible liquid; steady flow; efficiency given as a fraction; the situation is: pump
```

**What the worked example means:** This calculation determines the mechanical power input needed at the pump shaft to achieve a specific hydraulic output. It uses the fluid density 1000 kg/m^3, gravitational acceleration 9.80665 m/s^2, volume flow rate 0.03 m^3/s, and total head added 25 m, divided by the pump efficiency 0.8. The result, 9193.73 W, represents the actual electrical or mechanical load on the driver, accounting for internal losses within the pump mechanism.

### Radiation (5)

<a id="blackbody_emissive_power"></a>

#### Emissive power of a blackbody (Stefan-Boltzmann law), `blackbody_emissive_power`

One equation links emissive power per unit area (E), Stefan boltzmann constant (sigma) and absolute temperature of the surface (T). Give all but one of them and it is solved for the one left out, by default emissive power per unit area (E).

$$
E = \sigma T^{4}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `sigma` | in | Stefan boltzmann constant (optional) | W/(m^2*K^4) |
| `T` | in | absolute temperature of the surface | K |
| `E` | out | emissive power per unit area | W/m^2 |

**Applies when:** an ideal blackbody; absolute temperature; total hemispherical emission of a black surface.

```shell
$ thermofluids solve blackbody_emissive_power "sigma=5.670374419e-08 W/(m^2*K^4)" "T=1000 K"
emissive power per unit area (E) = 56703.74419 W/m^2
  by: Emissive power of a blackbody (Stefan-Boltzmann law)  (E = sigma*T**4)
  sigma = 5.670374419e-08 W/(m^2*K^4)   [5.67037e-08 W/(m^2*K^4)]
  T = 1000 K   [1000 K]
  assumes: an ideal blackbody; absolute temperature; the situation is: radiation, blackbody
```

**What the worked example means:** This calculation determines the total hemispherical emissive power of an ideal blackbody surface. It uses the Stefan-Boltzmann law, which states that emission scales with the fourth power of the absolute temperature. With a surface temperature of 1000 K, the resulting radiant heat flux per unit area is 56703.7 W/m^2. This value represents the maximum theoretical radiation output for a perfect emitter at this specific thermal condition.

<a id="radiation_heat_transfer_coefficient"></a>

#### Linearised radiation heat transfer coefficient, `radiation_heat_transfer_coefficient`

One equation links radiation heat transfer coefficient (hr), surface emissivity (eps), Stefan boltzmann constant (sigma), surface temperature (Ts) and temperature of the surroundings (Tsur). Give all but one of them and it is solved for the one left out, by default radiation heat transfer coefficient (hr).

$$
h_{r} = \varepsilon \sigma \left(T_{s} + \text{Tsur}\right) \left(T_{s}^{2} + \text{Tsur}^{2}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `eps` | in | surface emissivity | - |
| `sigma` | in | Stefan boltzmann constant (optional) | W/(m^2*K^4) |
| `Ts` | in | surface temperature | K |
| `Tsur` | in | temperature of the surroundings | K |
| `hr` | out | radiation heat transfer coefficient | W/(m^2*K) |

**Applies when:** small grey surface in large surroundings; combining radiation with convection.

```shell
$ thermofluids solve radiation_heat_transfer_coefficient "eps=0.9 1" "sigma=5.670374419e-08 W/(m^2*K^4)" "Ts=400 K" "Tsur=300 K"
radiation heat transfer coefficient (hr) = 8.93083971 W/(m^2*K)
  by: Linearised radiation heat transfer coefficient  (hr = eps*sigma*(Ts + Tsur)*(Ts**2 + Tsur**2))
  eps = 0.9   [0.9 1]
  sigma = 5.670374419e-08 W/(m^2*K^4)   [5.67037e-08 W/(m^2*K^4)]
  Ts = 400 K   [400 K]
  Tsur = 300 K   [300 K]
  assumes: small grey surface in large surroundings; the situation is: radiation
```

**What the worked example means:** This calculation determines the linearised radiation heat transfer coefficient, denoted as hr, which allows radiative exchange to be combined with convective heat transfer using a single overall coefficient. The result is derived from the surface emissivity eps, the Stefan-Boltzmann constant sigma, and the absolute temperatures of the surface Ts and the surroundings Tsur. The final value obtained for the radiation heat transfer coefficient is 8.93084 W/(m^2*K), representing the effective conductance per unit area for thermal radiation under these specific temperature conditions.

<a id="radiation_parallel_plates"></a>

#### Net radiation between two large parallel grey plates, `radiation_parallel_plates`

One equation links net radiative heat flux from plate 1 to plate 2 (qf), Stefan boltzmann constant (sigma), temperature of the first plate (T1), temperature of the second plate (T2), emissivity of the first plate (e1) and emissivity of the second plate (e2). Give all but one of them and it is solved for the one left out, by default net radiative heat flux from plate 1 to plate 2 (qf).

$$
q_{f} = \frac{\sigma \left(T_{1}^{4} - T_{2}^{4}\right)}{\frac{1}{e_{1}} + \frac{1}{e_{2}} - 1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `sigma` | in | Stefan boltzmann constant (optional) | W/(m^2*K^4) |
| `T1` | in | temperature of the first plate | K |
| `T2` | in | temperature of the second plate | K |
| `e1` | in | emissivity of the first plate | - |
| `e2` | in | emissivity of the second plate | - |
| `qf` | out | net radiative heat flux from plate 1 to plate 2 | W/m^2 |

**Applies when:** infinite parallel grey diffuse plates; no participating medium; closely spaced large plates.

```shell
$ thermofluids solve radiation_parallel_plates "sigma=5.670374419e-08 W/(m^2*K^4)" "T1=800 K" "T2=400 K" "e1=0.8 1" "e2=0.6 1"
net radiative heat flux from plate 1 to plate 2 (qf) = 11360.47188 W/m^2
  by: Net radiation between two large parallel grey plates  (qf = sigma*(T1**4 - T2**4)/(1/e1 + 1/e2 - 1))
  sigma = 5.670374419e-08 W/(m^2*K^4)   [5.67037e-08 W/(m^2*K^4)]
  T1 = 800 K   [800 K]
  T2 = 400 K   [400 K]
  e1 = 0.8   [0.8 1]
  e2 = 0.6   [0.6 1]
  assumes: infinite parallel grey diffuse plates; no participating medium; the situation is: radiation, parallel plates
```

**What the worked example means:** This calculation determines the net radiative heat flux exchanged between two large, parallel grey plates. Using the Stefan-Boltzmann constant and the temperatures and emissivities of both surfaces, the result quantifies the energy transfer rate per unit area, which is 11360.5 W/m^2. This value indicates how effectively radiation moves from the hotter surface to the cooler one under the assumption that the plates are closely spaced and extensive.

<a id="radiation_net_exchange"></a>

#### Net radiation exchange of a small grey surface with large surroundings, `radiation_net_exchange`

One equation links rate of heat transfer (q), emissivity of the surface (eps), Stefan boltzmann constant (sigma), surface area (A), surface temperature (Ts) and temperature of the surroundings (Tsur). Give all but one of them and it is solved for the one left out, by default rate of heat transfer (q).

$$
q = \varepsilon \sigma A \left(T_{s}^{4} - \text{Tsur}^{4}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `eps` | in | emissivity of the surface | - |
| `sigma` | in | Stefan boltzmann constant (optional) | W/(m^2*K^4) |
| `A` | in | surface area | m^2 |
| `Ts` | in | surface temperature | K |
| `Tsur` | in | temperature of the surroundings | K |
| `q` | out | rate of heat transfer | W |

**Applies when:** grey diffuse surface; surroundings large compared with the surface; a small body inside a large enclosure.

```shell
$ thermofluids solve radiation_net_exchange "eps=0.8 1" "sigma=5.670374419e-08 W/(m^2*K^4)" "A=0.5 m^2" "Ts=500 K" "Tsur=300 K"
rate of heat transfer (q) = 1233.873474 W
  by: Net radiation exchange of a small grey surface with large surroundings  (q = eps*sigma*A*(Ts**4 - Tsur**4))
  eps = 0.8   [0.8 1]
  sigma = 5.670374419e-08 W/(m^2*K^4)   [5.67037e-08 W/(m^2*K^4)]
  A = 0.5 m^2   [0.5 m^2]
  Ts = 500 K   [500 K]
  Tsur = 300 K   [300 K]
  assumes: grey diffuse surface; surroundings large compared with the surface; the situation is: radiation
```

**What the worked example means:** This calculation determines the net rate of heat transfer by radiation, denoted as q, between a small grey diffuse surface and its large surroundings. The result, 1233.87 W, represents the energy flow in watts. It is derived from the emissivity of the surface, the Stefan-Boltzmann constant, the surface area, and the fourth-power difference between the surface temperature and the surrounding temperature. A positive value indicates heat loss from the surface to the environment, while a negative value indicates heat gain from the environment to the surface.

<a id="wien_displacement"></a>

#### Wien's displacement law: `wien_displacement`

One equation links wavelength of maximum spectral emission (lam), absolute temperature of the surface (T) and Wien constant (b). Give all but one of them and it is solved for the one left out, by default wavelength of maximum spectral emission (lam).

$$
\lambda = \frac{b}{T}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | absolute temperature of the surface | K |
| `b` | in | Wien constant (optional) | m*K |
| `lam` | out | wavelength of maximum spectral emission | m |

**Applies when:** blackbody (or grey body) emission; the peak of the Planck spectrum.

```shell
$ thermofluids solve wien_displacement "T=5800 K" "b=0.002897771955 m*K"
wavelength of maximum spectral emission (lam) = 4.996158543e-07 m
  by: Wien's displacement law  (lam = b/T)
  T = 5800 K   [5800 K]
  b = 0.002897771955 m*K   [0.00289777 m*K]
  assumes: blackbody (or grey body) emission; the situation is: peak emission
```

**What the worked example means:** This calculation determines the wavelength of maximum spectral emission, 4.99616e-07 m, for a surface at absolute temperature 5800 K. Applying Wien's displacement law identifies where the blackbody radiation spectrum peaks. This result indicates whether the thermal energy is primarily released in the infrared, visible, or ultraviolet range, which is critical for understanding heat transfer and optical properties.

### Thermodynamic cycles (6)

<a id="carnot_efficiency"></a>

#### Carnot efficiency of a reversible heat engine, `carnot_efficiency`

One equation links Carnot (maximum) efficiency (eta), temperature of the cold reservoir (Tc) and temperature of the hot reservoir (Th). Give all but one of them and it is solved for the one left out, by default Carnot (maximum) efficiency (eta).

$$
\eta = 1 - \frac{T_{c}}{T_{h}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Tc` | in | temperature of the cold reservoir | K |
| `Th` | in | temperature of the hot reservoir | K |
| `eta` | out | Carnot (maximum) efficiency | - |

**Applies when:** reversible cycle between two reservoirs; absolute temperatures; the upper limit of any heat engine between two temperatures.

```shell
$ thermofluids solve carnot_efficiency "Tc=300 K" "Th=600 K"
Carnot (maximum) efficiency (eta) = 0.5
  by: Carnot efficiency of a reversible heat engine  (eta = 1 - Tc/Th)
  Tc = 300 K   [300 K]
  Th = 600 K   [600 K]
  assumes: reversible cycle between two reservoirs; absolute temperatures; the situation is: carnot
```

**What the worked example means:** This calculation determines the upper limit of efficiency for any heat engine working between two thermal reservoirs. Using the absolute temperatures of the hot source, given as 600 K, and the cold sink, given as 300 K, the model applies the formula eta equals one minus Tc divided by Th. The resulting value, 0.5, indicates that no real engine can convert more than this fraction of input heat into useful work under these specific temperature conditions.

<a id="carnot_cop_heat_pump"></a>

#### Coefficient of performance of a Carnot heat pump, `carnot_cop_heat_pump`

One equation links coefficient of performance (heating) (cop), temperature of the cold reservoir (Tc) and temperature of the hot reservoir (Th). Give all but one of them and it is solved for the one left out, by default coefficient of performance (heating) (cop).

$$
\text{cop} = \frac{T_{h}}{T_{h} - T_{c}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Tc` | in | temperature of the cold reservoir | K |
| `Th` | in | temperature of the hot reservoir | K |
| `cop` | out | coefficient of performance (heating) | - |

**Applies when:** reversible heat-pump cycle; absolute temperatures; the upper limit of a heat pump's COP.

```shell
$ thermofluids solve carnot_cop_heat_pump "Tc=275 K" "Th=295 K"
coefficient of performance (heating) (cop) = 14.75
  by: Coefficient of performance of a Carnot heat pump  (cop = Th/(Th - Tc))
  Tc = 275 K   [275 K]
  Th = 295 K   [295 K]
  assumes: reversible heat-pump cycle; absolute temperatures; the situation is: carnot, heat pump
```

**What the worked example means:** This calculation determines the upper limit of efficiency for a Carnot heat pump, which represents the best possible performance under ideal reversible conditions. The result, 14.75, indicates how many units of heat are delivered to the warm space for every unit of work input. This value is derived using the absolute temperatures of the hot reservoir, 295 K, and the cold reservoir, 275 K. A higher coefficient signifies greater energy efficiency in moving heat rather than generating it directly.

<a id="carnot_cop_refrigerator"></a>

#### Coefficient of performance of a Carnot refrigerator, `carnot_cop_refrigerator`

One equation links coefficient of performance (cooling) (cop), temperature of the cold reservoir (Tc) and temperature of the hot reservoir (Th). Give all but one of them and it is solved for the one left out, by default coefficient of performance (cooling) (cop).

$$
\text{cop} = \frac{T_{c}}{T_{h} - T_{c}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Tc` | in | temperature of the cold reservoir | K |
| `Th` | in | temperature of the hot reservoir | K |
| `cop` | out | coefficient of performance (cooling) | - |

**Applies when:** reversible refrigeration cycle; absolute temperatures; the upper limit of a refrigerator's COP.

```shell
$ thermofluids solve carnot_cop_refrigerator "Tc=270 K" "Th=300 K"
coefficient of performance (cooling) (cop) = 9
  by: Coefficient of performance of a Carnot refrigerator  (cop = Tc/(Th - Tc))
  Tc = 270 K   [270 K]
  Th = 300 K   [300 K]
  assumes: reversible refrigeration cycle; absolute temperatures; the situation is: carnot, refrigerator
```

**What the worked example means:** This calculation determines the theoretical maximum efficiency of a refrigerator operating between two thermal reservoirs. Using the cold reservoir temperature of 270 K and the hot reservoir temperature of 300 K, the resulting coefficient of performance is 9. This dimensionless value represents the ratio of heat removed from the cold space to the work input required, serving as an upper limit for real-world systems.

<a id="brayton_efficiency_air"></a>

#### Thermal efficiency of the air-standard Brayton cycle, `brayton_efficiency_air`

One equation links thermal efficiency of the cycle (eta) and compressor pressure ratio (rp). Give all but one of them and it is solved for the one left out, by default thermal efficiency of the cycle (eta).

$$
\eta = 1 - r_{p}^{\frac{-2}{7}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rp` | in | compressor pressure ratio | - |
| `eta` | out | thermal efficiency of the cycle | - |

**Applies when:** cold-air-standard: air as an ideal gas with gamma = 1.4; isentropic compressor and turbine; idealised gas-turbine cycles.

```shell
$ thermofluids solve brayton_efficiency_air "rp=10 1"
thermal efficiency of the cycle (eta) = 0.4820525321
  by: Thermal efficiency of the air-standard Brayton cycle  (eta = 1 - rp**(-2/7))
  rp = 10   [10 1]
  assumes: cold-air-standard: air as an ideal gas with gamma = 1.4; isentropic compressor and turbine; the situation is: brayton, air
```

**What the worked example means:** This calculation determines the theoretical thermal efficiency of an idealized gas-turbine power plant cycle using the cold-air-standard assumption. The result, 0.482053, indicates how effectively heat input is converted to work for a compressor pressure ratio of 10. Under these ideal conditions, efficiency rises as the pressure ratio increases, providing a fundamental metric for evaluating jet engines and stationary power generation systems.

<a id="diesel_efficiency_air"></a>

#### Thermal efficiency of the air-standard Diesel cycle, `diesel_efficiency_air`

One equation links thermal efficiency of the cycle (eta), compression ratio V1/V2 (r) and cutoff ratio V3/V2 (rc). Give all but one of them and it is solved for the one left out, by default thermal efficiency of the cycle (eta).

$$
\eta = 1 - \frac{r^{\frac{-2}{5}} \left(r_{c}^{\frac{7}{5}} - 1\right)}{\frac{7 \left(r_{c} - 1\right)}{5}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `r` | in | compression ratio V1/V2 | - |
| `rc` | in | cutoff ratio V3/V2 | - |
| `eta` | out | thermal efficiency of the cycle | - |

**Applies when:** cold-air-standard: air as an ideal gas with gamma = 1.4; heat added at constant pressure; idealised compression-ignition engines.

```shell
$ thermofluids solve diesel_efficiency_air "r=18 1" "rc=2 1"
thermal efficiency of the cycle (eta) = 0.6315775314
  by: Thermal efficiency of the air-standard Diesel cycle  (eta = 1 - r**(-2/5)*(rc**(7/5) - 1)/(7*(rc - 1)/5))
  r = 18   [18 1]
  rc = 2   [2 1]
  assumes: cold-air-standard: air as an ideal gas with gamma = 1.4; heat added at constant pressure; the situation is: diesel, air
```

**What the worked example means:** This calculation determines the theoretical thermal efficiency of an idealized compression-ignition engine operating on the air-standard Diesel cycle. The result, 0.631578, is derived from the specific compression ratio 18 and the cutoff ratio 2. In this model, heat addition occurs at constant pressure, which distinguishes it from spark-ignition cycles. The efficiency reflects how effectively the engine converts fuel energy into work under these idealized conditions, assuming air behaves as an ideal gas with a specific heat ratio of one point four.

<a id="otto_efficiency_air"></a>

#### Thermal efficiency of the air-standard Otto cycle, `otto_efficiency_air`

One equation links thermal efficiency of the cycle (eta) and compression ratio V1/V2 (r). Give all but one of them and it is solved for the one left out, by default thermal efficiency of the cycle (eta).

$$
\eta = 1 - r^{\frac{-2}{5}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `r` | in | compression ratio V1/V2 | - |
| `eta` | out | thermal efficiency of the cycle | - |

**Applies when:** cold-air-standard: air as an ideal gas with gamma = 1.4; reversible adiabatic compression and expansion; idealised spark-ignition engines.

```shell
$ thermofluids solve otto_efficiency_air "r=8 1"
thermal efficiency of the cycle (eta) = 0.5647247184
  by: Thermal efficiency of the air-standard Otto cycle  (eta = 1 - r**(-2/5))
  r = 8   [8 1]
  assumes: cold-air-standard: air as an ideal gas with gamma = 1.4; reversible adiabatic compression and expansion; the situation is: otto, air
```

**What the worked example means:** This calculation determines the ideal thermal efficiency of an air-standard Otto cycle. It uses the formula eta equals one minus r raised to the power of negative two-fifths, where r represents the compression ratio defined as the maximum cylinder volume divided by the minimum cylinder volume. The result, 0.564725, indicates the fraction of heat energy converted into useful work under ideal conditions. This value serves as a theoretical upper limit for spark-ignition engines operating with air as the working fluid.

### Transient conduction (4)

<a id="biot_number"></a>

#### Biot number: `biot_number`

One equation links Biot number (Bi), convection heat transfer coefficient (h), characteristic length (Lc) and thermal conductivity (k). Give all but one of them and it is solved for the one left out, by default Biot number (Bi).

$$
\mathrm{Bi} = \frac{h L_{c}}{k}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `h` | in | convection heat transfer coefficient | W/(m^2*K) |
| `Lc` | in | characteristic length | m |
| `k` | in | thermal conductivity | W/(m*K) |
| `Bi` | out | Biot number | - |

**Applies when:** Lc = volume / surface area for the lumped-capacitance test; judging whether a body's internal temperature is uniform (Bi < 0.1).

```shell
$ thermofluids solve biot_number "h=50 W/(m^2*K)" "Lc=0.01 m" "k=200 W/(m*K)"
Biot number (Bi) = 0.0025
  by: Biot number  (Bi = h*Lc/k)
  h = 50 W/(m^2*K)   [50 W/(m^2*K)]
  Lc = 0.01 m   [0.01 m]
  k = 200 W/(m*K)   [200 W/(m*K)]
  assumes: Lc = volume / surface area for the lumped-capacitance test; the situation is: biot
```

**What the worked example means:** This calculation determines the Biot number, defined as the product of the convection heat transfer coefficient and the characteristic length divided by the thermal conductivity. Using the given values for the convection heat transfer coefficient of 50 W/(m^2*K), the characteristic length of 0.01 m, and the thermal conductivity of 200 W/(m*K), the resulting dimensionless Biot number is 0.0025. This value assesses whether the temperature distribution within the body can be considered uniform for lumped capacitance analysis.

<a id="fourier_number"></a>

#### Fourier number: `fourier_number`

One equation links Fourier number (Fo), thermal diffusivity (alpha), elapsed time (t) and characteristic length (Lc). Give all but one of them and it is solved for the one left out, by default Fourier number (Fo).

$$
F_{o} = \frac{\alpha t}{L_{c}^{2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `alpha` | in | thermal diffusivity | m^2/s |
| `t` | in | elapsed time | s |
| `Lc` | in | characteristic length | m |
| `Fo` | out | Fourier number | - |

**Applies when:** constant properties; transient conduction charts and series solutions.

```shell
$ thermofluids solve fourier_number "alpha=1e-05 m^2/s" "t=60 s" "Lc=0.02 m"
Fourier number (Fo) = 1.5
  by: Fourier number  (Fo = alpha*t/Lc**2)
  alpha = 1e-05 m^2/s   [1e-05 m^2/s]
  t = 60 s   [60 s]
  Lc = 0.02 m   [0.02 m]
  assumes: constant properties; the situation is: fourier number
```

**What the worked example means:** The Fourier number, calculated as 1.5, represents dimensionless time in transient heat conduction. It compares the rate of heat conduction through a material to the rate at which that material stores thermal energy. A higher value suggests that heat has penetrated deeper into the body relative to its characteristic length scale over the elapsed time.

<a id="lumped_capacitance_temperature"></a>

#### Temperature of a lumped body cooling or heating in a fluid, `lumped_capacitance_temperature`

One equation links temperature of the body at time t (T), temperature of the fluid far from the surface (Tinf), initial temperature of the body (Ti), convection heat transfer coefficient (h), surface area of the body (A), elapsed time (t), density of the body (rho), volume of the body (V) and specific heat of the body (c). Give all but one of them and it is solved for the one left out, by default temperature of the body at time t (T).

$$
T = \text{Tinf} + \left(T_{i} - \text{Tinf}\right) e^{\frac{-h A t}{\rho V c}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Tinf` | in | temperature of the fluid far from the surface | K |
| `Ti` | in | initial temperature of the body | K |
| `h` | in | convection heat transfer coefficient | W/(m^2*K) |
| `A` | in | surface area of the body | m^2 |
| `t` | in | elapsed time | s |
| `rho` | in | density of the body | kg/m^3 |
| `V` | in | volume of the body | m^3 |
| `c` | in | specific heat of the body | J/(kg*K) |
| `T` | out | temperature of the body at time t | K |

**Applies when:** Biot number below about 0.1 (uniform internal temperature); constant h and properties; small or highly conductive bodies.

```shell
$ thermofluids solve lumped_capacitance_temperature "Tinf=298.15 K" "Ti=573.15 K" "h=40 W/(m^2*K)" "A=0.002 m^2" "t=120 s" "rho=7800 kg/m^3" "V=5e-06 m^3" "c=450 J/(kg*K)"
temperature of the body at time t (T) = 457.286536 K
  by: Temperature of a lumped body cooling or heating in a fluid  (T = Tinf + (Ti - Tinf)*exp(-h*A*t/(rho*V*c)))
  Tinf = 298.15 K   [298.15 K]
  Ti = 573.15 K   [573.15 K]
  h = 40 W/(m^2*K)   [40 W/(m^2*K)]
  A = 0.002 m^2   [0.002 m^2]
  t = 120 s   [120 s]
  rho = 7800 kg/m^3   [7800 kg/m^3]
  V = 5e-06 m^3   [5e-06 m^3]
  c = 450 J/(kg*K)   [450 J/(kg*K)]
  assumes: Biot number below about 0.1 (uniform internal temperature); constant h and properties; the situation is: lumped
```

**What the worked example means:** This calculation determines the uniform temperature of a solid body after it has been exposed to a surrounding fluid for a specific duration. The result, 457.287 K, represents the current state of the object, derived from its initial temperature 573.15 K, the far-field fluid temperature 298.15 K, and the thermal properties including density 7800 kg/m^3, volume 5e-06 m^3, and specific heat 450 J/(kg*K). The rate of change is governed by the convection coefficient 40 W/(m^2*K) and surface area 0.002 m^2 over the elapsed time 120 s. This value indicates how close the object is to reaching thermal equilibrium with its environment.

<a id="lumped_time_constant"></a>

#### Thermal time constant of a lumped body: `lumped_time_constant`

One equation links thermal time constant (tau), density of the body (rho), volume of the body (V), specific heat of the body (c), convection heat transfer coefficient (h) and surface area of the body (A). Give all but one of them and it is solved for the one left out, by default thermal time constant (tau).

$$
\tau = \frac{\rho V c}{h A}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `rho` | in | density of the body | kg/m^3 |
| `V` | in | volume of the body | m^3 |
| `c` | in | specific heat of the body | J/(kg*K) |
| `h` | in | convection heat transfer coefficient | W/(m^2*K) |
| `A` | in | surface area of the body | m^2 |
| `tau` | out | thermal time constant | s |

**Applies when:** Biot number below about 0.1; constant h and properties; thermocouples and small bodies.

```shell
$ thermofluids solve lumped_time_constant "rho=7800 kg/m^3" "V=5e-06 m^3" "c=450 J/(kg*K)" "h=40 W/(m^2*K)" "A=0.002 m^2"
thermal time constant (tau) = 219.375 s
  by: Thermal time constant of a lumped body  (tau = rho*V*c/(h*A))
  rho = 7800 kg/m^3   [7800 kg/m^3]
  V = 5e-06 m^3   [5e-06 m^3]
  c = 450 J/(kg*K)   [450 J/(kg*K)]
  h = 40 W/(m^2*K)   [40 W/(m^2*K)]
  A = 0.002 m^2   [0.002 m^2]
  assumes: Biot number below about 0.1; constant h and properties; the situation is: lumped
```

**What the worked example means:** This calculation determines the thermal time constant, denoted as tau, which represents the time required for a lumped body to reach approximately sixty-three percent of its total temperature change. The value is derived from the body's density, volume, and specific heat divided by the product of the convection heat transfer coefficient and surface area. A result of 219.375 s indicates how quickly the object responds to changes in its surrounding temperature environment.

Questions and reports: support@ai999x.com. AI999X, The Future Is Self-Improving.
