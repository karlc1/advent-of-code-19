# advent-of-code-19

2019's participation of advent of code: https://adventofcode.com/2019/  
This year I'm doing this to learn Rust from scratch, so low code quality ahead. 


## Automated boiler plate creation

### make create
The Makefile automates some of the initialization for each daily challange. Run `make create day={n}` from the root folder to to setup most boilerplate stuff for the puzzle of the given day n. For example `make create day=3` will create a template project for day 3. Paste your puzzle input to the file at `./day{n}/input/input.txt`, and youre ready implement the solutions in `./day{n}/src/main.rs`. This will initialize the cargo project with a main file with some rudimentary file reading and a project structure that is compatible with the next command:

### make run 
To run the solution for a specific day and part, run the command `make run day={n} part={n}`. If you run `make run day 2 part 1`, the first part of the second days puzzle with be executed, given that you have initialized it with `make create` and of course implemented some solution in the template that got created. 
