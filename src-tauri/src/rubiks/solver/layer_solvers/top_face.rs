use super::super::{Solver, SolverEnum};
use super::top_edge::TopEdgeSolver;
use crate::rubiks::cube::color::Color;
use crate::rubiks::cube::face::FaceOrientation;
use crate::rubiks::cube::Cube;
use super::utils::*;
pub struct TopFaceSolver {}

impl Solver for TopFaceSolver {
    fn target(&self) -> super::super::SolveTarget {
        super::super::SolveTarget::TopFace
    }

    fn solve_target(&mut self, cube: &mut Cube) -> Vec<char> {
        let mut steps = vec![];

        while !self.is_target_solved(cube) {
            let yellow_count = Self::count_yellow_corners(cube);

            match yellow_count {
                0 => Self::solve_no_yellow(cube, &mut steps),
                1 => Self::solve_one_yellow(cube, &mut steps),
                2 => Self::solve_two_yellow(cube, &mut steps),
                3 => Self::solve_three_yellow(cube, &mut steps),
                _ => break,
            }
        }

        steps
    }

    fn is_target_solved(&self, cube: &Cube) -> bool {
        let face_colors = cube.get_face_state(FaceOrientation::Up(Color::Yellow).ordinal());
        let color = face_colors[1][1];

        for i in 0..3 {
            for j in 0..3 {
                if face_colors[i][j] != color {
                    return false;
                }
            }
        }

        true
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        Some(SolverEnum::TopEdge(TopEdgeSolver {}))
    }
}

impl TopFaceSolver {
    fn count_yellow_corners(cube: &Cube) -> u8 {
        let face_colors = cube.get_face_state(FaceOrientation::Up(Color::Yellow).ordinal());
        let mut count = 0;

        if face_colors[0][0] == Color::Yellow { count += 1; }
        if face_colors[0][2] == Color::Yellow { count += 1; }
        if face_colors[2][0] == Color::Yellow { count += 1; }
        if face_colors[2][2] == Color::Yellow { count += 1; }

        count
    }

    fn solve_no_yellow(cube: &mut Cube, steps: &mut Vec<char>) {
        Self::apply_algorithm(cube, steps);
    }

    fn solve_one_yellow(cube: &mut Cube, steps: &mut Vec<char>) {
        // Rotate until yellow corner is at front-right
        while cube.get_face_state(FaceOrientation::Up(Color::Yellow).ordinal())[2][2] != Color::Yellow {
            rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, steps);
        }
        Self::apply_algorithm(cube, steps);
    }

    fn solve_two_yellow(cube: &mut Cube, steps: &mut Vec<char>) {
        // Rotate until yellow corners form a line
        while cube.get_face_state(FaceOrientation::Up(Color::Yellow).ordinal())[0][0] != Color::Yellow
            || cube.get_face_state(FaceOrientation::Up(Color::Yellow).ordinal())[2][2] != Color::Yellow
        {
            rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, steps);
        }
        Self::apply_algorithm(cube, steps);
    }

    fn solve_three_yellow(cube: &mut Cube, steps: &mut Vec<char>) {
        // Rotate until non-yellow corner is at front-right
        while cube.get_face_state(FaceOrientation::Up(Color::Yellow).ordinal())[2][2] == Color::Yellow {
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
    }
}