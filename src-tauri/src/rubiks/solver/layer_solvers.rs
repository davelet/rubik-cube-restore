use crate::rubiks::{cube::{color::Color, face::{FaceOrientation, TwistDirection}, Cube}, shuffler::CubeScrambler};

use super::*;

fn rotate(cube: &mut Cube, face: u8, direction: TwistDirection) {
    let orientation = FaceOrientation::from_u8(face);

    let mut shuffler = CubeScrambler::new(cube);
    shuffler.scramble(orientation, direction);
}

pub struct BottomCrossSolver {
    pub cube: Cube,
}

impl Solver for BottomCrossSolver {
    fn target(&self) -> SolveTarget {
        SolveTarget::BottomCross
    }

    fn solve_target(&self) -> Vec<String> {
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
            .get_face_state(FaceOrientation::Down(Color::White).index());
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
        Some(SolverEnum::BottomCorner(BottomCornerSolver {
            cube: self.cube.clone(),
        }))
    }
}

impl BottomCrossSolver {
    fn solve_edge(&self, face: FaceOrientation) -> Vec<String> {
        if self.is_edge_solved(face) {
            return vec![];
        }

        vec![]
    }

    fn is_edge_solved(&self, face: FaceOrientation) -> bool {
        let face_colors = self.cube.get_face_state(face.index());
        let color = face_colors[1][1];
        if face_colors[2][1] != color {
            return false;
        }

        let face_colors = self
            .cube
            .get_face_state(FaceOrientation::Down(Color::White).index());
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
}

pub struct BottomCornerSolver {
    pub cube: Cube,
}

impl Solver for BottomCornerSolver {
    fn target(&self) -> SolveTarget {
        SolveTarget::BottomCorner
    }

    fn solve_target(&self) -> Vec<String> {
        vec![]
    }

    fn is_target_solved(&self) -> bool {
        false
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        Some(SolverEnum::MiddleEdge(MiddleSolver {
            cube: self.cube.clone(),
        }))
    }
}

pub struct MiddleSolver {
    pub cube: Cube,
}

impl Solver for MiddleSolver {
    fn target(&self) -> SolveTarget {
        SolveTarget::MiddleEdge
    }

    fn solve_target(&self) -> Vec<String> {
        vec![]
    }

    fn is_target_solved(&self) -> bool {
        false
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        Some(SolverEnum::TopCross(TopCrossSolver {
            cube: self.cube.clone(),
        }))
    }
}

pub struct TopCrossSolver {
    pub cube: Cube,
}

impl Solver for TopCrossSolver {
    fn target(&self) -> SolveTarget {
        SolveTarget::TopCross
    }

    fn solve_target(&self) -> Vec<String> {
        vec![]
    }

    fn is_target_solved(&self) -> bool {
        false
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        Some(SolverEnum::TopFace(TopFaceSolver {
            cube: self.cube.clone(),
        }))
    }
}

pub struct TopFaceSolver {
    pub cube: Cube,
}

impl Solver for TopFaceSolver {
    fn target(&self) -> SolveTarget {
        SolveTarget::TopFace
    }

    fn solve_target(&self) -> Vec<String> {
        vec![]
    }

    fn is_target_solved(&self) -> bool {
        false
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        Some(SolverEnum::TopEdge(TopEdgeSolver {
            cube: self.cube.clone(),
        }))
    }
}

pub struct TopEdgeSolver {
    pub cube: Cube,
}

impl Solver for TopEdgeSolver {
    fn target(&self) -> SolveTarget {
        SolveTarget::TopEdge
    }

    fn solve_target(&self) -> Vec<String> {
        vec![]
    }

    fn is_target_solved(&self) -> bool {
        false
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        Some(SolverEnum::TopCorner(TopCornerSolver {
            cube: self.cube.clone(),
        }))
    }
}

pub struct TopCornerSolver {
    pub cube: Cube,
}

impl Solver for TopCornerSolver {
    fn target(&self) -> SolveTarget {
        SolveTarget::TopCorner
    }

    fn solve_target(&self) -> Vec<String> {
        vec![]
    }

    fn is_target_solved(&self) -> bool {
        false
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        None
    }
}
