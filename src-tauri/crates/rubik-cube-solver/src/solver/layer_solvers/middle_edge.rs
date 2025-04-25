use crate::utils::print_cube;

use super::prelude::*;

pub struct MiddleSolver;

fn side_faces() -> [Face; 4] {
    [Face::Front, Face::Right, Face::Back, Face::Left]
}

impl Solver for MiddleSolver {
    fn target(&self) -> super::super::SolveTarget {
        super::super::SolveTarget::MiddleEdge
    }

    fn solve_target(&mut self, cube: &mut Cube) -> Vec<char> {
        let mut steps = vec![];
        // print_cube(cube);

        for face in side_faces() {
            if Self::is_edge_correct(cube, face) {
                continue;
            }
            Self::handle_target_edge_in_middle(cube, face, &mut steps);

            Self::settle_target_edge_on_top(cube, face, &mut steps);

            if cube.get_block_color(face.ordinal(), 0, 1) == face.color() {
                Self::insert_edge_right(cube, face, &mut steps);
            } else {
                // The Java code rotates U' then calls insert_edge_left on the *right* side.
                // Let's stick to the Java logic for now.
                rotate_and_record(cube, Face::Up, false, &mut steps);
                let right_side = get_right_side(face);
                Self::insert_edge_left(cube, right_side, &mut steps);
            }
        }

        steps
    }

    fn is_target_solved(&self, cube: &Cube) -> bool {
        for f in side_faces() {
            let face_colors = cube.state[f.ordinal()];
            let color = f.color();
            if face_colors[1][0] != color || face_colors[1][2] != color {
                return false;
            }
        }
        true
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        Some(SolverEnum::TopCross(TopCrossSolver {}))
    }
}

impl MiddleSolver {
    // Checks if the middle edge between 'face' and its right side is correctly placed.
    fn is_edge_correct(cube: &Cube, face: Face) -> bool {
        let right_side = get_right_side(face);
        cube.get_block_color(face.ordinal(), 1, 2) == face.color()
            && cube.get_block_color(right_side.ordinal(), 1, 0) == right_side.color()
    }

    // Checks if the target edge for 'target_face' is currently misplaced in another middle layer slot.
    fn handle_target_edge_in_middle(cube: &mut Cube, target_face: Face, steps: &mut Vec<char>) {
        let target_color = target_face.color();
        let target_right_color = get_right_side(target_face).color();

        for check_face in side_faces() {
            // Skip the target face itself
            // if check_face == target_face {
            //     continue;
            // }
            let check_right_side = get_right_side(check_face);
            let edge_color1 = cube.get_block_color(check_face.ordinal(), 1, 2);
            let edge_color2 = cube.get_block_color(check_right_side.ordinal(), 1, 0);

            if edge_color1 == target_color && edge_color2 == target_right_color {
                Self::insert_edge_right(cube, check_face, steps);
                return;
            }
            if edge_color1 == target_right_color && edge_color2 == target_color {
                Self::insert_edge_right(cube, check_face, steps);
                return;
            }
        }
    }

    // Rotates the top layer until the target edge for 'target_face' is positioned above 'target_face'.
    fn settle_target_edge_on_top(cube: &mut Cube, target_face: Face, steps: &mut Vec<char>) {
        let target_color = target_face.color();
        let target_right_color = get_right_side(target_face).color();
        let (up_row, up_col) = get_up_center(target_face);

        let mut count = 0;
        loop {
            let top_color = cube.get_block_color(Face::Up.ordinal(), up_row, up_col);
            let front_color = cube.get_block_color(target_face.ordinal(), 0, 1);

            if (top_color == target_color && front_color == target_right_color)
                || (top_color == target_right_color && front_color == target_color)
            {
                break; // Found the edge
            }

            rotate_and_record(cube, Face::Up, true, steps);
            count += 1;
            if count > 4 {
                print_cube(cube);
                panic!("find_target_edge_on_top exceeded max rotations");
            }
        }
    }

    // Performs the left insertion algorithm: U' L' U L U F U' F'
    fn insert_edge_left(cube: &mut Cube, face: Face, steps: &mut Vec<char>) {
        let up = Face::Up;
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
    fn insert_edge_right(cube: &mut Cube, face: Face, steps: &mut Vec<char>) {
        let up = Face::Up;
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
