use super::super::{Solver, SolverEnum};
use super::top_corner::TopCornerSolver;
use super::utils::*;
use rubik_cube_core::cube::{color::Color, face::FaceOrientation, Cube};

pub struct TopEdgeSolver {}

impl Solver for TopEdgeSolver {
    fn target(&self) -> super::super::SolveTarget {
        super::super::SolveTarget::TopEdge
    }

    fn solve_target(&mut self, cube: &mut Cube) -> Vec<char> {
        let mut steps = vec![];

        while !self.is_target_solved(cube) {
            // Check if any edges are correctly positioned
            let mut correct_edges = 0;
            for f in 2..=5 {
                let face = FaceOrientation::from_u8(f as u8);
                if Self::is_edge_correct(cube, face) {
                    correct_edges += 1;
                }
            }

            match correct_edges {
                0 => Self::solve_no_correct(cube, &mut steps),
                1 => Self::solve_one_correct(cube, &mut steps),
                _ => break,
            }
        }

        steps
    }

    fn is_target_solved(&self, cube: &Cube) -> bool {
        for f in 2..=5 {
            let face = FaceOrientation::from_u8(f as u8);
            if !Self::is_edge_correct(cube, face) {
                return false;
            }
        }
        true
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        Some(SolverEnum::TopCorner(TopCornerSolver {}))
    }
}

impl TopEdgeSolver {
    fn is_edge_correct(cube: &Cube, face: FaceOrientation) -> bool {
        let face_color = face.color();
        cube.get_block_color(face.ordinal(), 0, 1) == face_color
    }

    fn solve_no_correct(cube: &mut Cube, steps: &mut Vec<char>) {
        Self::apply_algorithm(cube, steps);
    }

    fn solve_one_correct(cube: &mut Cube, steps: &mut Vec<char>) {
        // Rotate until correct edge is at back
        while !Self::is_edge_correct(cube, FaceOrientation::Back(Color::Green)) {
            rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, steps);
        }
        Self::apply_algorithm(cube, steps);
    }

    fn apply_algorithm(cube: &mut Cube, steps: &mut Vec<char>) {
        let front_face = FaceOrientation::Front(Color::Blue);
        let right_face = FaceOrientation::Right(Color::Red);
        let up_face = FaceOrientation::Up(Color::Yellow);

        rotate_and_record(cube, right_face, true, steps);
        rotate_and_record(cube, up_face, true, steps);
        rotate_and_record(cube, right_face, false, steps);
        rotate_and_record(cube, up_face, true, steps);
        rotate_and_record(cube, right_face, true, steps);
        rotate_and_record(cube, up_face, true, steps);
        rotate_and_record(cube, up_face, true, steps);
        rotate_and_record(cube, right_face, false, steps);
        rotate_and_record(cube, up_face, false, steps);
    }
}
