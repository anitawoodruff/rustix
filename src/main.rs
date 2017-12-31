use std::collections::HashMap;

// This is for implementing to_string.
use std::fmt::{
    Display,
    Formatter,
    Result as FmtResult,
};

struct Cube {
    solved : bool,
}

fn wrap_index(index: i8, max: i8) -> usize {
    ((max + index) % max) as usize
}

impl Display for Cube {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        fn get_color(solved: bool, face_index: i8) -> &'static str{
            const NUM_COLORS: usize = 4;
            const COLORS: [&str; NUM_COLORS as usize] = ["y", "r", "w", "p"];

            let color_index = wrap_index(
                if solved {face_index} else {face_index - 1},
                NUM_COLORS as i8);

            COLORS[color_index as usize]
        }

        let top = get_color(self.solved, 0);
        let front = get_color(self.solved, 1);
        let back_front = get_color(self.solved, 2);
        let back_bottom = get_color(self.solved, 3);
        write!(formatter, "
                ____________             ______1_____
               /  y  /  {}  /|           /|     |     |
              /_____/_____/ |          /g|  w  |  {}  |
             /  y  /  {}  /|b|         /| |_____|_____|2
            /_____/_____/ | |        |g|/|     |     |
            |     |     |b|/|        | /g|  w  |  {}  |
            |  r  |  {}  | /b|        |/| |_____|_____|
            |_____|_____|/| |        |g|/  p  /  {}  /
            |     |     |b|/         | /_____/_____/
            |  r  |  {}  | /2         |/  p  /  {}  /
            |_____|_____|/           |_____/_____/
                  1
", top, back_front, top, back_front, front, back_bottom, front, back_bottom)
    }
}

impl Cube {
    fn is_solved(&self) -> bool {
        return self.solved;
    }

    fn twist(&mut self) {
        self.solved = !self.solved;
    }

    fn twist_back(&mut self) {
        self.solved = !self.solved;
    }
}

fn build_cube() -> Cube {
    Cube {
        solved : true,
    }
}

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

    let mut cube = build_cube();
    print_cube_and_solved_status(&cube);

    cube.twist();
    print_cube_and_solved_status(&cube);

    cube.twist_back();
    print_cube_and_solved_status(&cube);
}

#[cfg(test)]
mod test {

    use super::build_cube;

    #[test]
    fn test_new_cube_is_solved() {
        let cube = build_cube();
        assert!(cube.is_solved());
    }

    #[test]
    fn test_twisted_cube_is_not_solved() {
        let mut cube = build_cube();
        cube.twist();
        assert!(!cube.is_solved());
    }

    #[test]
    fn test_twist_and_twist_back_is_solved() {
        let mut cube = build_cube();
        cube.twist();
        assert!(!cube.is_solved());
        cube.twist_back();
        assert!(cube.is_solved());
    }

    #[test]
    fn test_cube_to_string() {
        let cube = build_cube();
        let expected = "
                ____________             ______1_____
               /  y  /  y  /|           /|     |     |
              /_____/_____/ |          /g|  w  |  w  |
             /  y  /  y  /|b|         /| |_____|_____|2
            /_____/_____/ | |        |g|/|     |     |
            |     |     |b|/|        | /g|  w  |  w  |
            |  r  |  r  | /b|        |/| |_____|_____|
            |_____|_____|/| |        |g|/  p  /  p  /
            |     |     |b|/         | /_____/_____/
            |  r  |  r  | /2         |/  p  /  p  /
            |_____|_____|/           |_____/_____/
                  1
";
        assert!(
            cube.to_string() == expected,
            "Expected {} but got {}", expected, cube);
    }

    #[test]
    fn test_twisted_cube_to_string() {
        let mut cube = build_cube();

        cube.twist();

        let expected = "
                ____________             ______1_____
               /  y  /  p  /|           /|     |     |
              /_____/_____/ |          /g|  w  |  r  |
             /  y  /  p  /|b|         /| |_____|_____|2
            /_____/_____/ | |        |g|/|     |     |
            |     |     |b|/|        | /g|  w  |  r  |
            |  r  |  y  | /b|        |/| |_____|_____|
            |_____|_____|/| |        |g|/  p  /  w  /
            |     |     |b|/         | /_____/_____/
            |  r  |  y  | /2         |/  p  /  w  /
            |_____|_____|/           |_____/_____/
                  1
";
        assert!(
            cube.to_string() == expected,
            "Expected {} but got {}", expected, cube);
    }
}
