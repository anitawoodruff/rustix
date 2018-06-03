#[macro_use]
extern crate text_io;

use std::io::{self, Write};

mod cube;
mod solver;

use cube::Cube;

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

fn main() {
    let mut cube = Cube::new();
    print_cube_and_solved_status(&mut cube);

    loop {
        print!("Enter a letter (w/a/s/d/f/r/g/t/x/c/q): ");
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
            _ => {}
        }
        print_cube_and_solved_status(&mut cube);
    }
}
