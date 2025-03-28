use crate::rubiks::cube::{
        color::Color,
        face::FaceOrientation,
        Cube,
    };

use super::super::{Solver, SolverEnum};
use super::middle_edge::MiddleSolver;
use super::utils::*;
pub struct BottomCornerSolver {}

impl Solver for BottomCornerSolver {
    fn target(&self) -> super::super::SolveTarget {
        super::super::SolveTarget::BottomCorner
    }

    fn solve_target(&mut self, cube: &mut Cube) -> Vec<char> {
        let mut steps = vec![];

        for _ in 0..4 {
            // Check top layer corners
            let top_face = FaceOrientation::Up(Color::Yellow);
            Self::check_and_solve_corner(cube, top_face, 0, 0, &mut steps);
            Self::check_and_solve_corner(cube, top_face, 0, 2, &mut steps);
            Self::check_and_solve_corner(cube, top_face, 2, 0, &mut steps);
            Self::check_and_solve_corner(cube, top_face, 2, 2, &mut steps);

            // Check bottom layer corners
            let bottom_face = FaceOrientation::Down(Color::White);
            Self::check_and_solve_corner(cube, bottom_face, 0, 0, &mut steps);
            Self::check_and_solve_corner(cube, bottom_face, 0, 2, &mut steps);
            Self::check_and_solve_corner(cube, bottom_face, 2, 0, &mut steps);
            Self::check_and_solve_corner(cube, bottom_face, 2, 2, &mut steps);
        }

        steps
    }

    fn is_target_solved(&self, cube: &Cube) -> bool {
        let face_colors = cube.get_face_state(FaceOrientation::Down(Color::White).ordinal());
        let color = face_colors[1][1];
        for i in 0..3 {
            for j in 0..3 {
                if i == 1 && j == 1 {
                    continue;
                }
                if face_colors[i][j] != color {
                    return false;
                }
            }
        }

        for f in 2..=5 {
            let face_colors = cube.state[f];
            let color = face_colors[1][1];
            for j in 0..3 {
                if face_colors[2][j] != color {
                    return false;
                }
            }
        }

        true
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        Some(SolverEnum::MiddleEdge(MiddleSolver {}))
    }
}

impl BottomCornerSolver {
    fn check_and_solve_corner(
        cube: &mut Cube,
        face: FaceOrientation,
        row: usize,
        col: usize,
        steps: &mut Vec<char>,
    ) {
        if Self::is_white_corner(cube, face, row, col) {
            let mut row0 = row;
            if face == FaceOrientation::Down(Color::White) {
                Self::extract_corner(cube, face, steps);
                row0 = 2 - row;
            }
            Self::align_corner(cube, face, row0, col, steps);
        }
    }

    fn is_white_corner(cube: &Cube, face: FaceOrientation, row: usize, col: usize) -> bool {
        let colors = Self::get_corner_colors(cube, face, row, col);
        colors.0 == Color::White || colors.1 == Color::White || colors.2 == Color::White
    }

    fn extract_corner(cube: &mut Cube, face: FaceOrientation, steps: &mut Vec<char>) {
        let right_face = get_right_side(face);

        rotate_and_record(cube, right_face, true, steps);
        rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, steps);
        rotate_and_record(cube, right_face, false, steps);
        rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), false, steps);
    }

    fn align_corner(
        cube: &mut Cube,
        face: FaceOrientation,
        row: usize,
        col: usize,
        steps: &mut Vec<char>,
    ) {
        let mut current_row = row;
        let mut current_col = col;

        while !Self::is_corner_aligned(cube, current_row, current_col) {
            rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, steps);

            // Update coordinates based on rotation
            (current_row, current_col) = match (current_row, current_col) {
                (0, 0) => (0, 2),
                (0, 2) => (2, 2),
                (2, 2) => (2, 0),
                (2, 0) => (0, 0),
                _ => panic!("Invalid corner position"),
            };
        }
        Self::insert_corner(cube, current_row, current_col, steps);
    }

    fn is_corner_aligned(cube: &Cube, row: usize, col: usize) -> bool {
        let colors = Self::get_corner_colors(cube, FaceOrientation::Up(Color::Yellow), row, col);
        let center_color = cube.get_block_color(FaceOrientation::Front(Color::Blue).ordinal(), 1, 1);
        colors.0 == center_color || colors.1 == center_color || colors.2 == center_color
    }

    fn insert_corner(cube: &mut Cube, row: usize, col: usize, steps: &mut Vec<char>) {
        let right_face = get_right_side(FaceOrientation::Front(Color::Blue));
        rotate_and_record(cube, right_face, true, steps);
        rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, steps);
        rotate_and_record(cube, right_face, false, steps);
    }

    fn get_corner_colors(
        cube: &Cube,
        face: FaceOrientation,
        row: usize,
        col: usize,
    ) -> (Color, Color, Color) {
        match face {
            FaceOrientation::Up(_) => match (row, col) {
                (0, 0) => (
                    cube.get_block_color(face.ordinal(), 0, 0),
                    cube.get_block_color(FaceOrientation::Back(Color::Green).ordinal(), 0, 2),
                    cube.get_block_color(FaceOrientation::Left(Color::Orange).ordinal(), 0, 0),
                ),
                (0, 2) => (
                    cube.get_block_color(face.ordinal(), 0, 2),
                    cube.get_block_color(FaceOrientation::Back(Color::Green).ordinal(), 0, 0),
                    cube.get_block_color(FaceOrientation::Right(Color::Red).ordinal(), 0, 2),
                ),
                (2, 0) => (
                    cube.get_block_color(face.ordinal(), 2, 0),
                    cube.get_block_color(FaceOrientation::Front(Color::Blue).ordinal(), 0, 0),
                    cube.get_block_color(FaceOrientation::Left(Color::Orange).ordinal(), 0, 2),
                ),
                (2, 2) => (
                    cube.get_block_color(face.ordinal(), 2, 2),
                    cube.get_block_color(FaceOrientation::Front(Color::Blue).ordinal(), 0, 2),
                    cube.get_block_color(FaceOrientation::Right(Color::Red).ordinal(), 0, 0),
                ),
                _ => panic!("Invalid corner position"),
            },
            FaceOrientation::Down(_) => match (row, col) {
                (0, 0) => (
                    cube.get_block_color(face.ordinal(), 0, 0),
                    cube.get_block_color(FaceOrientation::Front(Color::Blue).ordinal(), 2, 0),
                    cube.get_block_color(FaceOrientation::Left(Color::Orange).ordinal(), 2, 2),
                ),
                (0, 2) => (
                    cube.get_block_color(face.ordinal(), 0, 2),
                    cube.get_block_color(FaceOrientation::Front(Color::Blue).ordinal(), 2, 2),
                    cube.get_block_color(FaceOrientation::Right(Color::Red).ordinal(), 2, 0),
                ),
                (2, 0) => (
                    cube.get_block_color(face.ordinal(), 2, 0),
                    cube.get_block_color(FaceOrientation::Back(Color::Green).ordinal(), 2, 2),
                    cube.get_block_color(FaceOrientation::Left(Color::Orange).ordinal(), 2, 0),
                ),
                (2, 2) => (
                    cube.get_block_color(face.ordinal(), 2, 2),
                    cube.get_block_color(FaceOrientation::Back(Color::Green).ordinal(), 2, 0),
                    cube.get_block_color(FaceOrientation::Right(Color::Red).ordinal(), 2, 2),
                ),
                _ => panic!("Invalid corner position"),
            },
            _ => panic!("Invalid face orientation"),
        }
    }
}