# thermofluids-ai999x — offline, verified thermofluids solver with step-by-step answers

[![tests](https://github.com/AI999X/thermofluids-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/thermofluids-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/thermofluids-ai999x)](https://pypi.org/project/thermofluids-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/thermofluids-ai999x)

**[Install](https://pypi.org/project/thermofluids-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/thermofluids-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/thermofluids-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

Field: **Engineering** · Tasks: Question Answering, Token Classification, Text Classification, Sentence Similarity, Feature Extraction

thermofluids-ai999x is an offline, deterministic Python solver that answers thermofluids questions with verified results from 126 certified capabilities, gives 1010 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**thermofluids-ai999x** is a compact, deterministic thermofluids engine for Python. It reads a question in plain English (or a typed call), picks a certified thermofluids capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified thermofluids capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 1010 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** fluid statics and dynamics, pipe flow, boundary layers, aerodynamics and compressible flow, heat conduction, convection and radiation, fins, heat exchangers, thermodynamic cycles, psychrometrics and combustion.

## Model overview

| Property | Value |
|---|---|
| Package | `thermofluids-ai999x` 1.0.0 (import `thermofluids_ai999x`) |
| Type | Deterministic neuro-symbolic thermofluids engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 27 certified derivations, 13 certified procedures, 86 certified relations |
| Knowledge pack | version 0.1.1 |
| Golden answers | 1010 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

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

Command line: `thermofluids-ai999x "<question>"`, `thermofluids-ai999x --capabilities`. The outputs above were produced by running this exact code on thermofluids-ai999x 1.0.0.

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Capabilities

27 certified derivations, 13 certified procedures, 86 certified relations. A few of them:

- **Derivations:** derive bernoulli horizontal; derive blasius local skin friction; derive blasius momentum thickness; derive brayton efficiency air; derive buoyant force; derive continuity speed; derive darcy weisbach head loss; derive darcy weisbach pressure drop; derive diesel efficiency air; derive dynamic pressure; derive effectiveness phase change; derive humidity ratio vapour pressure; derive hydraulic power; derive hydrostatic force plane; derive minor loss head; derive nusselt heat transfer coefficient; derive otto efficiency air; derive overall heat rate; derive overall u plane wall; derive plane wall conduction; derive pump shaft power; derive radiation net exchange; derive radiation parallel plates; derive reynolds number …
- **Procedures:** Churchill-Bernstein correlation for a cylinder in cross flow; Churchill-Chu correlation for natural convection on a vertical plate; Darcy friction factor from the Colebrook equation; Dew-point temperature from dry-bulb temperature and relative humidity; Efficiency of a pin fin with an insulated tip; Equivalence ratio of a fuel-air mixture; Gnielinski correlation for turbulent pipe flow; Heat exchanger rating by the effectiveness-NTU method; Humidity ratio from temperature, relative humidity and pressure; Log-mean temperature difference of a heat exchanger; Saturation pressure of water vapour; Stoichiometric air-fuel ratio of a gaseous fuel; Thermodynamic wet-bulb temperature
- **Relations:** Absolute pressure at a depth in a liquid; Actual outlet temperature of an adiabatic air compressor; Adiabatic flame temperature with constant product specific heat; Average Nusselt number for laminar flow over a flat plate; Average skin-friction coefficient of a laminar flat plate; Bernoulli's equation along a horizontal streamline; Biot number; Blasius friction factor for turbulent flow in a smooth pipe; Buoyant force (Archimedes' principle); Carnot efficiency of a reversible heat engine; Coefficient of performance of a Carnot heat pump; Coefficient of performance of a Carnot refrigerator; Continuity: speed after a change of pipe diameter; Critical radius of insulation on a cylinder; Darcy friction factor in laminar pipe flow; Darcy-Weisbach head loss in a pipe; Darcy-Weisbach pressure drop in a pipe; Displacement thickness of a laminar flat-plate boundary layer; Dittus-Boelter correlation, fluid being cooled; Dittus-Boelter correlation, fluid being heated; Drag force from the drag coefficient; Dynamic pressure of a moving fluid; Effectiveness of a heat exchanger with one stream changing phase; Effectiveness of a parallel-flow heat exchanger …

`thermofluids_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified thermofluids capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (fluid statics and dynamics, pipe flow, boundary layers, aerodynamics and compressible flow, heat conduction, convection and radiation, fins, heat exchangers, thermodynamic cycles, psychrometrics and combustion); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{thermofluids_ai999x_1_0_0,
  title  = {thermofluids-ai999x: an offline, verified thermofluids solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.0},
  url    = {https://huggingface.co/AI999X/thermofluids-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/thermofluids-ai999x/discussions or visit https://ai999x.com.

## Support the project

thermofluids-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/thermofluids-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/thermofluids-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/thermofluids-ai999x/tree/main/onnx).
