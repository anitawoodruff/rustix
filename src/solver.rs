use std::collections::{HashSet, VecDeque};
use cube::Cube;

pub struct Solver {
    checked: HashSet<Cube>,
}

impl Solver {
    pub fn new() -> Self {
        Solver {
            checked: HashSet::new(),
        }
    }

    /// Returns true if passed a solved cube, else recursively twists the cube till solved.
    pub fn solve(&mut self, cube: Cube, depth: usize) -> Cube {
        // this isn't really a stack but sshhh!
        let mut stack = VecDeque::new();
        stack.push_back(cube);
        while let Some(cube) = stack.pop_front() {
            println!("{}", stack.len());
            if cube.is_solved() {
                return cube;
            }
            self.checked.insert(cube);

            let mut twisty_cube = cube;
            twisty_cube.twist();
            if !self.checked.contains(&twisty_cube) {
                stack.push_back(twisty_cube);
            }

            twisty_cube = cube;
            twisty_cube.front_twist();
            if !self.checked.contains(&twisty_cube) {
                stack.push_back(twisty_cube);
            }

            twisty_cube = cube;
            twisty_cube.bottom_twist();
            if !self.checked.contains(&twisty_cube) {
                stack.push_back(twisty_cube);
            }
        }
        panic!("Unsolvamable cube oh nose");
    }
}

#[cfg(test)]
mod test {

    use super::{Cube, Solver};

    #[test]
    fn test_solve_is_noop_on_already_solved_cube() {
        let mut cube = Cube::new();
        let mut solver = Solver::new();
        let result = solver.solve(cube, 0);

        assert!(result.is_solved());
    }

    #[test]
    fn test_can_solve_twisted_cube() {
        let mut solver = Solver::new();
        let mut cube = Cube::new();
        cube.twist();
        cube.twist();
        let result = solver.solve(cube, 0);

        assert!(result.is_solved());
    }

    #[test]
    fn test_can_solve_turned_and_twisted_cube() {
        let mut solver = Solver::new();
        let mut cube = Cube::new();
        cube.turn();
        cube.turn();
        cube.twist();
        cube.twist();
        let result = solver.solve(cube, 0);

        assert!(result.is_solved());
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
        let result = solver.solve(cube, 0);

        assert!(result.is_solved());
    }
}
