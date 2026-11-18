# Thermofluids: the desktop app, user guide

**Thermofluids**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Thermofluids covers fluid statics and dynamics, pipe flow, boundary layers, aerodynamics, heat conduction, convection and radiation, heat exchangers, thermodynamic cycles, psychrometrics and combustion. Its desktop app holds all 106 calculations in 22 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-thermofluids-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-thermofluids-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-thermofluids-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-thermofluids.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-thermofluids-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-thermofluids-1.6.5-linux-x86_64.tar.gz
./ai999x-thermofluids-1.6.5-linux-x86_64/ai999x-thermofluids
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Thermofluids app, opened on Reynolds number from density, velocity, diameter and viscosity with its worked example.](https://www.ai999x.com/?ax_shot=thermofluids-ai999x/hero.webp)

*The Thermofluids app, opened on Reynolds number from density, velocity, diameter and viscosity with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Pipe flow › Reynolds number from density, velocity, diameter and viscosity (or type part of its name in the search box). It opens with a worked example filled in: “Water with density 998 kg/m^3 and dynamic viscosity 0.001 Pa*s flows at an average velocity of 1.5 m/s in a pipe of diameter 0.05 m. What is the Reynolds number?” The answer: Re = 74850 1.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![Reynolds number from density, velocity, diameter and viscosity: the live scene of the worked example.](https://www.ai999x.com/?ax_shot=thermofluids-ai999x/scene-1.webp)

*Reynolds number from density, velocity, diameter and viscosity: the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it, for example “Water with density 998 kg/m^3 and dynamic viscosity 0.001 Pa*s flows at an average velocity of 1.5 m/s in a pipe of diameter 0.05 m. What is the Reynolds number?” The app reads the quantities and their units, picks the calculation and answers; the command-line program reads it the same way and printed:

```text
Read as: Reynolds number from density, velocity, diameter and viscosity
Reynolds number (Re) = 74850
  by: Reynolds number from density, velocity, diameter and viscosity  (Re = rho*v*D/mu)
```

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Thermal efficiency of the air-standard Otto cycle](https://www.ai999x.com/?ax_shot=thermofluids-ai999x/scene-2.webp)

*Thermal efficiency of the air-standard Otto cycle*

![Dew-point temperature from dry-bulb temperature and relative humidity](https://www.ai999x.com/?ax_shot=thermofluids-ai999x/scene-3.webp)

*Dew-point temperature from dry-bulb temperature and relative humidity*

![Log-mean temperature difference of a heat exchanger](https://www.ai999x.com/?ax_shot=thermofluids-ai999x/scene-4.webp)

*Log-mean temperature difference of a heat exchanger*

![Lift coefficient of a thin symmetric airfoil](https://www.ai999x.com/?ax_shot=thermofluids-ai999x/scene-5.webp)

*Lift coefficient of a thin symmetric airfoil*

![Average skin-friction coefficient of a laminar flat plate](https://www.ai999x.com/?ax_shot=thermofluids-ai999x/scene-6.webp)

*Average skin-friction coefficient of a laminar flat plate*

![Adiabatic flame temperature with constant product specific heat](https://www.ai999x.com/?ax_shot=thermofluids-ai999x/scene-7.webp)

*Adiabatic flame temperature with constant product specific heat*

![Isentropic stagnation pressure of air from the Mach number](https://www.ai999x.com/?ax_shot=thermofluids-ai999x/scene-8.webp)

*Isentropic stagnation pressure of air from the Mach number*

![Reynolds number from density, velocity, diameter and viscosity: a chart, enlarged.](https://www.ai999x.com/?ax_shot=thermofluids-ai999x/chart-1.webp)

*Reynolds number from density, velocity, diameter and viscosity: a chart, enlarged.*

![Thermal efficiency of the air-standard Otto cycle: a chart, enlarged.](https://www.ai999x.com/?ax_shot=thermofluids-ai999x/chart-2.webp)

*Thermal efficiency of the air-standard Otto cycle: a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Thermofluids app in the dark theme.](https://www.ai999x.com/?ax_shot=thermofluids-ai999x/hero-dark.webp)

*The Thermofluids app in the dark theme.*

## 7. Every calculation

106 calculations in 22 topics; each has its own entry on the model's page (https://www.ai999x.com/models/thermofluids-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Aerodynamics | Lift coefficient of a thin symmetric airfoil |
| Boundary layers | Average skin-friction coefficient of a laminar flat plate, Displacement thickness of a laminar flat-plate boundary layer, Local skin-friction coefficient of a laminar flat plate, Momentum thickness of a laminar flat-plate boundary layer |
| Combustion | Adiabatic flame temperature with constant product specific heat, Equivalence ratio of a fuel-air mixture, Stoichiometric air-fuel ratio of a gaseous fuel |
| Compressible flow | Isentropic stagnation pressure of air from the Mach number, Mach number, Mach number behind a normal shock in air, Speed of sound in an ideal gas, Stagnation temperature from the Mach number, Static pressure behind a normal shock in air |
| Conduction | Critical radius of insulation on a cylinder, Fourier's law: heat conduction through a plane wall, Heat transfer rate from the overall heat transfer coefficient, Maximum temperature of a plane wall with uniform heat generation, Overall heat transfer coefficient of a plane wall with convection on both sides, Radial heat conduction through a cylindrical wall, Radial heat conduction through a spherical shell, Thermal diffusivity |
| Convection | Average Nusselt number for laminar flow over a flat plate, Churchill-Bernstein correlation for a cylinder in cross flow, Churchill-Chu correlation for natural convection on a vertical plate, Dittus-Boelter correlation, fluid being cooled, Dittus-Boelter correlation, fluid being heated, Gnielinski correlation for turbulent pipe flow, Heat transfer coefficient from the Nusselt number, Local Nusselt number for laminar flow over a flat plate, Newton's law of cooling (convection heat transfer), Prandtl number |
| External flow | Drag force from the drag coefficient, Lift force from the lift coefficient, Stokes drag on a small sphere, Terminal settling velocity of a small sphere (Stokes) |
| Fins | Efficiency of a pin fin with an insulated tip, Heat transfer rate of a very long fin |
| Fluid dynamics | Bernoulli's equation along a horizontal streamline, Continuity: speed after a change of pipe diameter, Dynamic pressure of a moving fluid, Speed of efflux from a hole (Torricelli's law) |
| Fluid flow | Bernoulli's equation with a change of height |
| Fluid statics | Absolute pressure at a depth in a liquid, Buoyant force (Archimedes' principle), Gauge pressure at a depth in a liquid, Hydraulic press (Pascal's principle), Hydrostatic force on a submerged plane surface |
| Gas processes | Actual outlet temperature of an adiabatic air compressor, Entropy change of an incompressible substance, Ideal gas law, Specific entropy change of an ideal gas, Temperature after an isentropic pressure change of air, Temperature after an isentropic volume change of air, Work done by an ideal gas in a reversible isothermal process |
| Heat exchangers | Effectiveness of a heat exchanger with one stream changing phase, Effectiveness of a parallel-flow heat exchanger, Heat added to a flowing stream (steady-flow energy balance), Heat exchanger rating by the effectiveness-NTU method, Heat transfer rate from the effectiveness, Log-mean temperature difference of a heat exchanger, Number of transfer units of a heat exchanger |
| Natural convection | Grashof number, Rayleigh number |
| Open channel | Froude number of an open-channel flow |
| Pipe flow | Blasius friction factor for turbulent flow in a smooth pipe, Darcy friction factor from the Colebrook equation, Darcy friction factor in laminar pipe flow, Darcy-Weisbach head loss in a pipe, Darcy-Weisbach pressure drop in a pipe, Haaland explicit friction factor for turbulent pipe flow, Hagen-Poiseuille flow rate in laminar pipe flow, Kinematic viscosity from dynamic viscosity and density, Minor head loss in a fitting, Reynolds number from density, velocity, diameter and viscosity, Reynolds number from velocity, diameter and kinematic viscosity, Swamee-Jain explicit friction factor for turbulent pipe flow |
| Psychrometrics | Dew-point temperature from dry-bulb temperature and relative humidity, Humidity ratio from temperature, relative humidity and pressure, Humidity ratio from the partial pressure of water vapour, Saturation pressure of water vapour, Specific enthalpy of moist air, Specific volume of moist air per kilogram of dry air, Thermodynamic wet-bulb temperature |
| Pumps | Hydraulic power delivered to a fluid, Shaft power required by a pump |
| Radiation | Emissive power of a blackbody (Stefan-Boltzmann law), Linearised radiation heat transfer coefficient, Net radiation between two large parallel grey plates, Net radiation exchange of a small grey surface with large surroundings, Wien's displacement law |
| Thermodynamic cycles | Carnot efficiency of a reversible heat engine, Coefficient of performance of a Carnot heat pump, Coefficient of performance of a Carnot refrigerator, Thermal efficiency of the air-standard Brayton cycle, Thermal efficiency of the air-standard Diesel cycle, Thermal efficiency of the air-standard Otto cycle |
| Thermodynamics | Boundary work of a constant-pressure (isobaric) process, Entropy change for heat exchanged at a constant temperature, Entropy generated when heat flows between two reservoirs, First law of thermodynamics: change of internal energy, Wet steam: quality and mixture properties from saturated values |
| Transient conduction | Biot number, Fourier number, Temperature of a lumped body cooling or heating in a fluid, Thermal time constant of a lumped body |

## 8. Other ways to use it

The same engine runs on the command line (`thermofluids`, its own guide), in Python (`pip install thermofluids-ai999x`, `import thermofluids_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/thermofluids-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/thermofluids-ai999x/
