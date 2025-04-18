use rubik_cube_core::cube::{color::Color, face::FaceOrientation, Cube};

use super::super::{Solver, SolverEnum};
use super::top_face::TopFaceSolver;
use super::utils::*;
pub struct TopCrossSolver {}

impl Solver for TopCrossSolver {
    fn target(&self) -> super::super::SolveTarget {
        super::super::SolveTarget::TopCross
    }

    fn solve_target(&mut self, cube: &mut Cube) -> Vec<char> {
        if !self.is_previous_solved() {
            panic!("previous not solved");
        }

        let mut steps = vec![];

        while !self.is_target_solved(cube) {
            let top_face = FaceOrientation::Up(Color::Yellow);
            let yellow_count = Self::count_yellow_edges(cube);

            match yellow_count {
                0 => Self::solve_no_yellow(cube, &mut steps),
                1 => Self::solve_one_yellow(cube, &mut steps),
                2 => Self::solve_two_yellow(cube, &mut steps),
                _ => break,
            }
        }

        steps
    }

    fn is_target_solved(&self, cube: &Cube) -> bool {
        let face_colors = cube.get_face_state(FaceOrientation::Up(Color::Yellow).ordinal());
        let color = face_colors[1][1];

        face_colors[0][1] == color
            && face_colors[1][0] == color
            && face_colors[1][2] == color
            && face_colors[2][1] == color
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        Some(SolverEnum::TopFace(TopFaceSolver {}))
    }
}

impl TopCrossSolver {
    fn is_previous_solved(self: &Self) -> bool {
        false
    }

    fn count_yellow_edges(cube: &Cube) -> u8 {
        let face_colors = cube.get_face_state(FaceOrientation::Up(Color::Yellow).ordinal());
        let mut count = 0;

        if face_colors[0][1] == Color::Yellow {
            count += 1;
        }
        if face_colors[1][0] == Color::Yellow {
            count += 1;
        }
        if face_colors[1][2] == Color::Yellow {
            count += 1;
        }
        if face_colors[2][1] == Color::Yellow {
            count += 1;
        }

        count
    }

    fn solve_no_yellow(cube: &mut Cube, steps: &mut Vec<char>) {
        let front_face = FaceOrientation::Front(Color::Blue);
        rotate_and_record(cube, front_face, true, steps);
        rotate_and_record(cube, FaceOrientation::Right(Color::Red), true, steps);
        rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, steps);
        rotate_and_record(cube, FaceOrientation::Right(Color::Red), false, steps);
        rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), false, steps);
        rotate_and_record(cube, front_face, false, steps);
    }

    fn solve_one_yellow(cube: &mut Cube, steps: &mut Vec<char>) {
        // Rotate top face until yellow edge is at the back
        while cube.get_face_state(FaceOrientation::Up(Color::Yellow).ordinal())[0][1]
            != Color::Yellow
        {
            rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, steps);
        }

        // Apply algorithm
        let front_face = FaceOrientation::Front(Color::Blue);
        rotate_and_record(cube, front_face, true, steps);
        rotate_and_record(cube, FaceOrientation::Right(Color::Red), true, steps);
        rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, steps);
        rotate_and_record(cube, FaceOrientation::Right(Color::Red), false, steps);
        rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), false, steps);
        rotate_and_record(cube, front_face, false, steps);
    }

    fn solve_two_yellow(cube: &mut Cube, steps: &mut Vec<char>) {
        // Rotate top face until yellow edges form a line
        while cube.get_face_state(FaceOrientation::Up(Color::Yellow).ordinal())[1][0]
            != Color::Yellow
            || cube.get_face_state(FaceOrientation::Up(Color::Yellow).ordinal())[1][2]
                != Color::Yellow
        {
            rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, steps);
        }

        // Apply algorithm
        let front_face = FaceOrientation::Front(Color::Blue);
        rotate_and_record(cube, front_face, true, steps);
        rotate_and_record(cube, FaceOrientation::Right(Color::Red), true, steps);
        rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, steps);
        rotate_and_record(cube, FaceOrientation::Right(Color::Red), false, steps);
        rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), false, steps);
        rotate_and_record(cube, front_face, false, steps);
    }
}
