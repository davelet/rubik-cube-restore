use super::super::{Solver, SolverEnum};
use super::utils::*;
use rubik_cube_core::cube::{color::Color, face::FaceOrientation, Cube};

pub struct TopCornerSolver {}

impl Solver for TopCornerSolver {
    fn target(&self) -> super::super::SolveTarget {
        super::super::SolveTarget::TopCorner
    }

    fn solve_target(&mut self, cube: &mut Cube) -> Vec<char> {
        let mut steps = vec![];

        while !self.is_target_solved(cube) {
            // Check if any corners are correctly positioned
            let mut correct_corners = 0;
            for f in 2..=5 {
                let face = FaceOrientation::from_u8(f as u8);
                if Self::is_corner_correct(cube, face) {
                    correct_corners += 1;
                }
            }

            match correct_corners {
                0 => Self::solve_no_correct(cube, &mut steps),
                1 => Self::solve_one_correct(cube, &mut steps),
                2 => Self::solve_two_correct(cube, &mut steps),
                _ => break,
            }
        }

        steps
    }

    fn is_target_solved(&self, cube: &Cube) -> bool {
        for f in 2..=5 {
            let face = FaceOrientation::from_u8(f as u8);
            if !Self::is_corner_correct(cube, face) {
                return false;
            }
        }
        true
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        None
    }
}

impl TopCornerSolver {
    fn is_corner_correct(cube: &Cube, face: FaceOrientation) -> bool {
        let face_color = face.color();
        cube.get_block_color(face.ordinal(), 0, 0) == face_color
            && cube.get_block_color(face.ordinal(), 0, 2) == face_color
    }

    fn solve_no_correct(cube: &mut Cube, steps: &mut Vec<char>) {
        Self::apply_algorithm(cube, steps);
    }

    fn solve_one_correct(cube: &mut Cube, steps: &mut Vec<char>) {
        // Rotate until correct corner is at back-right
        while !Self::is_corner_correct(cube, FaceOrientation::Back(Color::Green)) {
            rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, steps);
        }
        Self::apply_algorithm(cube, steps);
    }

    fn solve_two_correct(cube: &mut Cube, steps: &mut Vec<char>) {
        // Rotate until correct corners are at back
        while !Self::is_corner_correct(cube, FaceOrientation::Back(Color::Green)) {
            rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, steps);
        }
        Self::apply_algorithm(cube, steps);
    }

    fn apply_algorithm(cube: &mut Cube, steps: &mut Vec<char>) {
        let front_face = FaceOrientation::Front(Color::Blue);
        let right_face = FaceOrientation::Right(Color::Red);
        let up_face = FaceOrientation::Up(Color::Yellow);

        rotate_and_record(cube, right_face, true, steps);
        rotate_and_record(cube, up_face, false, steps);
        rotate_and_record(cube, right_face, false, steps);
        rotate_and_record(cube, up_face, false, steps);
        rotate_and_record(cube, right_face, true, steps);
        rotate_and_record(cube, up_face, true, steps);
        rotate_and_record(cube, right_face, false, steps);
    }
}
