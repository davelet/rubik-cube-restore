pub use layer_solvers::*;
use rubik_cube_core::cube::Cube;

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
    TopCorner,
    TopEdge,
}

impl SolveTarget {
    pub fn from_u8(value: u8) -> SolveTarget {
        match value {
            0 => SolveTarget::BottomCross,
            1 => SolveTarget::BottomCorner,
            2 => SolveTarget::MiddleEdge,
            3 => SolveTarget::TopCross,
            4 => SolveTarget::TopFace,
            5 => SolveTarget::TopCorner,
            6 => SolveTarget::TopEdge,
            _ => panic!("Invalid solve target value: {}", value),
        }
    }
}

pub fn execute(cube: &mut Cube, target: SolveTarget) -> (Vec<char>, Cube) {
    if cube.is_solved() {
        return (vec![], cube.clone());
    }

    let mut seq = vec![];
    let first_solver = BottomCrossSolver {};
    let mut solver = SolverEnum::BottomCross(first_solver);

    loop {
        if !solver.is_target_solved(cube) {
            seq.extend(solver.solve_target(cube));
        }
        println!("DONE: current solver is {:?}", solver.target());
        if solver.target() == target {
            println!("reached target  {:?}", target);
            break;
        }

        let next_solver = solver.next_solver();
        solver = match next_solver {
            Some(s) => s,
            None => break,
        };
    }
    (seq, cube.clone())
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
        match self {
            SolverEnum::BottomCross(s) => s.target(),
            SolverEnum::BottomCorner(s) => s.target(),
            SolverEnum::MiddleEdge(s) => s.target(),
            SolverEnum::TopCross(s) => s.target(),
            SolverEnum::TopFace(s) => s.target(),
            SolverEnum::TopEdge(s) => s.target(),
            SolverEnum::TopCorner(s) => s.target(),
        }
    }

    fn solve_target(&mut self, cube: &mut Cube) -> Vec<char> {
        match self {
            SolverEnum::BottomCross(s) => s.solve_target(cube),
            SolverEnum::BottomCorner(s) => s.solve_target(cube),
            SolverEnum::MiddleEdge(s) => s.solve_target(cube),
            SolverEnum::TopCross(s) => s.solve_target(cube),
            SolverEnum::TopFace(s) => s.solve_target(cube),
            SolverEnum::TopEdge(s) => s.solve_target(cube),
            SolverEnum::TopCorner(s) => s.solve_target(cube),
        }
    }

    fn is_target_solved(&self, cube: &Cube) -> bool {
        match self {
            SolverEnum::BottomCross(s) => s.is_target_solved(cube),
            SolverEnum::BottomCorner(s) => s.is_target_solved(cube),
            SolverEnum::MiddleEdge(s) => s.is_target_solved(cube),
            SolverEnum::TopCross(s) => s.is_target_solved(cube),
            SolverEnum::TopFace(s) => s.is_target_solved(cube),
            SolverEnum::TopEdge(s) => s.is_target_solved(cube),
            SolverEnum::TopCorner(s) => s.is_target_solved(cube),
        }
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        match self {
            SolverEnum::BottomCross(s) => s.next_solver(),
            SolverEnum::BottomCorner(s) => s.next_solver(),
            SolverEnum::MiddleEdge(s) => s.next_solver(),
            SolverEnum::TopCross(s) => s.next_solver(),
            SolverEnum::TopFace(s) => s.next_solver(),
            SolverEnum::TopEdge(s) => s.next_solver(),
            SolverEnum::TopCorner(s) => s.next_solver(),
        }
    }
}

pub trait Solver {
    fn target(&self) -> SolveTarget;

    fn solve_target(&mut self, cube: &mut Cube) -> Vec<char>;

    fn is_target_solved(&self, cube: &Cube) -> bool;

    fn next_solver(&self) -> Option<SolverEnum>;
}
