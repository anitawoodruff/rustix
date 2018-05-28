#[macro_use] extern crate text_io;

use std::io::{self, Write};

mod cube;
use cube::Cube;

fn print_cube(cube: &Cube) {
    println!("{}", cube);
}

fn print_solved_status(cube: &Cube) {
    println!("{}", if cube.is_solved() {" SOLVED ! "} else { " Not solved yet..."});
}

fn print_cube_and_solved_status(cube: &Cube) {
    print_cube(&cube);
    print_solved_status(&cube);
}

fn main() {
    let mut cube = Cube::new();
    print_cube_and_solved_status(&cube);

    loop {
        print!("Enter a letter (a/d/f/g/t/q): ");
        let _ = io::stdout().flush();

        let input: char = read!();

        println!("You entered: {}", input);

        match input {
            'q' => return,
            'a' => {
                cube.turn();
                print_cube_and_solved_status(&cube);
            }
            'd' => {
                cube.turn_back();
                print_cube_and_solved_status(&cube);
            }
            'g' => {
                cube.twist();
                print_cube_and_solved_status(&cube);
            }
            't' => {
                cube.twist_back();
                print_cube_and_solved_status(&cube);
            }
            'f' => {
                cube.front_twist();
                print_cube_and_solved_status(&cube);
            }
            _ => {}
        }
    }
}
