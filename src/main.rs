#[macro_use]
extern crate text_io;

use std::io::{self, Write};

mod cube;
mod solver;

use cube::Cube;
use solver::Solver;

fn print_cube(cube: &Cube) {
    println!("{}", cube);
}

fn print_solved_status(cube: &mut Cube) {
    println!(
        "{}",
        if cube.is_solved() {
            " SOLVED ! "
        } else {
            " Not solved yet..."
        }
    );
}

fn print_cube_and_solved_status(cube: &mut Cube) {
    print_cube(cube);
    print_solved_status(cube);
}

fn solve_cube(cube: Cube) -> Cube {
    let mut solver = Solver::new();
    solver.solve(cube)
}

fn main() {
    let mut cube = Cube::new();
    print_cube_and_solved_status(&mut cube);

    loop {
        print!("Enter a letter (w/a/s/d/f/r/g/t/x/c/S/h/q): ");
        let _ = io::stdout().flush();

        let input: char = read!();

        println!("You entered: {}", input);

        match input {
            'q' => return,
            'w' => cube.tip_back(),
            'a' => cube.turn(),
            's' => cube.tip_forwards(),
            'd' => cube.turn_back(),
            'g' => cube.twist(),
            't' => cube.twist_back(),
            'f' => cube.front_twist(),
            'r' => cube.undo_front_twist(),
            'x' => cube.bottom_twist_back(),
            'c' => cube.bottom_twist(),
            'S' => cube = solve_cube(cube),
            'e' => cube.tip_right(),
            'z' => cube.tip_left(),
            'h' => {
                println!("q - quits");
                println!("g - twists RHS of cube towards screen");
                println!("t - twists RHS of cube away from screen");
                println!("f - twists front face of cube clockwise");
                println!("r - twists front face of cube anti-clockwise");
                println!("S - Solves the cube!!!");
            }
            _ => {}
        }
        print_cube_and_solved_status(&mut cube);
    }
}
