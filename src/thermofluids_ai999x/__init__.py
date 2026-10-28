"""thermofluids-ai999x — offline thermofluids answers with their formula, steps and verification.

    import thermofluids_ai999x as m
    m.ask("Oil of density 900 kg/m^3 and viscosity 0.1 Pa*s flows in laminar flow at a mean velocity of 0.5 m/s "
          "in a pipe of diameter 0.02 m. What is the friction factor?").show()          # 0.711 (via Re = 90)
    m.ask("Dew point temperature: T = 25 °C, RH = 60%").show()                          # 289.85 K
    m.solve("lmtd", Th_in="150 °C", Th_out="90 °C", Tc_in="30 °C", Tc_out="70 °C", arrangement="counterflow")
    m.formulas()                       # every certified capability

99 certified capabilities: fluid statics (hydrostatic pressure, buoyancy, forces on gates), fluid dynamics (continuity,
Bernoulli, Torricelli, dynamic pressure), pipe flow (Reynolds number, laminar/Blasius/Swamee-Jain/Haaland/Colebrook
friction factors, Darcy-Weisbach, Hagen-Poiseuille, minor losses, pump power), external flow and aerodynamics (drag,
lift, Stokes drag and settling, thin-airfoil lift), compressible flow (Mach number, speed of sound, stagnation state,
normal shocks in air), Blasius boundary layers, conduction (plane wall, cylinder, sphere, U-value, critical insulation,
heat generation), convection (Newton cooling, Nusselt, Prandtl, Dittus-Boelter, Gnielinski, Churchill-Bernstein,
flat-plate, Grashof/Rayleigh, Churchill-Chu), radiation (Stefan-Boltzmann, grey exchange, parallel plates, Wien), fins,
lumped transients, heat exchangers (NTU, effectiveness, LMTD, effectiveness-NTU rating), cycles (Carnot, Otto, Diesel,
Brayton), ideal-gas processes and entropy, psychrometrics (humidity ratio, enthalpy, specific volume, saturation
pressure, dew point, wet bulb) and combustion (stoichiometric air-fuel ratio, equivalence ratio, adiabatic flame
temperature). Built by the AI999X model machine from the same engine as every domain; a question that is not
determined is refused rather than guessed. Percentages in word questions are refused (write 0.75, not 75%), except in
the documented `RH = 60%` forms.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.0.2"
_P = Product("thermofluids", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "__version__"]
