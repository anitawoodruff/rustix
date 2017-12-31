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
    println!("Hello, world!");

    let mut cube = Cube::new();
    print_cube_and_solved_status(&cube);

    cube.twist();
    print_cube(&cube);

    cube.twist();
    print_cube(&cube);

    cube.twist();
    print_cube_and_solved_status(&cube);

    cube.twist();
    print_cube_and_solved_status(&cube);
}
