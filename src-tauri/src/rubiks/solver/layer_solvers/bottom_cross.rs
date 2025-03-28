use crate::rubiks::cube::{
        color::Color,
        face::FaceOrientation,
        Cube,
    };

use super::super::{Solver, SolverEnum};
use super::bottom_corner::BottomCornerSolver;
use super::utils::*;

pub struct BottomCrossSolver;

impl Solver for BottomCrossSolver {
    fn target(&self) -> super::super::SolveTarget {
        super::super::SolveTarget::BottomCross
    }

    fn solve_target(&mut self, cube: &mut Cube) -> Vec<char> {
        let mut steps = vec![];

        for f in 2..=5 {
            let face = FaceOrientation::from_u8(f as u8);
            steps.extend(Self::solve_edge(cube, face));
        }

        steps
    }

    fn is_target_solved(&self, cube: &Cube) -> bool {
        let face_colors = cube.get_face_state(FaceOrientation::Down(Color::White).ordinal());
        let color = face_colors[1][1];
        for i in [0, 2] {
            if face_colors[i][1] != color {
                return false;
            }
            if face_colors[1][i] != color {
                return false;
            }
        }

        for f in 2..=5 {
            let face_colors = cube.state[f];
            let color = face_colors[1][1];
            if face_colors[2][1] != color {
                return false;
            }
        }

        true
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        Some(SolverEnum::BottomCorner(BottomCornerSolver {}))
    }
}

impl BottomCrossSolver {
    fn solve_edge(cube: &mut Cube, face: FaceOrientation) -> Vec<char> {
        let mut steps = vec![];
        if Self::is_edge_solved(cube, face) {
            return steps;
        }
        if Self::find_edge_in_top(cube, face, &mut steps)
            || Self::find_edge_in_middle(cube, face, &mut steps)
            || Self::find_edge_in_bottom(cube, face, &mut steps)
        {
            // 将边块插入到顶层
            rotate_and_record(cube, face, true, &mut steps);
            rotate_and_record(cube, face, true, &mut steps);
        }
        steps
    }

    fn is_edge_solved(cube: &mut Cube, face: FaceOrientation) -> bool {
        let face_colors = cube.get_face_state(face.ordinal());
        let color = face_colors[1][1];
        if face_colors[2][1] != color {
            return false;
        }

        let face_colors = cube.get_face_state(FaceOrientation::Down(Color::White).ordinal());
        let color = face_colors[1][1];
        let down_center = Self::down_center_index(face);
        if face_colors[down_center.0][down_center.1] != color {
            return false;
        }
        true
    }

    fn down_center_index(face: FaceOrientation) -> (usize, usize) {
        match face {
            FaceOrientation::Front(_) => (0, 1),
            FaceOrientation::Back(_) => (2, 1),
            FaceOrientation::Left(_) => (1, 0),
            FaceOrientation::Right(_) => (1, 2),
            _ => panic!("Invalid face orientation: {:?}", face),
        }
    }

