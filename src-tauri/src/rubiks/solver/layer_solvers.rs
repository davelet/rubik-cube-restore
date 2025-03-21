use crate::rubiks::{
    cube::{
        color::Color,
        face::{FaceOrientation, TwistDirection},
        Cube,
    },
    shuffler::CubeScrambler,
};

use super::*;

fn rotate_and_record(
    cube: &mut Cube,
    face: FaceOrientation,
    clockwize_direction: bool,
    steps: &mut Vec<char>,
) {
    let mut shuffler = CubeScrambler::new(cube);
    shuffler.scramble(
        face,
        if clockwize_direction {
            TwistDirection::Clockwise
        } else {
            TwistDirection::CounterClockwise
        },
    );

    let step = match face {
        FaceOrientation::Front(_) => {
            if clockwize_direction {
                'f'
            } else {
                'F'
            }
        }
        FaceOrientation::Back(_) => {
            if clockwize_direction {
                'b'
            } else {
                'B'
            }
        }
        FaceOrientation::Left(_) => {
            if clockwize_direction {
                'l'
            } else {
                'L'
            }
        }
        FaceOrientation::Right(_) => {
            if clockwize_direction {
                'r'
            } else {
                'R'
            }
        }
        FaceOrientation::Up(_) => {
            if clockwize_direction {
                'u'
            } else {
                'U'
            }
        }
        FaceOrientation::Down(_) => {
            if clockwize_direction {
                'd'
            } else {
                'D'
            }
        }
    };
    steps.push(step);
}

pub struct BottomCrossSolver {
    pub cube: Cube,
}

impl Solver for BottomCrossSolver {
    fn target(&self) -> SolveTarget {
        SolveTarget::BottomCross
    }

    fn solve_target(&mut self) -> Vec<char> {
        let mut steps = vec![];

        for f in 2..=5 {
            let face = FaceOrientation::from_u8(f as u8);
            steps.extend(self.solve_edge(face));
        }

        steps
    }

    fn is_target_solved(&self) -> bool {
        let face_colors = self
            .cube
            .get_face_state(FaceOrientation::Down(Color::White).ordinal());
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
            let face_colors = self.cube.state[f];
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
        Some(SolverEnum::BottomCorner(Rc::new(RefCell::new(Box::new(
            BottomCornerSolver {
                cube: self.cube.clone(),
            },
        )))))
    }
}

impl BottomCrossSolver {
    fn solve_edge(&mut self, face: FaceOrientation) -> Vec<char> {
        let mut steps = vec![];
        if self.is_edge_solved(face) {
            return steps;
        }
        if self.find_edge_in_top(face, &mut steps)
            || self.find_edge_in_middle(face, &mut steps)
            || self.find_edge_in_bottom(face, &mut steps)
        {
            // 将边块插入到顶层
            rotate_and_record(&mut self.cube, face, true, &mut steps);
            rotate_and_record(&mut self.cube, face, true, &mut steps);
        }
        steps
    }

