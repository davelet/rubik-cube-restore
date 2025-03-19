use layer_solvers::{BottomCrossSolver, Solver as _, SolverEnum};

use super::cube::Cube;

mod layer_solvers;

/// 求解分分步骤
/// 1. 底层十字
/// 2. 底层角块
/// 3. 中间层（只有棱块需要还原）
/// 4. 顶层十字
/// 5. 顶层黄面
/// 6. 顶层棱块
/// 7. 顶层角块
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SolveTarget {
    BottomCross,
    BottomCorner,
    MiddleEdge,
    TopCross,
    TopFace,
    TopEdge,
    TopCorner,
}

impl SolveTarget {
    pub fn from_u8(value: u8) -> SolveTarget {
        match value {
            0 => SolveTarget::BottomCross,
            1 => SolveTarget::BottomCorner,
            2 => SolveTarget::MiddleEdge,
            3 => SolveTarget::TopCross,
            4 => SolveTarget::TopFace,
            5 => SolveTarget::TopEdge,
            6 => SolveTarget::TopCorner,
            _ => panic!("Invalid solve target value: {}", value),
        }
    }
}

pub struct Solver {
    cube: Cube,
    target: SolveTarget,
}

impl Solver {
    pub fn new(cube: Cube, target: SolveTarget) -> Solver {
        Solver { cube, target }
    }

    pub fn solve(&self) -> Vec<String> {
        if self.cube.is_solved() {
            return vec![];
        }
        let mut seq = vec![];
        let first_solver = BottomCrossSolver {
            cube: self.cube.clone(),
        };
        let mut solver = SolverEnum::BottomCross(first_solver);
        loop {
            let target = solver.target();
            if target == self.target {
                seq.extend(solver.solve_target());
                break;
            }

            let next_solver = solver.next_solver();
            match next_solver {
                Some(s) => {
                    solver = s;
                }
                None => break,
            }
        }
        seq
    }
}