    fn find_edge_in_top(cube: &mut Cube, face: FaceOrientation, steps: &mut Vec<char>) -> bool {
        let face_color = face.color();

        let up_center = get_up_center(face);
        let top_face = FaceOrientation::Up(Color::Yellow).ordinal();

        if cube.get_block_color(top_face, up_center.0, up_center.1) == Color::White
            && cube.get_block_color(face.ordinal(), 0, 1) == face_color
        {
            return true;
        }
        if cube.get_block_color(face.ordinal(), 0, 1) == Color::White
            && cube.get_block_color(top_face, up_center.0, up_center.1) == face_color
        {
            Self::swap_edge_on_top(cube, face, steps);
            return true;
        }

        let right_side = get_right_side(face);
        let up_center = get_up_center(right_side);
        if cube.get_block_color(top_face, up_center.0, up_center.1) == Color::White
            && cube.get_block_color(right_side.ordinal(), 0, 1) == face_color
        {
            rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, steps);
            return true;
        }
        if cube.get_block_color(right_side.ordinal(), 0, 1) == Color::White
            && cube.get_block_color(top_face, up_center.0, up_center.1) == face_color
        {
            rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, steps);
            Self::swap_edge_on_top(cube, face, steps);
            return true;
        }

        let left_side = get_left_side(face);
        let up_center = get_up_center(left_side);
        if cube.get_block_color(top_face, up_center.0, up_center.1) == Color::White
            && cube.get_block_color(left_side.ordinal(), 0, 1) == face_color
        {
            rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), false, steps);
            return true;
        }
        if cube.get_block_color(left_side.ordinal(), 0, 1) == Color::White
            && cube.get_block_color(top_face, up_center.0, up_center.1) == face_color
        {
            rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), false, steps);
            Self::swap_edge_on_top(cube, face, steps);
            return true;
        }

        let back_side = get_right_side(right_side);
        let up_center = get_up_center(back_side);
        if cube.get_block_color(top_face, up_center.0, up_center.1) == Color::White
            && cube.get_block_color(back_side.ordinal(), 0, 1) == face_color
        {
            rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), false, steps);
            rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), false, steps);
            return true;
        }
        if cube.get_block_color(back_side.ordinal(), 0, 1) == Color::White
            && cube.get_block_color(top_face, up_center.0, up_center.1) == face_color
        {
            rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), false, steps);
            rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), false, steps);
            Self::swap_edge_on_top(cube, face, steps);
            return true;
        }

        false
    }

    fn swap_edge_on_top(cube: &mut Cube, face: FaceOrientation, steps: &mut Vec<char>) {
        let right_face = get_right_side(face);
        rotate_and_record(cube, face, true, steps);
        rotate_and_record(cube, right_face, true, steps);
        rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, steps);
        rotate_and_record(cube, right_face, false, steps);
    }

    fn find_edge_in_middle(cube: &mut Cube, face: FaceOrientation, steps: &mut Vec<char>) -> bool {
        let face_color = face.color();

        let left_side = get_left_side(face);
        if cube.get_block_color(left_side.ordinal(), 1, 2) == Color::White
            && cube.get_block_color(face.ordinal(), 1, 0) == face_color
        {
            rotate_and_record(cube, face, true, steps);
            return true;
        }
        if cube.get_block_color(face.ordinal(), 1, 0) == Color::White
            && cube.get_block_color(left_side.ordinal(), 1, 2) == face_color
        {
            rotate_and_record(cube, left_side, false, steps);
            rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), false, steps);
            rotate_and_record(cube, left_side, true, steps);
            return true;
        }

        let right_side = get_right_side(face);
        if cube.get_block_color(right_side.ordinal(), 1, 0) == Color::White
            && cube.get_block_color(face.ordinal(), 1, 2) == face_color
        {
            rotate_and_record(cube, face, false, steps);
            return true;
        }
        if cube.get_block_color(face.ordinal(), 1, 2) == Color::White
            && cube.get_block_color(right_side.ordinal(), 1, 0) == face_color
        {
            rotate_and_record(cube, right_side, true, steps);
            rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, steps);
            rotate_and_record(cube, right_side, false, steps);
            return true;
        }

        let back_side = get_right_side(right_side);
        if cube.get_block_color(back_side.ordinal(), 1, 0) == Color::White
            && cube.get_block_color(right_side.ordinal(), 1, 2) == face_color
        {
            rotate_and_record(cube, right_side, false, steps);
            rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, steps);
            rotate_and_record(cube, right_side, true, steps);
            return true;
        }
        if cube.get_block_color(right_side.ordinal(), 1, 2) == Color::White
            && cube.get_block_color(back_side.ordinal(), 1, 0) == face_color
        {
            rotate_and_record(cube, back_side, true, steps);
            rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), false, steps);
            rotate_and_record(cube, back_side, false, steps);
            rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), false, steps);
            return true;
        }

        if cube.get_block_color(back_side.ordinal(), 1, 2) == Color::White
            && cube.get_block_color(left_side.ordinal(), 1, 0) == face_color
        {
            rotate_and_record(cube, left_side, true, steps);
            rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), false, steps);
            rotate_and_record(cube, left_side, false, steps);
            return true;
        }
        if cube.get_block_color(left_side.ordinal(), 1, 0) == Color::White
            && cube.get_block_color(back_side.ordinal(), 1, 2) == face_color
        {
            rotate_and_record(cube, back_side, false, steps);
            rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), false, steps);
            rotate_and_record(cube, back_side, true, steps);
            rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), false, steps);
            return true;
        }

        false
    }

    fn find_edge_in_bottom(cube: &mut Cube, face: FaceOrientation, steps: &mut Vec<char>) -> bool {
        let face_color = face.color();
        let down_center = Self::down_center_index(face);

        if cube.get_block_color(
            FaceOrientation::Down(Color::White).ordinal(),
            down_center.0,
            down_center.1,
        ) == face_color
            && cube.get_block_color(face.ordinal(), 2, 1) == Color::White
        {
            rotate_and_record(cube, face, true, steps);
            rotate_and_record(cube, face, true, steps);
            return Self::find_edge_in_top(cube, face, steps);
        }

        let right_side = get_right_side(face);
        if Self::check_bottom_edge(cube, face, right_side, steps) {
            return true;
        }

        let left_side = get_left_side(face);
        if Self::check_bottom_edge(cube, face, left_side, steps) {
            return true;
        }

        let back_side = get_left_side(left_side);
        if Self::check_bottom_edge(cube, face, back_side, steps) {
            return true;
        }

        false
    }

    fn check_bottom_edge(
        cube: &mut Cube,
        current_face: FaceOrientation,
        side_face: FaceOrientation,
        steps: &mut Vec<char>,
    ) -> bool {
        let down_center = Self::down_center_index(side_face);
        let current_color = current_face.color();

        let side_color = cube.get_block_color(side_face.ordinal(), 2, 1);
        let down_color = cube.get_block_color(
            FaceOrientation::Down(Color::White).ordinal(),
            down_center.0,
            down_center.1,
        );

        if (side_color == current_color && down_color == Color::White)
            || (side_color == Color::White && down_color == current_color)
        {
            rotate_and_record(cube, side_face, true, steps);
            rotate_and_record(cube, side_face, true, steps);
            return Self::find_edge_in_top(cube, current_face, steps);
        }
        false
    }
}