    fn is_edge_solved(&self, face: FaceOrientation) -> bool {
        let face_colors = self.cube.get_face_state(face.ordinal());
        let color = face_colors[1][1];
        if face_colors[2][1] != color {
            return false;
        }

        let face_colors = self
            .cube
            .get_face_state(FaceOrientation::Down(Color::White).ordinal());
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

    fn find_edge_in_top(&mut self, face: FaceOrientation, steps: &mut Vec<char>) -> bool {
        // 在顶层寻找目标白色边块
        let face_color = face.color();

        let up_center = self.get_up_center(face);
        let top_face = FaceOrientation::Up(Color::Yellow).ordinal();
        if self
            .cube
            .get_block_color(top_face, up_center.0, up_center.1)
            == Color::White
            && self.cube.get_block_color(face.ordinal(), 0, 1) == face_color
        {
            return true;
        }
        if self.cube.get_block_color(face.ordinal(), 0, 1) == Color::White
            && self
                .cube
                .get_block_color(top_face, up_center.0, up_center.1)
                == face_color
        {
            self.swap_edge_on_top(face, steps);
            return true;
        }

        let right_side = self.get_right_side(face);
        let up_center = self.get_up_center(right_side);
        if self
            .cube
            .get_block_color(top_face, up_center.0, up_center.1)
            == Color::White
            && self.cube.get_block_color(right_side.ordinal(), 0, 1) == face_color
        {
            rotate_and_record(
                &mut self.cube,
                FaceOrientation::Up(Color::Yellow),
                true,
                steps,
            );
            return true;
        }
        if self.cube.get_block_color(right_side.ordinal(), 0, 1) == Color::White
            && self
                .cube
                .get_block_color(top_face, up_center.0, up_center.1)
                == face_color
        {
            rotate_and_record(
                &mut self.cube,
                FaceOrientation::Up(Color::Yellow),
                true,
                steps,
            );
            self.swap_edge_on_top(face, steps);
            return true;
        }

        let left_side = self.get_left_side(face);
        let up_center = self.get_up_center(left_side);
        if self
            .cube
            .get_block_color(top_face, up_center.0, up_center.1)
            == Color::White
            && self.cube.get_block_color(left_side.ordinal(), 0, 1) == face_color
        {
            rotate_and_record(
                &mut self.cube,
                FaceOrientation::Up(Color::Yellow),
                false,
                steps,
            );
            return true;
        }
        if self.cube.get_block_color(left_side.ordinal(), 0, 1) == Color::White
            && self
                .cube
                .get_block_color(top_face, up_center.0, up_center.1)
                == face_color
        {
            rotate_and_record(
                &mut self.cube,
                FaceOrientation::Up(Color::Yellow),
                false,
                steps,
            );
            self.swap_edge_on_top(face, steps);
            return true;
        }

        // Get back side (right of right)
        let back_side = self.get_right_side(right_side);
        let up_center = self.get_up_center(back_side);
        if self
            .cube
            .get_block_color(top_face, up_center.0, up_center.1)
            == Color::White
            && self.cube.get_block_color(back_side.ordinal(), 0, 1) == face_color
        {
            rotate_and_record(
                &mut self.cube,
                FaceOrientation::Up(Color::Yellow),
                false,
                steps,
            );
            rotate_and_record(
                &mut self.cube,
                FaceOrientation::Up(Color::Yellow),
                false,
                steps,
            );
            return true;
        }
        if self.cube.get_block_color(back_side.ordinal(), 0, 1) == Color::White
            && self
                .cube
                .get_block_color(top_face, up_center.0, up_center.1)
                == face_color
        {
            rotate_and_record(
                &mut self.cube,
                FaceOrientation::Up(Color::Yellow),
                false,
                steps,
            );
            rotate_and_record(
                &mut self.cube,
                FaceOrientation::Up(Color::Yellow),
                false,
                steps,
            );
            self.swap_edge_on_top(face, steps);
            return true;
        }

        false
    }

    // Helper methods needed for findEdgeInTop
    fn get_up_center(&self, face: FaceOrientation) -> (usize, usize) {
        match face {
            FaceOrientation::Front(_) => (2, 1),
            FaceOrientation::Right(_) => (1, 2),
            FaceOrientation::Back(_) => (0, 1),
            FaceOrientation::Left(_) => (1, 0),
            _ => panic!("Invalid face orientation for get_up_center: {:?}", face),
        }
    }

    fn get_right_side(&self, face: FaceOrientation) -> FaceOrientation {
        match face {
            FaceOrientation::Front(_) => FaceOrientation::Right(Color::Red),
            FaceOrientation::Right(_) => FaceOrientation::Back(Color::Green),
            FaceOrientation::Back(_) => FaceOrientation::Left(Color::Orange),
            FaceOrientation::Left(_) => FaceOrientation::Front(Color::Blue),
            _ => panic!("Invalid face orientation for get_right_side: {:?}", face),
        }
    }

    fn get_left_side(&self, face: FaceOrientation) -> FaceOrientation {
        match face {
            FaceOrientation::Front(_) => FaceOrientation::Left(Color::Orange),
            FaceOrientation::Left(_) => FaceOrientation::Back(Color::Green),
            FaceOrientation::Back(_) => FaceOrientation::Right(Color::Red),
            FaceOrientation::Right(_) => FaceOrientation::Front(Color::Blue),
            _ => panic!("Invalid face orientation for get_left_side: {:?}", face),
        }
    }

    fn swap_edge_on_top(&mut self, face: FaceOrientation, steps: &mut Vec<char>) {
        let right_face = self.get_right_side(face);
        rotate_and_record(&mut self.cube, face, true, steps);
        rotate_and_record(&mut self.cube, right_face, true, steps);
        rotate_and_record(
            &mut self.cube,
            FaceOrientation::Up(Color::Yellow),
            true,
            steps,
        );
        rotate_and_record(&mut self.cube, right_face, false, steps);
    }

    fn find_edge_in_middle(&self, face: FaceOrientation, steps: &mut Vec<char>) -> bool {
        // TODO: Implement finding edge in middle layer
        false
    }

    fn find_edge_in_bottom(&self, face: FaceOrientation, steps: &mut Vec<char>) -> bool {
        // TODO: Implement finding edge in bottom layer
        false
    }
}

pub struct BottomCornerSolver {
    pub cube: Cube,
}

impl Solver for BottomCornerSolver {
    fn target(&self) -> SolveTarget {
        SolveTarget::BottomCorner
    }

