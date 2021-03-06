use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Cube {
    blocks: [Block; 8],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub struct Block {
    top: Color,
    bottom: Color,
    lhs: Color,
    rhs: Color,
    front: Color,
    back: Color,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Color {
    Y,
    P,
    B,
    R,
    G,
    W,
    X,
}

impl From<Color> for char {
    fn from(color: Color) -> char {
        match color {
            Y => 'y',
            P => 'p',
            B => 'b',
            R => 'r',
            G => 'g',
            W => 'w',
            X => panic!("Tried to print the insides of the cube :S"),
        }
    }
}

impl Display for Color {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        write!(formatter, "{}", char::from(*self))
    }
}

use self::Color::*;

pub const BLOCKS: [Block; 8] = [Block {
    top: Y,
    back: P,
    lhs: G,
    rhs: B,
    front: R,
    bottom: W,
}; 8];

impl Block {
    pub fn tip_back(&mut self) -> Block {
        Block {
            bottom: self.back,
            front: self.bottom,
            top: self.front,
            back: self.top,
            lhs: self.lhs,
            rhs: self.rhs,
        }
    }
    pub fn tip_forwards(&mut self) -> Block {
        return self.tip_back().tip_back().tip_back();
    }
    // rotates the cube to the right so that the lhs is now facing front.
    pub fn turn_right(&mut self) -> Block {
        Block {
            bottom: self.bottom,
            front: self.lhs,
            top: self.top,
            back: self.rhs,
            lhs: self.back,
            rhs: self.front,
        }
    }

    pub fn turn_left(&mut self) -> Block {
        return self.turn_right().turn_right().turn_right();
    }
}

impl Cube {
    pub fn new() -> Self {
        Cube { blocks: BLOCKS }
    }

    pub fn is_solved(&self) -> bool {
        let mut test = *self;
        for _ in 0..4 {
            for _ in 0..4 {
                if test.blocks == BLOCKS {
                    return true;
                }
                test.turn();
            }
            test.tip_back();
        }
        // ugly hack because otherwise some positions
        // get left out; this could be more efficient:
        for _ in 0..4 {
            for _ in 0..4 {
                if test.blocks == BLOCKS {
                    return true;
                }
                test.tip_back();
            }
            test.turn();
        }
        return false;
    }

    /// Rotates the right-hand side of the cube towards the viewer.
    pub fn twist(&mut self) {
        fn pretwist_posn(posn: usize) -> usize {
            match posn {
                1 => 5,
                3 => 1,
                5 => 7,
                7 => 3,
                _ => posn,
            }
        }
        let mut next_blocks = self.blocks;

        for &posn in &[1, 3, 5, 7] {
            next_blocks[posn] = self.blocks[pretwist_posn(posn)].tip_forwards();
        }

        self.blocks = next_blocks;
    }

    pub fn bottom_twist_back(&mut self) {
        for _ in 0..3 {
            self.bottom_twist();
        }
    }

    /// Rotates the bottom of the cube to the right.
    pub fn bottom_twist(&mut self) {
        fn next_posn(posn: usize) -> usize {
            match posn {
                4 => 6,
                5 => 4,
                6 => 7,
                7 => 5,
                _ => posn,
            }
        }
        let mut next_blocks = self.blocks;
        for posn in 4..8 {
            next_blocks[next_posn(posn)] = self.blocks[posn].turn_right();
        }
        self.blocks = next_blocks;
    }

    /// Rotates the front of the cube counter clockwise.
    pub fn undo_front_twist(&mut self) {
        self.front_twist();
        self.front_twist();
        self.front_twist();
    }

    /// Rotates the front of the cube clockwise.
    pub fn front_twist(&mut self) {
        self.turn_back();
        self.twist_back();
        self.turn();
    }

    /// Tips the cube away from the viewer.
    pub fn tip_back(&mut self) {
        self.twist_back();
        self.turn();
        self.turn();
        self.twist();
        self.turn();
        self.turn();
    }

    /// Tips the cube towars the viewer.
    pub fn tip_forwards(&mut self) {
        self.tip_back();
        self.tip_back();
        self.tip_back();
    }

    /// Turns the cube to the left so the rhs now faces to the front.
    pub fn turn(&mut self) {
        fn preturn_posn(posn: usize) -> usize {
            match posn {
                0 => 2,
                1 => 0,
                2 => 3,
                3 => 1,
                4 => 6,
                5 => 4,
                6 => 7,
                7 => 5,
                _ => posn,
            }
        }
        let mut next_blocks = self.blocks;

        for posn in 0..8 {
            next_blocks[posn] = self.blocks[preturn_posn(posn)].turn_left();
        }

        self.blocks = next_blocks;
    }

    /// Tips the cube to the right so the top now faces to the rhs.
    pub fn tip_right(&mut self) {
        self.turn();
        self.tip_forwards();
        self.turn_back();
    }

    /// Tips the cube to the left so the top now faces to the lhs.
    pub fn tip_left(&mut self) {
        self.tip_right();
        self.tip_right();
        self.tip_right();
    }

    /// Rotates the right-hand side of the cube away from the viewer.
    pub fn twist_back(&mut self) {
        self.twist_by(3);
    }

    pub fn turn_back(&mut self) {
        self.turn();
        self.turn();
        self.turn();
    }

    /// Rotates the right-hand side of the cube towards the viewer `amount` times.
    fn twist_by(&mut self, amount: i8) {
        for _ in 0..amount {
            self.twist();
        }
    }

    fn color_at(&self, face_index: i8) -> Color {
        match face_index {
            0 => self.blocks[0].top,
            1 => self.blocks[1].top,
            2 => self.blocks[2].top,
            3 => self.blocks[3].top,
            4 => self.blocks[1].rhs,
            5 => self.blocks[3].rhs,
            6 => self.blocks[2].front,
            7 => self.blocks[3].front,
            8 => self.blocks[5].rhs,
            9 => self.blocks[7].rhs,
            10 => self.blocks[6].front,
            11 => self.blocks[7].front,
            _ => W,
        }
    }
}

impl Display for Cube {
    fn fmt(&self, formatter: &mut Formatter) -> FmtResult {
        write!(
            formatter,
            "
                ____________
               /  {0}  /  {1}  /|
              /_____/_____/ |
             /  {2}  /  {3}  /|{4}|
            /_____/_____/ | |
            |     |     |{5}|/|
            |  {6}  |  {7}  | /{8}|
            |_____|_____|/| |
            |     |     |{9}|/
            |  {10}  |  {11}  | /
            |_____|_____|/
",
            self.color_at(0),
            self.color_at(1),
            self.color_at(2),
            self.color_at(3),
            self.color_at(4),
            self.color_at(5),
            self.color_at(6),
            self.color_at(7),
            self.color_at(8),
            self.color_at(9),
            self.color_at(10),
            self.color_at(11),
        )
    }
}

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
        assert_eq!(cube, Cube::new());
        assert!(cube.is_solved());
    }

    #[test]
    fn test_twist_and_twist_back_is_solved() {
        let mut cube = Cube::new();
        cube.twist();
        assert!(!cube.is_solved());
        cube.twist_back();
        assert!(cube.is_solved(), "{:#?}", cube);
    }

    #[test]
    fn test_twist_front_four_times_is_solved() {
        let mut cube = Cube::new();
        cube.front_twist();
        cube.front_twist();
        cube.front_twist();
        cube.front_twist();
        assert!(cube.is_solved(), "{:#?}", cube);
    }

    #[test]
    fn test_turned_cube_is_still_solved() {
        let mut cube = Cube::new();
        cube.turn();
        assert!(cube.is_solved());
    }

    #[test]
    fn test_turn_and_tip_back_is_still_solved() {
        let mut cube = Cube::new();
        cube.turn();
        cube.tip_back();
        assert!(cube.is_solved());
    }

    #[test]
    fn test_tip_back_and_turn_is_still_solved() {
        let mut cube = Cube::new();
        cube.tip_back();
        cube.turn();
        assert!(cube.is_solved());
    }

    #[test]
    fn test_turn_back_is_still_solved() {
        let mut cube = Cube::new();
        cube.turn_back();
        assert!(cube.is_solved());
    }

    #[test]
    fn test_tip_forwards_is_still_solved() {
        let mut cube = Cube::new();
        cube.tip_forwards();
        assert!(cube.is_solved());
    }

    #[test]
    fn test_tip_back_is_still_solved() {
        let mut cube = Cube::new();
        cube.tip_back();
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
    fn test_bottom_twist_to_string() {
        let mut cube = Cube::new();
        cube.bottom_twist();
        let expected = "
                ____________
               /  y  /  y  /|
              /_____/_____/ |
             /  y  /  y  /|b|
            /_____/_____/ | |
            |     |     |b|/|
            |  r  |  r  | /r|
            |_____|_____|/| |
            |     |     |r|/
            |  g  |  g  | /
            |_____|_____|/
";
        assert_cube_strings_eq(expected, &cube.to_string());
    }

    #[test]
    fn test_tip_back_to_string() {
        let mut cube = Cube::new();
        cube.tip_back();
        let expected = "
                ____________
               /  r  /  r  /|
              /_____/_____/ |
             /  r  /  r  /|b|
            /_____/_____/ | |
            |     |     |b|/|
            |  w  |  w  | /b|
            |_____|_____|/| |
            |     |     |b|/
            |  w  |  w  | /
            |_____|_____|/
";
        assert_cube_strings_eq(expected, &cube.to_string());
    }

    #[test]
    fn test_front_twist_cube_to_string() {
        let mut cube = Cube::new();
        cube.front_twist();
        let expected = "
                ____________
               /  y  /  y  /|
              /_____/_____/ |
             /  g  /  g  /|b|
            /_____/_____/ | |
            |     |     |y|/|
            |  r  |  r  | /b|
            |_____|_____|/| |
            |     |     |y|/
            |  r  |  r  | /
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
        assert!(!cube.is_solved(), "{:#?}", cube);
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
    fn test_twist_then_turn_to_string() {
        let mut cube = Cube::new();

        cube.twist();
        cube.turn();

        let expected = "
                ____________
               /  y  /  y  /|
              /_____/_____/ |
             /  p  /  p  /|p|
            /_____/_____/ | |
            |     |     |w|/|
            |  b  |  b  | /p|
            |_____|_____|/| |
            |     |     |w|/
            |  b  |  b  | /
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
    fn test_tip_right_to_string() {
        let mut cube = Cube::new();

        cube.tip_right();

        let expected = "
                ____________
               /  g  /  g  /|
              /_____/_____/ |
             /  g  /  g  /|y|
            /_____/_____/ | |
            |     |     |y|/|
            |  r  |  r  | /y|
            |_____|_____|/| |
            |     |     |y|/
            |  r  |  r  | /
            |_____|_____|/
";
        assert_cube_strings_eq(expected, &cube.to_string());
    }
    #[test]
    fn test_turn_then_twist_to_string() {
        let mut cube = Cube::new();

        cube.turn();
        cube.twist();

        let expected = "
                ____________
               /  y  /  g  /|
              /_____/_____/ |
             /  y  /  g  /|p|
            /_____/_____/ | |
            |     |     |p|/|
            |  b  |  y  | /p|
            |_____|_____|/| |
            |     |     |p|/
            |  b  |  y  | /
            |_____|_____|/
";
        assert_eq!(expected, &cube.to_string(), "{:#?}", cube);
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
