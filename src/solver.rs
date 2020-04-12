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

    /// Returns true if passed a solved cube, else twists the cube
    /// till solved.
    pub fn solve(&mut self, cube: Cube) -> Cube {
        // this isn't really a stack but sshhh!
        let mut stack = VecDeque::new();
        let mut operations = VecDeque::new();
        stack.push_back(cube);
        while let Some(cube) = stack.pop_front() {
            let op = operations.pop_front();
            println!("{:?}", op);
            if cube.is_solved() {
                println!("Solved with operations: {:?}", op);
                return cube;
            }
            self.checked.insert(cube);

            let mut twisty_cube = cube;
            twisty_cube.twist();
            if !self.checked.contains(&twisty_cube) {
                stack.push_back(twisty_cube);
                stack.push_back(twisty_cube);
                let op_string = "bottom_twist,";
                if (op.is_some()) {
                    op_string = "{:?} {}", op, op_string;
                    operations.push_back(op_string);
                } else {
                    operations.push_back(op_string);
                }
            }

            twisty_cube = cube;
            twisty_cube.front_twist();
            if !self.checked.contains(&twisty_cube) {
                stack.push_back(twisty_cube);
                let op_string = "front_twist,";
                if (op.is_some()) {
                    op_string = print!("{:?} {}", op, op_string);
                    operations.push_back(op_string);
                } else {
                    operations.push_back(op_string);
                }
            }

            twisty_cube = cube;
            twisty_cube.bottom_twist();
            if !self.checked.contains(&twisty_cube) {
                stack.push_back(twisty_cube);
                let op_string = "bottom_twist,";
                if (op.is_some()) {
                    op_string = print!("{:?} {}", op, op_string);
                    operations.push_back(op_string);
                } else {
                    operations.push_back(op_string);
                }
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
        let result = solver.solve(cube);

        assert!(result.is_solved());
    }

    #[test]
    fn test_can_solve_twisted_cube() {
        let mut solver = Solver::new();
        let mut cube = Cube::new();
        cube.twist();
        cube.twist();
        let result = solver.solve(cube);

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
        let result = solver.solve(cube);

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
        let result = solver.solve(cube);

        assert!(result.is_solved());
    }
}