    fn solve_target(&mut self) -> Vec<char> {
        vec![]
    }

    fn is_target_solved(&self) -> bool {
        false
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        Some(SolverEnum::MiddleEdge(Rc::new(RefCell::new(Box::new(
            MiddleSolver {
                cube: self.cube.clone(),
            },
        )))))
    }
}

pub struct MiddleSolver {
    pub cube: Cube,
}

impl Solver for MiddleSolver {
    fn target(&self) -> SolveTarget {
        SolveTarget::MiddleEdge
    }

    fn solve_target(&mut self) -> Vec<char> {
        vec![]
    }

    fn is_target_solved(&self) -> bool {
        false
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        Some(SolverEnum::TopCross(Rc::new(RefCell::new(Box::new(
            TopCrossSolver {
                cube: self.cube.clone(),
            },
        )))))
    }
}

pub struct TopCrossSolver {
    pub cube: Cube,
}

impl Solver for TopCrossSolver {
    fn target(&self) -> SolveTarget {
        SolveTarget::TopCross
    }

    fn solve_target(&mut self) -> Vec<char> {
        vec![]
    }

    fn is_target_solved(&self) -> bool {
        false
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        Some(SolverEnum::TopFace(Rc::new(RefCell::new(Box::new(
            TopFaceSolver {
                cube: self.cube.clone(),
            },
        )))))
    }
}

pub struct TopFaceSolver {
    pub cube: Cube,
}

impl Solver for TopFaceSolver {
    fn target(&self) -> SolveTarget {
        SolveTarget::TopFace
    }

    fn solve_target(&mut self) -> Vec<char> {
        vec![]
    }

    fn is_target_solved(&self) -> bool {
        false
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        Some(SolverEnum::TopEdge(Rc::new(RefCell::new(Box::new(
            TopEdgeSolver {
                cube: self.cube.clone(),
            },
        )))))
    }
}

pub struct TopEdgeSolver {
    pub cube: Cube,
}

impl Solver for TopEdgeSolver {
    fn target(&self) -> SolveTarget {
        SolveTarget::TopEdge
    }

    fn solve_target(&mut self) -> Vec<char> {
        vec![]
    }

    fn is_target_solved(&self) -> bool {
        false
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        Some(SolverEnum::TopCorner(Rc::new(RefCell::new(Box::new(
            TopCornerSolver {
                cube: self.cube.clone(),
            },
        )))))
    }
}

pub struct TopCornerSolver {
    pub cube: Cube,
}

impl Solver for TopCornerSolver {
    fn target(&self) -> SolveTarget {
        SolveTarget::TopCorner
    }

    fn solve_target(&mut self) -> Vec<char> {
        vec![]
    }

    fn is_target_solved(&self) -> bool {
        false
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        None
    }
}
