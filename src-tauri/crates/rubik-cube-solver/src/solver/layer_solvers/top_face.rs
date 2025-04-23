use super::prelude::*;

pub struct TopFaceSolver {}

impl Solver for TopFaceSolver {
    fn target(&self) -> super::super::SolveTarget {
        super::super::SolveTarget::TopFace
    }

    fn solve_target(&mut self, cube: &mut Cube) -> Vec<char> {
        let mut steps = vec![];
        let mut count = 0;
        
        while !self.is_target_solved(cube) {
            if count > 6 {
                panic!("Exceeded maximum iterations");
            }
            count += 1;
            
            let yellow_corners = self.count_yellow_corners(cube);
            if yellow_corners == 1 {
                if self.on_right_side(cube) {
                    self.right_hand_algorithm(cube, &mut steps);
                } else {
                    self.left_hand_algorithm(cube, &mut steps);
                }
            } else if yellow_corners == 2 {
                self.align_2_not_yellow(cube, &mut steps);
                self.right_hand_algorithm(cube, &mut steps);
            } else if yellow_corners == 0 {
                self.align_4_not_yellow(cube, &mut steps);
                self.right_hand_algorithm(cube, &mut steps);
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
        Some(SolverEnum::TopCorner(TopCornerSolver {}))
    }
}


impl TopFaceSolver {
    fn count_yellow_corners(&self, cube: &Cube) -> usize {
        let up = FaceOrientation::Up(Color::Yellow);
        let up_ordinal = up.ordinal();
        
        let mut count = 0;
        if cube.get_block_color(up_ordinal, 0, 0) == Color::Yellow { count += 1; }
        if cube.get_block_color(up_ordinal, 0, 2) == Color::Yellow { count += 1; }
        if cube.get_block_color(up_ordinal, 2, 0) == Color::Yellow { count += 1; }
        if cube.get_block_color(up_ordinal, 2, 2) == Color::Yellow { count += 1; }
        count
    }

    fn on_right_side(&self, cube: &mut Cube) -> bool {
        let up = FaceOrientation::Up(Color::Yellow);
        let left = FaceOrientation::Left(Color::Green);
        let front = FaceOrientation::Front(Color::Blue);
        let right = FaceOrientation::Right(Color::Red);
        let back = FaceOrientation::Back(Color::Orange);
        
        if cube.get_block_color(up.ordinal(), 0, 0) == Color::Yellow {
            if cube.get_block_color(left.ordinal(), 0, 2) == Color::Yellow {
                rotate_and_record(cube, up, false, &mut vec![]);
                return true;
            }
            if cube.get_block_color(back.ordinal(), 0, 0) == Color::Yellow {
                rotate_and_record(cube, up, true, &mut vec![]);
                rotate_and_record(cube, up, true, &mut vec![]);
                return false;
            }
        }

        if cube.get_block_color(up.ordinal(), 0, 2) == Color::Yellow {
            if cube.get_block_color(back.ordinal(), 0, 2) == Color::Yellow {
                rotate_and_record(cube, up, false, &mut vec![]);
                rotate_and_record(cube, up, false, &mut vec![]);
                return true;
            }
            if cube.get_block_color(right.ordinal(), 0, 0) == Color::Yellow {
                rotate_and_record(cube, up, true, &mut vec![]);
                return false;
            }
        }

        if cube.get_block_color(up.ordinal(), 2, 2) == Color::Yellow {
            if cube.get_block_color(right.ordinal(), 0, 2) == Color::Yellow {
                rotate_and_record(cube, up, true, &mut vec![]);
                return true;
            }
            if cube.get_block_color(front.ordinal(), 0, 0) == Color::Yellow {
                return false;
            }
        }

        if cube.get_block_color(up.ordinal(), 2, 0) == Color::Yellow {
            if cube.get_block_color(front.ordinal(), 0, 2) == Color::Yellow {
                rotate_and_record(cube, up, true, &mut vec![]);
                return true;
            }
            if cube.get_block_color(left.ordinal(), 0, 0) == Color::Yellow {
                rotate_and_record(cube, up, false, &mut vec![]);
                return false;
            }
        }
        
        panic!("Invalid state for on_right_side check");
    }

    fn align_2_not_yellow(&self, cube: &mut Cube, steps: &mut Vec<char>) {
        let front = FaceOrientation::Front(Color::Blue);
        while cube.get_block_color(front.ordinal(), 0, 0) != Color::Yellow {
            rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, steps);
        }
    }

    fn align_4_not_yellow(&self, cube: &mut Cube, steps: &mut Vec<char>) {
        let left = FaceOrientation::Left(Color::Green);
        while cube.get_block_color(left.ordinal(), 0, 2) != Color::Yellow {
            rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, steps);
        }
    }

    fn left_hand_algorithm(&self, cube: &mut Cube, steps: &mut Vec<char>) {
        let left = FaceOrientation::Left(Color::Green);
        let up = FaceOrientation::Up(Color::Yellow);
        
        rotate_and_record(cube, left, false, steps);
        rotate_and_record(cube, up, false, steps);
        rotate_and_record(cube, left, true, steps);
        rotate_and_record(cube, up, false, steps);
        rotate_and_record(cube, left, false, steps);
        rotate_and_record(cube, up, false, steps);
        rotate_and_record(cube, up, false, steps);
        rotate_and_record(cube, left, true, steps);
    }

    fn right_hand_algorithm(&self, cube: &mut Cube, steps: &mut Vec<char>) {
        let right = FaceOrientation::Right(Color::Red);
        let up = FaceOrientation::Up(Color::Yellow);
        
        rotate_and_record(cube, right, true, steps);
        rotate_and_record(cube, up, true, steps);
        rotate_and_record(cube, right, false, steps);
        rotate_and_record(cube, up, true, steps);
        rotate_and_record(cube, right, true, steps);
        rotate_and_record(cube, up, true, steps);
        rotate_and_record(cube, up, true, steps);
        rotate_and_record(cube, right, false, steps);
    }
}
