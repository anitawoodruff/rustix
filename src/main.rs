// This is for implementing to_string.
use std::fmt::{
    Display,
    Formatter,
    Result as FmtResult,
};

struct Cube {
    solved: bool,
    turned: bool,
    twists: i8
}

fn wrap_index(index: i8, max: i8) -> usize {
    ((max + index) % max) as usize
}

impl Display for Cube {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {

        let top = self.get_color(0);
        let front = self.get_color(3);
        let back_front = self.get_color(2);
        let back_bottom = self.get_color(1);

        write!(formatter, "
                ______3_____             ______1_____
               /  y  /  {}  /|           /|     |     |
              /_____/_____/ |          /g|  w  |  {}  |
             /  y  /  {}  /|b|         /| |_____|_____|
            /_____0_____/ | |        |g|/|     |     |
            |     |     |b|/|        | /g|  w  |  {}  |
            |  r  |  {}  | /b|        |/| |_____2_____|
            |_____|_____|/| |        |g|/  p  /  {}  /
            |     |     |b|/         | /_____/_____/
            |  r  |  {}  | /          |/  p  /  {}  /
            |_____1_____|/           |_____3_____/
",
top, back_front,
top, back_front,
front, back_bottom,
front, back_bottom)
    }
}

impl Cube {
    fn is_solved(&self) -> bool {
        return self.solved;
    }

    fn twist(&mut self) {
        self.twists += 1;
        self.twists = self.twists % 4;
        self.solved = self.twists == 0;
    }

    fn turn(&mut self) {
        self.turned = true;
    }

    fn twist_back(&mut self) {
        self.solved = !self.solved;
    }

    fn get_color(&self, face_index: i8) -> &'static str{
        const NUM_COLORS: usize = 4;

        // The order of this array determines the color order of the faces.
        const COLORS: [&str; NUM_COLORS as usize] = ["y", "p", "w", "r"];

        let color_index = wrap_index(
            if self.solved {face_index} else {face_index + self.twists},
            NUM_COLORS as i8);

        COLORS[color_index as usize]
    }
}

fn build_cube() -> Cube {
    Cube {
        solved: true,
        turned: false,
        twists: 0,
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
    print_cube(&cube);

    cube.twist();
    print_cube(&cube);

    cube.twist();
    print_cube_and_solved_status(&cube);

    cube.twist();
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
    fn test_cube_twisted_once_is_not_solved() {
        let mut cube = build_cube();
        cube.twist();
        assert!(!cube.is_solved());
    }

    #[test]
    fn test_cube_twisted_twice_is_not_solved() {
        let mut cube = build_cube();
        cube.twist();
        cube.twist();
        assert!(!cube.is_solved());
    }

    #[test]
    fn test_cube_twisted_four_times_is_solved() {
        let mut cube = build_cube();
        cube.twist();
        cube.twist();
        cube.twist();
        cube.twist();
        assert!(cube.is_solved());
    }

    #[test]
    fn test_turned_cube_is_still_solved() {
        let mut cube = build_cube();
        cube.turn();
        assert!(cube.is_solved());
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
                ______3_____             ______1_____
               /  y  /  y  /|           /|     |     |
              /_____/_____/ |          /g|  w  |  w  |
             /  y  /  y  /|b|         /| |_____|_____|
            /_____0_____/ | |        |g|/|     |     |
            |     |     |b|/|        | /g|  w  |  w  |
            |  r  |  r  | /b|        |/| |_____2_____|
            |_____|_____|/| |        |g|/  p  /  p  /
            |     |     |b|/         | /_____/_____/
            |  r  |  r  | /          |/  p  /  p  /
            |_____1_____|/           |_____3_____/
";
        assert!(
            cube.to_string() == expected,
            "Expected {} but got {}\n", expected, cube);
    }

    #[test]
    fn test_twisted_cube_to_string() {
        let mut cube = build_cube();

        cube.twist();

        let expected = "
                ______3_____             ______1_____
               /  y  /  p  /|           /|     |     |
              /_____/_____/ |          /g|  w  |  r  |
             /  y  /  p  /|b|         /| |_____|_____|
            /_____0_____/ | |        |g|/|     |     |
            |     |     |b|/|        | /g|  w  |  r  |
            |  r  |  y  | /b|        |/| |_____2_____|
            |_____|_____|/| |        |g|/  p  /  w  /
            |     |     |b|/         | /_____/_____/
            |  r  |  y  | /          |/  p  /  w  /
            |_____1_____|/           |_____3_____/
";
        assert!(
            cube.to_string() == expected,
            "Expected {} but got {}", expected, cube);
    }

    #[test]
    fn test_twice_twisted_cube_to_string() {
        let mut cube = build_cube();

        cube.twist();
        cube.twist();

        let expected = "
                ______3_____             ______1_____
               /  y  /  w  /|           /|     |     |
              /_____/_____/ |          /g|  w  |  y  |
             /  y  /  w  /|b|         /| |_____|_____|
            /_____0_____/ | |        |g|/|     |     |
            |     |     |b|/|        | /g|  w  |  y  |
            |  r  |  p  | /b|        |/| |_____2_____|
            |_____|_____|/| |        |g|/  p  /  r  /
            |     |     |b|/         | /_____/_____/
            |  r  |  p  | /          |/  p  /  r  /
            |_____1_____|/           |_____3_____/
";
        assert!(
            cube.to_string() == expected,
            "Expected {} but got {}", expected, cube);
    }
}
