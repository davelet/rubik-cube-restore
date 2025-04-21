use super::prelude::*;

pub struct TopCornerSolver {}

impl TopCornerSolver {
    fn has_eyed_corner(&self, cube: &Cube) -> Option<FaceOrientation> {
        for face in [
            FaceOrientation::Front(Color::Blue),
            FaceOrientation::Right(Color::Red),
            FaceOrientation::Back(Color::Orange),
            FaceOrientation::Left(Color::Green)
        ] {
            if cube.get_block_color(face.ordinal(), 0, 0) == cube.get_block_color(face.ordinal(), 0, 2) {
                return Some(face);
            }
        }
        None
    }
    
    fn align_solved_corner(&self, cube: &mut Cube, face: FaceOrientation, steps: &mut Vec<char>) {
        match face {
            FaceOrientation::Right(Color::Red) => {
                rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, steps);
            },
            FaceOrientation::Back(Color::Orange) => {
                rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, steps);
                rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, steps);
            },
            FaceOrientation::Left(Color::Green) => {
                rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), false, steps);
            },
            _ => {}
        }
    }
    
    fn execute_corner_permutation_algorithm(&self, cube: &mut Cube, steps: &mut Vec<char>) {
        // RB'RF2 R'BRF2 R2
        let right = FaceOrientation::Right(Color::Red);
        let back = FaceOrientation::Back(Color::Orange);
        let front = FaceOrientation::Front(Color::Blue);
        
        rotate_and_record(cube, right, true, steps);
        rotate_and_record(cube, back, false, steps);
        rotate_and_record(cube, right, true, steps);
        rotate_and_record(cube, front, true, steps);
        rotate_and_record(cube, front, true, steps);
        
        rotate_and_record(cube, right, false, steps);
        rotate_and_record(cube, back, true, steps);
        rotate_and_record(cube, right, true, steps);
        rotate_and_record(cube, front, true, steps);
        rotate_and_record(cube, front, true, steps);
        
        rotate_and_record(cube, right, true, steps);
        rotate_and_record(cube, right, true, steps);
    }
    
    fn almost_ready(&self, cube: &Cube) -> bool {
        for face in [
            FaceOrientation::Front(Color::Blue),
            FaceOrientation::Right(Color::Red),
            FaceOrientation::Back(Color::Orange),
            FaceOrientation::Left(Color::Green)
        ] {
            if cube.get_block_color(face.ordinal(), 0, 0) != cube.get_block_color(face.ordinal(), 0, 2) {
                return false;
            }
        }
        true
    }
}

impl Solver for TopCornerSolver {
    fn target(&self) -> super::super::SolveTarget {
        super::super::SolveTarget::TopCorner
    }

    fn solve_target(&mut self, cube: &mut Cube) -> Vec<char> {
        let mut steps = vec![];
        let mut count = 0;
        let mut to_fix = false;
        
        while !self.is_target_solved(cube) {
            if count > 2 {
                panic!("Exceeded maximum iterations");
            }
            count += 1;
            
            if let Some(face) = self.has_eyed_corner(cube) {
                self.align_solved_corner(cube, face, &mut steps);
            }
            
            self.execute_corner_permutation_algorithm(cube, &mut steps);
            
            if self.almost_ready(cube) {
                to_fix = true;
                break;
            }
        }
        
        if to_fix {
            let front = FaceOrientation::Front(Color::Blue);
            let color = cube.get_block_color(front.ordinal(), 0, 0);
            
            match color {
                Color::Green => rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, &mut steps),
                Color::Orange => {
                    rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, &mut steps);
                    rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, &mut steps);
                },
                Color::Red => rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), false, &mut steps),
                _ => {}
            }
        }
        
        steps
    }
    
    fn is_target_solved(&self, cube: &Cube) -> bool {
        let up = FaceOrientation::Up(Color::Yellow);
        let up_ordinal = up.ordinal();
        
        // Check all top face colors are yellow
        for i in 0..3 {
            for j in 0..3 {
                if cube.get_block_color(up_ordinal, i, j) != Color::Yellow {
                    return false;
                }
            }
        }
        
        // Check all top corners are in correct position
        for face in [
            FaceOrientation::Front(Color::Blue),
            FaceOrientation::Right(Color::Red),
            FaceOrientation::Back(Color::Orange),
            FaceOrientation::Left(Color::Green)
        ] {
            if cube.get_block_color(face.ordinal(), 0, 0) != face.color() ||
               cube.get_block_color(face.ordinal(), 0, 2) != face.color() {
                return false;
            }
        }
        
        true
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        Some(SolverEnum::TopEdge(TopEdgeSolver {}))
    }
}

