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

pub struct BottomCrossSolver;

impl Solver for BottomCrossSolver {
    fn target(&self) -> SolveTarget {
        SolveTarget::BottomCross
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
        // 在顶层寻找目标白色边块
        let face_color = face.color();

        let up_center = Self::get_up_center(face);
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

        let right_side = Self::get_right_side(face);
        let up_center = Self::get_up_center(right_side);
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

        let left_side = Self::get_left_side(face);
        let up_center = Self::get_up_center(left_side);
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

        // Get back side (right of right)
        let back_side = Self::get_right_side(right_side);
        let up_center = Self::get_up_center(back_side);
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

    // Helper methods needed for findEdgeInTop
    fn get_up_center(face: FaceOrientation) -> (usize, usize) {
        match face {
            FaceOrientation::Front(_) => (2, 1),
            FaceOrientation::Right(_) => (1, 2),
            FaceOrientation::Back(_) => (0, 1),
            FaceOrientation::Left(_) => (1, 0),
            _ => panic!("Invalid face orientation for get_up_center: {:?}", face),
        }
    }

    fn get_right_side(face: FaceOrientation) -> FaceOrientation {
        match face {
            FaceOrientation::Front(_) => FaceOrientation::Right(Color::Red),
            FaceOrientation::Right(_) => FaceOrientation::Back(Color::Green),
            FaceOrientation::Back(_) => FaceOrientation::Left(Color::Orange),
            FaceOrientation::Left(_) => FaceOrientation::Front(Color::Blue),
            _ => panic!("Invalid face orientation for get_right_side: {:?}", face),
        }
    }

    fn get_left_side(face: FaceOrientation) -> FaceOrientation {
        match face {
            FaceOrientation::Front(_) => FaceOrientation::Left(Color::Orange),
            FaceOrientation::Left(_) => FaceOrientation::Back(Color::Green),
            FaceOrientation::Back(_) => FaceOrientation::Right(Color::Red),
            FaceOrientation::Right(_) => FaceOrientation::Front(Color::Blue),
            _ => panic!("Invalid face orientation for get_left_side: {:?}", face),
        }
    }

    fn swap_edge_on_top(cube: &mut Cube, face: FaceOrientation, steps: &mut Vec<char>) {
        let right_face = Self::get_right_side(face);
        rotate_and_record(cube, face, true, steps);
        rotate_and_record(cube, right_face, true, steps);
        rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), true, steps);
        rotate_and_record(cube, right_face, false, steps);
    }

    fn find_edge_in_middle(cube: &mut Cube, face: FaceOrientation, steps: &mut Vec<char>) -> bool {
        let face_color = face.color();

        let left_side = Self::get_left_side(face);
        // 检查左边：白色在相邻面且目标颜色在当前面
        if cube.get_block_color(left_side.ordinal(), 1, 2) == Color::White
            && cube.get_block_color(face.ordinal(), 1, 0) == face_color
        {
            rotate_and_record(cube, face, true, steps);
            return true;
        }
        // 检查左边：白色在当前面且目标颜色在相邻面
        if cube.get_block_color(face.ordinal(), 1, 0) == Color::White
            && cube.get_block_color(left_side.ordinal(), 1, 2) == face_color
        {
            rotate_and_record(cube, left_side, false, steps);
            rotate_and_record(cube, FaceOrientation::Up(Color::Yellow), false, steps);
            rotate_and_record(cube, left_side, true, steps);
            return true;
        }

        let right_side = Self::get_right_side(face);
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

        let back_side = Self::get_right_side(right_side);
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

        // 检查当前面底层的边块
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

        // 检查右侧面
        let right_side = Self::get_right_side(face);
        if Self::check_bottom_edge(cube, face, right_side, steps) {
            return true;
        }

        // 检查左侧面
        let left_side = Self::get_left_side(face);
        if Self::check_bottom_edge(cube, face, left_side, steps) {
            return true;
        }

        // 检查背面（左侧面的左侧面）
        let back_side = Self::get_left_side(left_side);
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

pub struct BottomCornerSolver {}

impl Solver for BottomCornerSolver {
    fn target(&self) -> SolveTarget {
        SolveTarget::BottomCorner
    }

    fn solve_target(&mut self, cube: &mut Cube) -> Vec<char> {
        let prev_solver = BottomCrossSolver {};
        if !prev_solver.is_target_solved(cube) {
            panic!("BottomCrossSolver is not solved");
        }

        // TODO: Implement solving bottom corners
        vec![]
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

pub struct MiddleSolver {}

impl Solver for MiddleSolver {
    fn target(&self) -> SolveTarget {
        SolveTarget::MiddleEdge
    }

    fn solve_target(&mut self, cube: &mut Cube) -> Vec<char> {
        vec![]
    }

    fn is_target_solved(&self, cube: &Cube) -> bool {
        false
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        Some(SolverEnum::TopCross(TopCrossSolver {}))
    }
}

pub struct TopCrossSolver {}

impl Solver for TopCrossSolver {
    fn target(&self) -> SolveTarget {
        SolveTarget::TopCross
    }

    fn solve_target(&mut self, cube: &mut Cube) -> Vec<char> {
        vec![]
    }

    fn is_target_solved(&self, cube: &Cube) -> bool {
        false
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        Some(SolverEnum::TopFace(TopFaceSolver {}))
    }
}

pub struct TopFaceSolver {}

impl Solver for TopFaceSolver {
    fn target(&self) -> SolveTarget {
        SolveTarget::TopFace
    }

    fn solve_target(&mut self, cube: &mut Cube) -> Vec<char> {
        vec![]
    }

    fn is_target_solved(&self, cube: &Cube) -> bool {
        false
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        Some(SolverEnum::TopEdge(TopEdgeSolver {}))
    }
}

pub struct TopEdgeSolver {}

impl Solver for TopEdgeSolver {
    fn target(&self) -> SolveTarget {
        SolveTarget::TopEdge
    }

    fn solve_target(&mut self, cube: &mut Cube) -> Vec<char> {
        vec![]
    }

    fn is_target_solved(&self, cube: &Cube) -> bool {
        false
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        Some(SolverEnum::TopCorner(TopCornerSolver))
    }
}

pub struct TopCornerSolver;

impl Solver for TopCornerSolver {
    fn target(&self) -> SolveTarget {
        SolveTarget::TopCorner
    }

    fn solve_target(&mut self, cube: &mut Cube) -> Vec<char> {
        vec![]
    }

    fn is_target_solved(&self, cube: &Cube) -> bool {
        false
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        None
    }
}
