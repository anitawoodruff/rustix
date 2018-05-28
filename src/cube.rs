use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy)]
pub struct Cube {
    turned: bool,
    twists: usize,
}

pub struct Block(Color, Color, Color);

#[derive(Clone, Copy)]
enum Color {
    Y, P, B, R, G, W
}

impl From<Color> for char {
    fn from(color : Color) -> char {
        match color {
            Y => 'y',
            P => 'p',
            B => 'b',
            R => 'r',
            G => 'g',
            W => 'w',
        }
    }
}

impl Display for Color {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        write!(formatter, "{}", char::from(*self))
    }
}

use self::Color::*;

const BLOCKS: [Block; 8] = [
    Block(Y, P, G), // 0
    Block(Y, B, P), // 1
    Block(R, Y, G), // 2
    Block(R, B, Y), // 3
    Block(G, P, W), // 4
    Block(P, B, W), // 5
    Block(R, G, W), // 6
    Block(W, B, R), // 7
];

impl Cube {
    pub fn new() -> Self {
        Cube {
            //config: [0, 1, 2, 3, 4, 5, 6, 7],
            turned: false,
            twists: 0,
        }
    }

    pub fn is_solved(&self) -> bool {
        return self.twists == 0;
    }

    /// Rotates the right-hand side of the cube towards the viewer.
    pub fn twist(&mut self) {
        self.twist_by(1);
    }

    pub fn turn(&mut self) {
        self.turned = true;
    }

    /// Rotates the right-hand side of the cube away from the viewer.
    pub fn twist_back(&mut self) {
        self.twist_by(-1);
    }

    pub fn turn_back(&mut self) {
        self.turned = false;
    }

    /// Rotates the right-hand side of the cube towards the viewer `amount` times.
    fn twist_by(&mut self, amount: i8) {
        self.twists = wrap_index(self.twists as i8 + amount, NUM_COLORS as i8);
    }

    fn get_color(&self, face_index: i8) -> &'static str {
        // The order of this determines order of the faces.
        const COLORS: [&str; NUM_COLORS as usize] = ["y", "p", "w", "r"];

        let color_index = wrap_index(face_index + self.twists as i8, NUM_COLORS as i8);

        COLORS[color_index as usize]
    }
}

impl Display for Cube {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        let top = self.get_color(0);
        let front_left = if self.turned { "b" } else { "r" };
        let front_right =
            if self.turned { "b" } else { self.get_color(3) };
        let rhs = if self.turned { "p" } else { "b" };

        write!(
            formatter,
            "
                ____________ 
               /  y  /  {}  /|
              /_____/_____/ |
             /  y  /  {}  /|{}|
            /_____/_____/ | |
            |     |     |{}|/|
            |  {}  |  {}  | /{}|
            |_____|_____|/| |
            |     |     |{}|/
            |  {}  |  {}  | /
            |_____|_____|/
",
            top,
            top,
            rhs,
            rhs,
            front_left,
            front_right,
            rhs,
            rhs,
            front_left,
            front_right
        )
    }
}

fn wrap_index(index: i8, n: i8) -> usize {
    ((n + index) % n) as usize
}

const NUM_COLORS: usize = 4;

#[cfg(test)]
mod test {

    use super::Cube;

    #[test]
    fn test_new_cube_is_solved() {
        let cube = Cube::new();
        assert!(cube.is_solved());
    }

    #[test]
    fn test_cube_twisted_once_is_not_solved() {
        let mut cube = Cube::new();
        cube.twist();
        assert!(!cube.is_solved());
    }

    #[test]
    fn test_cube_twisted_twice_is_not_solved() {
        let mut cube = Cube::new();
        cube.twist();
        cube.twist();
        assert!(!cube.is_solved());
    }

    #[test]
    fn test_cube_twisted_four_times_is_solved() {
        let mut cube = Cube::new();
        cube.twist();
        cube.twist();
        cube.twist();
        cube.twist();
        assert!(cube.is_solved());
    }

