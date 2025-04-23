use super::prelude::*;

pub struct TopCrossSolver {}

impl Solver for TopCrossSolver {
    fn target(&self) -> super::super::SolveTarget {
        super::super::SolveTarget::TopCross
    }

    fn solve_target(&mut self, cube: &mut Cube) -> Vec<char> {
        let mut steps = vec![];
        let mut count = 0;
        
        while !self.is_target_solved(cube) {
            if count > 4 {
                panic!("Exceeded maximum iterations");
            }
            count += 1;
            
            if Self::is_top_dot(cube) {
                Self::execute_top_cross(cube, &mut steps);
            } else if Self::is_top_l(cube) {
                Self::execute_top_cross(cube, &mut steps);
            } else {
                Self::align_top_line(cube, &mut steps);
                Self::execute_top_cross(cube, &mut steps);
            }
        }
        
        steps
    }
    
  

    fn is_target_solved(&self, cube: &Cube) -> bool {
        let face_colors = cube.get_face_state(FaceOrientation::Up(Color::Yellow).ordinal());
        let color = face_colors[1][1]; // Center color

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
    fn is_top_dot(cube: &Cube) -> bool {
        let up = FaceOrientation::Up(Color::Yellow);
        let up_ordinal = up.ordinal();
        
        cube.get_block_color(up_ordinal, 0, 1) != Color::Yellow &&
        cube.get_block_color(up_ordinal, 1, 0) != Color::Yellow &&
        cube.get_block_color(up_ordinal, 1, 2) != Color::Yellow &&
        cube.get_block_color(up_ordinal, 2, 1) != Color::Yellow
    }
    
    fn is_top_l(cube: &mut Cube) -> bool {
        let up = FaceOrientation::Up(Color::Yellow);
        let up_ordinal = up.ordinal();
        
        let down = cube.get_block_color(up_ordinal, 2, 1) == Color::Yellow;
        let left = cube.get_block_color(up_ordinal, 1, 0) == Color::Yellow;
        let right = cube.get_block_color(up_ordinal, 1, 2) == Color::Yellow;
        let up_color = cube.get_block_color(up_ordinal, 0, 1) == Color::Yellow;
        
        let left_down = left && down;
        if left_down {
            rotate_and_record(cube, up, true, &mut vec![]);
            return true;
        }
        
        let right_down = right && down;
        if right_down {
            rotate_and_record(cube, up, false, &mut vec![]);
            rotate_and_record(cube, up, false, &mut vec![]);
            return true;
        }
        
        let right_up = right && up_color;
        if right_up {
            rotate_and_record(cube, up, false, &mut vec![]);
            return true;
        }
        
        left && up_color
    }
    
    fn is_top_line(cube: &Cube) -> bool {
        let up = FaceOrientation::Up(Color::Yellow);
        let up_ordinal = up.ordinal();
        
        cube.get_block_color(up_ordinal, 1, 0) == Color::Yellow &&
        cube.get_block_color(up_ordinal, 1, 2) == Color::Yellow
    }
    
    fn execute_top_cross(cube: &mut Cube, steps: &mut Vec<char>) {
        let front = FaceOrientation::Front(Color::Blue);
        let right = FaceOrientation::Right(Color::Red);
        let up = FaceOrientation::Up(Color::Yellow);
        
        rotate_and_record(cube, front, true, steps);
        rotate_and_record(cube, right, true, steps);
        rotate_and_record(cube, up, true, steps);
        rotate_and_record(cube, right, false, steps);
        rotate_and_record(cube, up, false, steps);
        rotate_and_record(cube, front, false, steps);
    }
    
    fn align_top_line(cube: &mut Cube, steps: &mut Vec<char>) {
        if !Self::is_top_line(cube) {
            rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, steps);
        }
    }
}
