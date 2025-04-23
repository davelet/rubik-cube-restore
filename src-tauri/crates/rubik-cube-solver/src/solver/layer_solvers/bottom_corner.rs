use super::prelude::*;

pub struct BottomCornerSolver;

impl Solver for BottomCornerSolver {
    fn target(&self) -> SolveTarget {
        SolveTarget::BottomCorner
    }

    fn solve_target(&mut self, cube: &mut Cube) -> Vec<char> {
        let mut steps = vec![];

        'l: for _ in 0..4 {
            let top_face = Face::Up;
            let bottom_face = Face::Down;
            for i in [0, 2].iter() {
                for j in [0, 2].iter() {
                    if Self::check_and_solve_corner(cube, top_face, *i, *j, &mut steps) {
                        continue 'l;
                    }
                    if Self::check_and_solve_corner(cube, bottom_face, *i, *j, &mut steps) {
                        continue 'l;
                    }
                }
            }
        }

        steps
    }

    fn is_target_solved(&self, cube: &Cube) -> bool {
        let face_colors = cube.get_face_state(Face::Down.ordinal());
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
        face: Face,
        row: usize,
        col: usize,
        steps: &mut Vec<char>,
    ) -> bool {
        if Self::is_white_corner_need_solve(cube, face, row, col) {
            let mut row0 = row;
            if face == Face::Down {
                Self::extract_corner(cube, row, col, steps);
                row0 = 2 - row;
            }
            Self::align_top_corner(cube, row0, col, steps);
            return true;
        }
        false
    }

    fn is_white_corner_need_solve(
        cube: &Cube,
        face: Face,
        row: usize,
        col: usize,
    ) -> bool {
        let colors = Self::get_corner_colors(cube, face, row, col);
        let has_white =
            colors.0 == Color::White || colors.1 == Color::White || colors.2 == Color::White;
        if !has_white {
            return false;
        }

        if face == Face::Down {
            return !Self::is_bottom_corner_done(cube, row, col);
        }
        return true;
    }

    fn is_bottom_corner_done(cube: &Cube, row: usize, col: usize) -> bool {
        let colors = Self::get_corner_colors(cube, Face::Down, row, col);

        // Check if white faces down
        if colors.0 != Color::White {
            return false;
        }

        // Get center colors of adjacent faces
        let front_color = Color::Blue;
        let right_color = Color::Red;
        let back_color = Color::Green;
        let left_color = Color::Orange;

        // Check corner colors match with center colors
        match (row, col) {
            (0, 0) => colors.1 == front_color && colors.2 == left_color, // Left front corner
            (0, 2) => colors.1 == front_color && colors.2 == right_color, // Right front corner
            (2, 0) => colors.1 == back_color && colors.2 == left_color,  // Left back corner
            (2, 2) => colors.1 == back_color && colors.2 == right_color, // Right back corner
            _ => panic!("Invalid corner position"),
        }
    }

    fn extract_corner(cube: &mut Cube, row: usize, col: usize, steps: &mut Vec<char>) {
        let right_face =
            Self::get_right_face_on_position(Face::Down, row, col);

        rotate_and_record(cube, right_face, true, steps);
        rotate_and_record(cube, Face::Up, true, steps);
        rotate_and_record(cube, right_face, false, steps);
        rotate_and_record(cube, Face::Up, false, steps);
    }

    fn align_top_corner(cube: &mut Cube, row: usize, col: usize, steps: &mut Vec<char>) {
        let mut current_row = row;
        let mut current_col = col;

        while !Self::is_corner_aligned(cube, current_row, current_col) {
            rotate_and_record(cube, Face::Up, true, steps);

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
        let up_face = Face::Up;
        let colors = Self::get_corner_colors(cube, up_face, row, col);
        let faces = Self::get_side_faces(up_face, row, col);
        let center_colors = (
            Color::White,
            cube.get_block_color(faces.0.ordinal(), 1, 1),
            cube.get_block_color(faces.1.ordinal(), 1, 1),
        );
        (colors.0 == center_colors.0 || colors.1 == center_colors.0 || colors.2 == center_colors.0)
            && (colors.0 == center_colors.1
                || colors.1 == center_colors.1
                || colors.2 == center_colors.1)
            && (colors.0 == center_colors.2
                || colors.1 == center_colors.2
                || colors.2 == center_colors.2)
    }

    fn insert_corner(cube: &mut Cube, row: usize, col: usize, steps: &mut Vec<char>) {
        let right_face =
            Self::get_right_face_on_position(Face::Up, row, col);
        let mut at_bottom = false;

        for _ in 0..8 {
            if at_bottom {
                let bottom_row = 2 - row;
                if Self::is_bottom_corner_done(cube, bottom_row, col) {
                    break;
                } else {
                    rotate_and_record(cube, right_face, true, steps);
                    rotate_and_record(cube, Face::Up, true, steps);
                    rotate_and_record(cube, right_face, false, steps);
                    rotate_and_record(cube, Face::Up, false, steps);
                    at_bottom = false;
                }
            } else {
                rotate_and_record(cube, right_face, true, steps);
                rotate_and_record(cube, Face::Up, true, steps);
                rotate_and_record(cube, right_face, false, steps);
                at_bottom = true;
            }
        }
    }

    fn get_right_face_on_position(
        face: Face,
        row: usize,
        col: usize,
    ) -> Face {
        let (face1, face2) = Self::get_side_faces(face, row, col);
        if row == col {
            if face == Face::Up {
                face2
            } else {
                face1
            }
        } else {
            if face == Face::Up {
                face1
            } else {
                face2
            }
        }
    }

    fn get_side_faces(
        face: Face,
        row: usize,
        col: usize,
    ) -> (Face, Face) {
        match face {
            Face::Up => {
                let side1 = if row == 0 {
                    Face::Back
                } else if row == 2 {
                    Face::Front
                } else {
                    panic!("Invalid row for side faces");
                };
                let side2 = if col == 0 {
                    Face::Left
                } else if col == 2 {
                    Face::Right
                } else {
                    panic!("Invalid column for side faces");
                };
                (side1, side2)
            }
            Face::Down => {
                Self::get_side_faces(Face::Up, 2 - row, col)
            }
            _ => panic!("Invalid face orientation"),
        }
    }

    fn get_corner_colors(
        cube: &Cube,
        face: Face,
        row: usize,
        col: usize,
    ) -> (Color, Color, Color) {
        let (side1, side2) = Self::get_side_faces(face, row, col);
        let face_color0 = cube.get_block_color(face.ordinal(), row, col);
        match face {
            Face::Up => {
                let i = 0;
                let j = 2 - (col as isize - row as isize).abs() as usize;
                let face_color1 = cube.get_block_color(side1.ordinal(), i, j);

                let j = 2 - j;
                let face_color2 = cube.get_block_color(side2.ordinal(), i, j);

                (face_color0, face_color1, face_color2)
            }
            Face::Down => {
                let i = 2;
                let j = (row as isize - col as isize).abs() as usize;
                let face_color1 = cube.get_block_color(side1.ordinal(), i, j);

                let j = 2 - j;
                let face_color2 = cube.get_block_color(side2.ordinal(), i, j);
                (face_color0, face_color1, face_color2)
            }
            _ => panic!("Invalid face orientation"),
        }
    }
}
