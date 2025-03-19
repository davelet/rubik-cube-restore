use crate::rubiks::cube::Cube;

use super::SolveTarget;

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
pub enum SolverEnum {
    BottomCross(BottomCrossSolver),
    BottomCorner(BottomCornerSolver),
    MiddleEdge(MiddleSolver),
    TopCross(TopCrossSolver),
    TopFace(TopFaceSolver),
    TopEdge(TopEdgeSolver),
    TopCorner(TopCornerSolver),
}

impl Solver for SolverEnum {
    fn target(&self) -> SolveTarget {
        match_solver_enum!(self, target)
    }

    fn solve_target(&self) -> Vec<String> {
        match_solver_enum!(self, solve_target)
    }

    fn target_solved(&self) -> bool {
        match_solver_enum!(self, target_solved)
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        match_solver_enum!(self, next_solver)
    }
}
pub trait Solver {
    fn target(&self) -> SolveTarget;

    fn solve_target(&self) -> Vec<String>;

    fn target_solved(&self) -> bool;

    fn next_solver(&self) -> Option<SolverEnum>;
}

pub struct BottomCrossSolver {
    pub cube: Cube,
}

impl Solver for BottomCrossSolver {
    fn target(&self) -> SolveTarget {
        SolveTarget::BottomCross
    }

    fn solve_target(&self) -> Vec<String> {
        vec![]
    }

    fn target_solved(&self) -> bool {
        false
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        Some(SolverEnum::BottomCorner(BottomCornerSolver {
            cube: self.cube.clone(),
        }))
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

    fn target_solved(&self) -> bool {
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

    fn target_solved(&self) -> bool {
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

    fn target_solved(&self) -> bool {
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

    fn target_solved(&self) -> bool {
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

    fn target_solved(&self) -> bool {
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

    fn target_solved(&self) -> bool {
        false
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        None
    }
}
