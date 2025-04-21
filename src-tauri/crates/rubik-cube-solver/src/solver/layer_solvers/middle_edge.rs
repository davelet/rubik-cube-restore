use crate::utils::print_cube;

use super::prelude::*;

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

        let faces_to_solve = [
            FaceOrientation::Front(Color::Blue),
            FaceOrientation::Right(Color::Red),
            FaceOrientation::Back(Color::Green),
            FaceOrientation::Left(Color::Orange),
        ];

        for face in faces_to_solve {
            if Self::is_edge_correct(cube, face) {
                continue;
            }

            if Self::has_target_edge_in_middle(cube, face) {
                Self::extract_target_edge(cube, face, &mut steps);
            }

            Self::find_target_edge_on_top(cube, face, &mut steps);

            if Self::is_edge_ready_for_right_insert(cube, face) {
                Self::insert_edge_right(cube, face, &mut steps);
            } else {
                // The Java code rotates U' then calls insert_edge_left on the *right* side.
                // Let's stick to the Java logic for now.
                rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), false, &mut steps);
                let right_side = get_right_side(face);
                Self::insert_edge_left(cube, right_side, &mut steps);
            }
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

    // Checks if the middle edge between 'face' and its right side is correctly placed.
    fn is_edge_correct(cube: &Cube, face: FaceOrientation) -> bool {
        let face_color = face.color();
        let right_side = get_right_side(face);
        let right_color = right_side.color();

        cube.get_block_color(face.ordinal(), 1, 2) == face_color
            && cube.get_block_color(right_side.ordinal(), 1, 0) == right_color
    }

    // Checks if the target edge for 'target_face' is currently misplaced in another middle layer slot.
    fn has_target_edge_in_middle(cube: &Cube, target_face: FaceOrientation) -> bool {
        let target_color = target_face.color();
        let target_right_color = get_right_side(target_face).color();

        let faces_to_check = [
            FaceOrientation::Front(Color::Blue),
            FaceOrientation::Right(Color::Red),
            FaceOrientation::Back(Color::Green),
            FaceOrientation::Left(Color::Orange),
        ];

        for check_face in faces_to_check {
            // Skip the target face itself
            if check_face.ordinal() == target_face.ordinal() {
                continue;
            }
            let check_right_side = get_right_side(check_face);
            let edge_color1 = cube.get_block_color(check_face.ordinal(), 1, 2);
            let edge_color2 = cube.get_block_color(check_right_side.ordinal(), 1, 0);

            if (edge_color1 == target_color && edge_color2 == target_right_color)
                || (edge_color1 == target_right_color && edge_color2 == target_color)
            {
                return true;
            }
        }
        false
    }

    // Finds the misplaced edge corresponding to 'target_face' and extracts it to the top layer.
    fn extract_target_edge(cube: &mut Cube, target_face: FaceOrientation, steps: &mut Vec<char>) {
        let target_color = target_face.color();
        let target_right_color = get_right_side(target_face).color();

        let faces_to_check = [
            FaceOrientation::Front(Color::Blue),
            FaceOrientation::Right(Color::Red),
            FaceOrientation::Back(Color::Green),
            FaceOrientation::Left(Color::Orange),
        ];

        for check_face in faces_to_check {
            let check_right_side = get_right_side(check_face);
            let edge_color1 = cube.get_block_color(check_face.ordinal(), 1, 2);
            let edge_color2 = cube.get_block_color(check_right_side.ordinal(), 1, 0);

            if (edge_color1 == target_color && edge_color2 == target_right_color)
                || (edge_color1 == target_right_color && edge_color2 == target_color)
            {
                // Use insert_edge_right on the face where the edge was found to bring it up.
                Self::insert_edge_right(cube, check_face, steps);
                return; // Found and extracted
            }
        }
        // Should not happen if has_target_edge_in_middle was true
        // panic!("Could not find target edge to extract");
    }

    // Rotates the top layer until the target edge for 'target_face' is positioned above 'target_face'.
    fn find_target_edge_on_top(cube: &mut Cube, target_face: FaceOrientation, steps: &mut Vec<char>) {
        let target_color = target_face.color();
        let target_right_color = get_right_side(target_face).color();
        let up_face = FaceOrientation::Up(Color::Yellow);
        let up_ordinal = up_face.ordinal();
        let (up_row, up_col) = get_up_center(target_face);
        let face_ordinal = target_face.ordinal();

        let mut count = 0;
        loop {
            let top_color = cube.get_block_color(up_ordinal, up_row, up_col);
            let front_color = cube.get_block_color(face_ordinal, 0, 1);

            if (top_color == target_color && front_color == target_right_color)
                || (top_color == target_right_color && front_color == target_color)
            {
                break; // Found the edge
            }

            rotate_and_record(cube, up_face, true, steps);
            count += 1;
            if count > 4 {
                print_cube(cube);
                panic!("find_target_edge_on_top exceeded max rotations");
            }
        }
    }

    // Checks if the edge currently on top, above 'face', is oriented correctly for a right insert.
    fn is_edge_ready_for_right_insert(cube: &Cube, face: FaceOrientation) -> bool {
        let face_color = face.color();
        // Check the color on the 'face' side of the top edge piece
        cube.get_block_color(face.ordinal(), 0, 1) == face_color
    }

    // Performs the left insertion algorithm: U' L' U L U F U' F'
    fn insert_edge_left(cube: &mut Cube, face: FaceOrientation, steps: &mut Vec<char>) {
        let up = FaceOrientation::Up(Color::Yellow);
        let left = get_left_side(face);

        rotate_and_record(cube, up, false, steps); // U'
        rotate_and_record(cube, left, false, steps); // L'
        rotate_and_record(cube, up, true, steps); // U
        rotate_and_record(cube, left, true, steps); // L
        rotate_and_record(cube, up, true, steps); // U
        rotate_and_record(cube, face, true, steps); // F
        rotate_and_record(cube, up, false, steps); // U'
        rotate_and_record(cube, face, false, steps); // F'
    }

    // Performs the right insertion algorithm: U R U' R' U' F' U F
    fn insert_edge_right(cube: &mut Cube, face: FaceOrientation, steps: &mut Vec<char>) {
        let up = FaceOrientation::Up(Color::Yellow);
        let right = get_right_side(face);

        rotate_and_record(cube, up, true, steps); // U
        rotate_and_record(cube, right, true, steps); // R
        rotate_and_record(cube, up, false, steps); // U'
        rotate_and_record(cube, right, false, steps); // R'
        rotate_and_record(cube, up, false, steps); // U'
        rotate_and_record(cube, face, false, steps); // F'
        rotate_and_record(cube, up, true, steps); // U
        rotate_and_record(cube, face, true, steps); // F
    }
}
