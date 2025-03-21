// use rubik_cube_restore::rubiks::solver::{Executor, SolveTarget, Solver, SolverEnum};
// use rubik_cube_restore_lib::rubiks::cube::Cube;

// struct MockSolver {
//     target: SolveTarget,
//     is_solved: bool,
//     solution_steps: Vec<char>,
//     next: Option<Box<dyn Solver>>,
// }

// impl Solver for MockSolver {
//     fn target(&self) -> SolveTarget {
//         self.target
//     }

//     fn solve_target(&mut self) -> Vec<char> {
//         self.solution_steps.clone()
//     }

//     fn is_target_solved(&self) -> bool {
//         self.is_solved
//     }

//     fn next_solver(&self) -> Option<SolverEnum> {
//         self.next.as_ref().map(|s| SolverEnum::Mock(s.clone()))
//     }
// }

// impl Clone for MockSolver {
//     fn clone(&self) -> Self {
//         MockSolver {
//             target: self.target,
//             is_solved: self.is_solved,
//             solution_steps: self.solution_steps.clone(),
//             next: self.next.clone(),
//         }
//     }
// }

// #[derive(Clone)]
// pub enum SolverEnum {
//     Mock(MockSolver),
//     BottomCross(BottomCrossSolver),
//     BottomCorner(BottomCornerSolver),
//     MiddleEdge(MiddleSolver),
//     TopCross(TopCrossSolver),
//     TopFace(TopFaceSolver),
//     TopEdge(TopEdgeSolver),
//     TopCorner(TopCornerSolver),
// }

// impl Solver for SolverEnum {
//     fn target(&self) -> SolveTarget {
//         match_solver_enum!(self, target)
//     }

//     fn solve_target(&mut self) -> Vec<char> {
//         match_solver_enum!(self, solve_target)
//     }

//     fn is_target_solved(&self) -> bool {
//         match_solver_enum!(self, is_target_solved)
//     }

//     fn next_solver(&self) -> Option<SolverEnum> {
//         match_solver_enum!(self, next_solver)
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_executor_with_mock_solvers() {
//         let cube = Cube::new(); // Assuming Cube::new() creates a new unsolved cube
//         let target = SolveTarget::TopCorner;

//         let top_corner_solver = MockSolver {
//             target: SolveTarget::TopCorner,
//             is_solved: false,
//             solution_steps: vec!['T'],
//             next: None,
//         };

//         let top_edge_solver = MockSolver {
//             target: SolveTarget::TopEdge,
//             is_solved: false,
//             solution_steps: vec!['E'],
//             next: Some(Box::new(top_corner_solver)),
//         };

//         let top_face_solver = MockSolver {
//             target: SolveTarget::TopFace,
//             is_solved: false,
//             solution_steps: vec!['F'],
//             next: Some(Box::new(top_edge_solver)),
//         };

//         let top_cross_solver = MockSolver {
//             target: SolveTarget::TopCross,
//             is_solved: false,
//             solution_steps: vec!['C'],
//             next: Some(Box::new(top_face_solver)),
//         };

//         let middle_solver = MockSolver {
//             target: SolveTarget::MiddleEdge,
//             is_solved: false,
//             solution_steps: vec!['M'],
//             next: Some(Box::new(top_cross_solver)),
//         };

//         let bottom_corner_solver = MockSolver {
//             target: SolveTarget::BottomCorner,
//             is_solved: false,
//             solution_steps: vec!['B'],
//             next: Some(Box::new(middle_solver)),
//         };

//         let bottom_cross_solver = MockSolver {
//             target: SolveTarget::BottomCross,
//             is_solved: false,
//             solution_steps: vec!['X'],
//             next: Some(Box::new(bottom_corner_solver)),
//         };

//         let executor = Executor::new(cube, target);
//         let solution = executor.execute();

//         assert_eq!(solution, vec!['X', 'B', 'M', 'C', 'F', 'E', 'T']);
//     }

//     #[test]
//     fn test_executor_with_solved_cube() {
//         let cube = Cube::new(); // Assuming Cube::new() creates a new solved cube
//         let target = SolveTarget::TopCorner;

//         let executor = Executor::new(cube, target);
//         let solution = executor.execute();

//         assert!(solution.is_empty());
//     }
// }
