# wasmgp
A library for implementing genetic programming using WebAssembly

# Building Wasm Code using Genetic Programming
You will need to define the structure of what you want the output code to be able to do. At the minimum, you will need
to define the signature of the main entry point to the program.

## Main Entry Point
- Writing a program for evaluating stocks? Maybe that's a `func(i32) -> f64` that takes a symbol as a number and returns
a relative value for how the stock is likely to perform tomorrow.
- Writing a game AI? Maybe that's a `func()` that using imported functions to detect the current state of the world and
other functions to act upon it.
- Writing an approximation for a complex mathematical equation? Maybe that's a `func(i32, i32) -> (i32, i32)` that takes
a memory index and length, processes it and puts the results in a new memory index and length.

## Imports
Depending upon what you want the program to do, it may need to observe values from the host, or tell the host to
initiate actions (or both). Use Imports to define the functions on the host that the program may access.

## Memory
Decide what total amount of memory (zero is okay) that you want the program to be able to access. The memory is
allocated in pages and must include room for any data that is pre-loaded. It is hard for genetic programs to learn how
to structure a large block of empty memory, so it is usually more useful to keep structured records on the host and
provide imports that allow access. I.E. `get_close(i32) -> f64`, `set_weight(i32, f64)`, etc. Alternatively, you can
provide pre-defined functions that perform these actions as well.

## Data
In some programs, you will want to pre-load a chunk of data for each run. It might represent the current state of a game
character, or a list of circuit board parts that need to be laid out, etc.

## Pre-defined Functions
If you know your program will need to perform specific actions with it's memory (say your memory is a long linear array
of Open-High-Low-Close-Volume value), you may want to provide accessor functions to the programs that can be selected
during genetic programming along with other instructions.

## Fitness
Fitness has three components in wasmgp:
- Sorted Order: you will need to provide a function that is able to order two individuals according to their relative
fitness.
- Score (optional): if fitness can be reduced to a single numerical value, it can be useful to some algorithms.
- Zero Fitness (optional): if a program didn't even try to solve a particular problem, it can be useful to notate that.