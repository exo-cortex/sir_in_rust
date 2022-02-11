# a basic SIR model programmed in rust

This project serves mostly my learning the rust programming language. In order to do something interesting I decided to use the SIR model as an example. SIR stands for "susceptible", "infected" and "removed". It is a system of coupled differential equations that model the different sets of a population during an epidemic. The variables S+I+R=1 represent fractions. At the arrival of a new disease everybody is susceptible, so S is very near to 1, I is very small and R=0 (if I starts at 0 nothing ever happens, because no disease exists). The removed fraction consists of the recovered and/or dead people. Three parameters are important for the dynamics: the infectious rate *beta*, recovery rate *gamma* and death/birth rate **mu**. For further reading check out the wikipedia page of the [SIR model](https://en.wikipedia.org/wiki/Compartmental_models_in_epidemiology). R_0 = beta/gamma is the basic reproduction number which is characteristic for different diseases.

# features
- integration with Runge-Kutta-4 method.
- writing out simplified timeseries of the dynamic variables S, I, R using the [RDP-algorithm](https://en.wikipedia.org/wiki/Ramer%E2%80%93Douglas%E2%80%93Peucker_algorithm)