    #[test]
    fn test_twist_and_twist_back_is_solved() {
        let mut cube = Cube::new();
        cube.twist();
        assert!(!cube.is_solved());
        cube.twist_back();
        assert!(cube.is_solved());
    }

    #[test]
    fn test_turned_cube_is_still_solved() {
        let mut cube = Cube::new();
        cube.turn();
        assert!(cube.is_solved());
    }

    #[test]
    fn test_turn_and_turn_back_same_as_original() {
        let mut cube = Cube::new();
        cube.turn();
        cube.turn_back();
        assert_cube_strings_eq(&Cube::new().to_string(), &cube.to_string());
    }

    #[test]
    fn test_cube_to_string() {
        let cube = Cube::new();
        let expected = "
                ____________ 
               /  y  /  y  /|
              /_____/_____/ |
             /  y  /  y  /|b|
            /_____/_____/ | |
            |     |     |b|/|
            |  r  |  r  | /b|
            |_____|_____|/| |
            |     |     |b|/
            |  r  |  r  | /
            |_____|_____|/
";
        assert_cube_strings_eq(expected, &cube.to_string());
    }

    #[test]
    fn test_twisted_cube_to_string() {
        let mut cube = Cube::new();

        cube.twist();

        let expected = "
                ____________ 
               /  y  /  p  /|
              /_____/_____/ |
             /  y  /  p  /|b|
            /_____/_____/ | |
            |     |     |b|/|
            |  r  |  y  | /b|
            |_____|_____|/| |
            |     |     |b|/
            |  r  |  y  | /
            |_____|_____|/
";
        assert_cube_strings_eq(expected, &cube.to_string());
    }

    #[test]
    fn test_twice_twisted_cube_to_string() {
        let mut cube = Cube::new();

        cube.twist();
        cube.twist();

        let expected = "
                ____________ 
               /  y  /  w  /|
              /_____/_____/ |
             /  y  /  w  /|b|
            /_____/_____/ | |
            |     |     |b|/|
            |  r  |  p  | /b|
            |_____|_____|/| |
            |     |     |b|/
            |  r  |  p  | /
            |_____|_____|/
";
        assert_cube_strings_eq(expected, &cube.to_string());
    }

    #[test]
    fn test_turned_cube_to_string() {
        let mut cube = Cube::new();

        cube.turn();

        let expected = "
                ____________ 
               /  y  /  y  /|
              /_____/_____/ |
             /  y  /  y  /|p|
            /_____/_____/ | |
            |     |     |p|/|
            |  b  |  b  | /p|
            |_____|_____|/| |
            |     |     |p|/
            |  b  |  b  | /
            |_____|_____|/
";
        assert_cube_strings_eq(expected, &cube.to_string());
    }

    #[test]
    fn test_twisted_and_turned_cube_to_string() {
        let mut cube = Cube::new();

        cube.twist();
        cube.turn();

        let expected = "
                ____________ 
               /  y  /  y  /|
              /_____/_____/ |
             /  p  /  p  /|g|
            /_____/_____/ | |
            |     |     |g|/|
            |  b  |  b  | /g|
            |_____|_____|/| |
            |     |     |g|/
            |  b  |  b  | /
            |_____|_____|/
";
        assert_cube_strings_eq(expected, &cube.to_string());
    }

    #[test]
    fn test_thrice_twisted_to_string_same_as_twist_back() {
        let orig_cube = Cube::new();
        let mut cube_a = orig_cube;
        let mut cube_b = orig_cube;

        cube_a.twist();
        cube_a.twist();
        cube_a.twist();

        cube_b.twist_back();

        assert_cube_strings_eq(&cube_a.to_string(), &cube_b.to_string());
    }

    fn assert_cube_strings_eq(expected: &str, actual: &str) {
        assert!(
            actual == expected,
            "Expected {} but got {}\n",
            expected,
            actual
        );
    }
}
