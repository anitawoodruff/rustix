use std::collections::HashSet;
use cube::{Cube};

struct Solver {
    checked: HashSet<Cube>,
}

impl Solver {
    pub fn new() -> Self {
        Solver {
            checked: HashSet::new(),
        }
    }

    /// Returns true if passed a solved cube, else recursively twists the cube till solved.
    pub fn solve(&mut self, mut cube: Cube) -> (bool, Cube) {
        if self.checked.contains(&cube) {
            return (false, cube);
        }
        if cube.is_solved() {
            return (true, cube);
        }
        self.checked.insert(cube);
        cube.twist();
        let mut result = self.solve(cube);
        if result.0 {
            return result;
        }
        cube.turn();
        result = self.solve(cube);
        if result.0 {
            return result;
        }
        cube.tip_back();
        result = self.solve(cube);
        if result.0 {
            return result;
        }
        cube.tip_right();
        result = self.solve(cube);
        if result.0 {
            return result;
        }
        panic!("Unsolvable!");
    }
}

#[cfg(test)]
mod test {

    use super::{Cube, Solver};

    #[test]
    fn test_solve_is_noop_on_already_solved_cube() {
        let mut cube = Cube::new();
        let mut solver = Solver::new();
        let result = solver.solve(cube);
        assert!(result.1.is_solved());
    }

    #[test]
    fn test_can_solve_twisted_cube() {
        let mut solver = Solver::new();
        let mut cube = Cube::new();
        cube.twist();
        cube.twist();
        let result = solver.solve(cube);
        assert!(result.1.is_solved());
    }

    #[test]
    fn test_can_solve_turned_and_twisted_cube() {
        let mut solver = Solver::new();
        let mut cube = Cube::new();
        cube.turn();
        cube.turn();
        cube.twist();
        cube.twist();
        let result = solver.solve(cube);
        assert!(result.1.is_solved());
    }

    #[test]
    fn test_solve_cube_after_various_operations() {
        let mut cube = Cube::new();
        cube.twist();
        cube.turn();

        cube.twist();
        cube.turn();

        cube.twist();
        cube.turn();

        cube.bottom_twist();

        assert!(!cube.is_solved());

        let mut solver = Solver::new();
        let result = solver.solve(cube);

        assert!(result.1.is_solved());
    }
}
