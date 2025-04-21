use crate::utils::print_cube;

use super::prelude::*;

pub struct TopEdgeSolver {}

impl Solver for TopEdgeSolver {
    fn target(&self) -> super::super::SolveTarget {
        super::super::SolveTarget::TopEdge
    }

    fn solve_target(&mut self, cube: &mut Cube) -> Vec<char> {
        let mut steps = vec![];
        let mut count = 0;

        while !self.is_target_solved(cube) {
            if count > 3 {
                print_cube(cube);
                panic!("Exceeded maximum iterations");
            }
            count += 1;

            let face = self.has_aligned_edge(cube);
            self.execute_edge_permutation_algorithm(cube, face, &mut steps);
        }

        steps
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        None
    }

    fn is_target_solved(&self, cube: &Cube) -> bool {
        cube.is_solved()
    }
}

impl TopEdgeSolver {
    fn has_aligned_edge(&self, cube: &Cube) -> FaceOrientation {
        // Check if any edge is already aligned
        for face in [
            FaceOrientation::Front(Color::Blue),
            FaceOrientation::Right(Color::Red),
            FaceOrientation::Back(Color::Orange),
            FaceOrientation::Left(Color::Green),
        ] {
            if cube.get_block_color(face.ordinal(), 0, 1) == face.color() {
                return face;
            }
        }
        FaceOrientation::Back(Color::Orange)
    }

    fn execute_edge_permutation_algorithm(
        &self,
        cube: &mut Cube,
        face: FaceOrientation,
        steps: &mut Vec<char>,
    ) {
        // Calculate pre-rotation count based on face
        let rotations = match face {
            FaceOrientation::Front(Color::Blue) => 2,
            FaceOrientation::Right(Color::Red) => 3,
            FaceOrientation::Left(Color::Green) => 1,
            _ => 0,
        };

        // Pre-rotate to move aligned face to back
        for _ in 0..rotations {
            rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, steps);
        }

        // Execute edge permutation algorithm: R U' R U R U R U' R' U' R' R'
        rotate_and_record(cube, FaceOrientation::Right(Color::Red), true, steps);
        rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), false, steps);
        rotate_and_record(cube, FaceOrientation::Right(Color::Red), true, steps);
        rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, steps);
        rotate_and_record(cube, FaceOrientation::Right(Color::Red), true, steps);
        rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, steps);
        rotate_and_record(cube, FaceOrientation::Right(Color::Red), true, steps);
        rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), false, steps);

        rotate_and_record(cube, FaceOrientation::Right(Color::Red), false, steps);
        rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), false, steps);
        rotate_and_record(cube, FaceOrientation::Right(Color::Red), false, steps);
        rotate_and_record(cube, FaceOrientation::Right(Color::Red), false, steps);

        // Rotate back to original orientation
        for _ in 0..(4 - rotations) % 4 {
            rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, steps);
        }
    }
}
