use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy)]
pub struct Cube {
    turned: bool,
    twists: usize,
}

impl Cube {
    pub fn new() -> Self {
        Cube {
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
        let back_front = self.get_color(2);
        let back_bottom_right =
            if self.turned { "g" } else { self.get_color(1) };
        let back_bottom_left = if self.turned { "g" } else { "p" };
        let rhs = if self.turned { "p" } else { "b" };
        let lhs = if self.turned { "r" } else { "g" };

        write!(
            formatter,
            "
                ______3_____             ______1_____
               /  y  /  {}  /|           /|     |     |
              /_____/_____/ |          /{}|  w  |  {}  |
             /  y  /  {}  /|{}|         /| |_____|_____|
            /_____0_____/ | |        |{}|/|     |     |
            |     |     |{}|/|        | /{}|  w  |  {}  |
            |  {}  |  {}  | /{}|        |/| |_____2_____|
            |_____|_____|/| |        |{}|/  {}  /  {}  /
            |     |     |{}|/         | /_____/_____/
            |  {}  |  {}  | /          |/  {}  /  {}  /
            |_____1_____|/           |_____3_____/
",
            top,
            lhs,
            back_front,
            top,
            rhs,
            lhs,
            rhs,
            lhs,
            back_front,
            front_left,
            front_right,
            rhs,
            lhs,
            back_bottom_left,
            back_bottom_right,
            rhs,
            front_left,
            front_right,
            back_bottom_left,
            back_bottom_right
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
        assert_cube_strings_eq(expected, &cube.to_string());
    }

    #[test]
    fn test_twisted_cube_to_string() {
        let mut cube = Cube::new();

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
        assert_cube_strings_eq(expected, &cube.to_string());
    }

    #[test]
    fn test_turned_cube_to_string() {
        let mut cube = Cube::new();

        cube.turn();

        let expected = "
                ______3_____             ______1_____
               /  y  /  y  /|           /|     |     |
              /_____/_____/ |          /r|  w  |  w  |
             /  y  /  y  /|p|         /| |_____|_____|
            /_____0_____/ | |        |r|/|     |     |
            |     |     |p|/|        | /r|  w  |  w  |
            |  b  |  b  | /p|        |/| |_____2_____|
            |_____|_____|/| |        |r|/  g  /  g  /
            |     |     |p|/         | /_____/_____/
            |  b  |  b  | /          |/  g  /  g  /
            |_____1_____|/           |_____3_____/
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

    #[test]
    fn test_twice_twisted_cube_to_string() {
        let mut cube = Cube::new();

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
        assert_cube_strings_eq(expected, &cube.to_string());
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
