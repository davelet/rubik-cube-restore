use crate::rubiks::cube::color::Color;
use crate::rubiks::cube::face::FaceOrientation;
use crate::rubiks::cube::Cube;

use super::{utils::*, BottomCornerSolver};

use super::super::{Solver, SolverEnum};
use super::top_cross::TopCrossSolver;

pub struct MiddleSolver;

impl Solver for MiddleSolver {
    fn target(&self) -> super::super::SolveTarget {
        super::super::SolveTarget::MiddleEdge
    }

    fn solve_target(&mut self, cube: &mut Cube) -> Vec<char> {
        if !Self::bottom_layer_solved(cube) {
            panic!("Bottom layer not solved");
        }

        let mut steps = vec![];

        for _ in 0..4 {
            // Check front face middle edges
            let front_face = FaceOrientation::Front(Color::Blue);
            Self::check_and_solve_edge(cube, front_face, &mut steps);

            // Check right face middle edges
            let right_face = FaceOrientation::Right(Color::Red);
            Self::check_and_solve_edge(cube, right_face, &mut steps);

            // Check back face middle edges
            let back_face = FaceOrientation::Back(Color::Green);
            Self::check_and_solve_edge(cube, back_face, &mut steps);

            // Check left face middle edges
            let left_face = FaceOrientation::Left(Color::Orange);
            Self::check_and_solve_edge(cube, left_face, &mut steps);
        }

        steps
    }

    fn is_target_solved(&self, cube: &Cube) -> bool {
        for f in 2..=5 {
            let face_colors = cube.state[f];
            let color = face_colors[1][1];
            for i in 0..3 {
                if face_colors[1][i] != color {
                    return false;
                }
            }
        }
        true
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        Some(SolverEnum::TopCross(TopCrossSolver {}))
    }
}

impl MiddleSolver {
    fn bottom_layer_solved(cube: &Cube) -> bool {
        (&BottomCornerSolver).is_target_solved(cube)
    }

    fn check_and_solve_edge(cube: &mut Cube, face: FaceOrientation, steps: &mut Vec<char>) {
        if !Self::is_edge_solved(cube, face) {
            if Self::find_edge_in_top(cube, face, steps) {
                Self::insert_edge(cube, face, steps);
            }
        }
    }

    fn is_edge_solved(cube: &Cube, face: FaceOrientation) -> bool {
        let face_color = face.color();
        let face_idx = face.ordinal();

        // Check if middle edge piece matches center color
        if cube.get_block_color(face_idx, 1, 0) != face_color
            || cube.get_block_color(face_idx, 1, 2) != face_color
        {
            return false;
        }

        // Check adjacent faces
        let left_side = get_left_side(face);
        let right_side = get_right_side(face);

        let left_color = left_side.color();
        let right_color = right_side.color();

        cube.get_block_color(left_side.ordinal(), 1, 2) == left_color
            && cube.get_block_color(right_side.ordinal(), 1, 0) == right_color
    }

    fn find_edge_in_top(cube: &mut Cube, face: FaceOrientation, steps: &mut Vec<char>) -> bool {
        let face_color = face.color();
        let up_face = FaceOrientation::Up(Color::Yellow);

        // Check if edge piece is in top layer
        for _ in 0..4 {
            let up_center = get_up_center(face);
            if cube.get_block_color(up_face.ordinal(), up_center.0, up_center.1) == face_color {
                return true;
            }
            rotate_and_record(cube, up_face, true, steps);
        }

        false
    }

    fn insert_edge(cube: &mut Cube, face: FaceOrientation, steps: &mut Vec<char>) {
        let right_face = get_right_side(face);

        // Standard algorithm for inserting edge piece from top layer to middle layer
        rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, steps);
        rotate_and_record(cube, right_face, true, steps);
        rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), false, steps);
        rotate_and_record(cube, right_face, false, steps);
        rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), false, steps);
        rotate_and_record(cube, face, false, steps);
        rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, steps);
        rotate_and_record(cube, face, true, steps);
    }
}
