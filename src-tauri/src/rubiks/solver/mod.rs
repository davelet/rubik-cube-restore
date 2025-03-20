use layer_solvers::*;

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

pub struct Executor {
    cube: Cube,
    target: SolveTarget,
}

impl Executor {
    pub fn new(cube: Cube, target: SolveTarget) -> Executor {
        Executor { cube, target }
    }

    pub fn execute(&self) -> Vec<String> {
        if self.cube.is_solved() {
            return vec![];
        }

        let mut seq = vec![];
        let first_solver = BottomCrossSolver {
            cube: self.cube.clone(),
        };
        let mut solver = SolverEnum::BottomCross(first_solver);

        loop {
            if !solver.is_target_solved() {
                seq.extend(solver.solve_target());
            }
            if solver.target() == self.target {
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

pub enum SolverEnum {
    BottomCross(BottomCrossSolver),
    BottomCorner(BottomCornerSolver),
    MiddleEdge(MiddleSolver),
    TopCross(TopCrossSolver),
    TopFace(TopFaceSolver),
    TopEdge(TopEdgeSolver),
    TopCorner(TopCornerSolver),
}

macro_rules! match_solver_enum {
    ($self:ident, $method:ident) => {
        match $self {
            SolverEnum::BottomCross(solver) => solver.$method(),
            SolverEnum::BottomCorner(solver) => solver.$method(),
            SolverEnum::MiddleEdge(solver) => solver.$method(),
            SolverEnum::TopCross(solver) => solver.$method(),
            SolverEnum::TopFace(solver) => solver.$method(),
            SolverEnum::TopEdge(solver) => solver.$method(),
            SolverEnum::TopCorner(solver) => solver.$method(),
        }
    };
}

impl Solver for SolverEnum {
    fn target(&self) -> SolveTarget {
        match_solver_enum!(self, target)
    }

    fn solve_target(&self) -> Vec<String> {
        match_solver_enum!(self, solve_target)
    }

    fn is_target_solved(&self) -> bool {
        match_solver_enum!(self, is_target_solved)
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        match_solver_enum!(self, next_solver)
    }
}

pub trait Solver {
    fn target(&self) -> SolveTarget;

    fn solve_target(&self) -> Vec<String>;

    fn is_target_solved(&self) -> bool;

    fn next_solver(&self) -> Option<SolverEnum>;
}
