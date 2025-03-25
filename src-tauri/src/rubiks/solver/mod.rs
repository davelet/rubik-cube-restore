use std::{cell::RefCell, rc::Rc};

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

pub fn execute(cube: &mut Cube, target: SolveTarget) -> (Vec<char>, Cube) {
    if cube.is_solved() {
        return (vec![], cube.clone());
    }

    let mut seq = vec![];
    let first_solver = BottomCrossSolver { cube: cube.clone() };
    let mut solver = SolverEnum::BottomCross(Rc::new(RefCell::new(Box::new(first_solver))));

    loop {
        if !solver.is_target_solved() {
            seq.extend(solver.solve_target());
        }
        if solver.target() == target {
            break;
        }

        let next_solver = solver.next_solver();
        solver = match next_solver {
            Some(s) => s,
            None => break,
        };
    }
    let cube = solver.cube();
    (seq, cube)
}

#[derive(Clone)]
pub enum SolverEnum {
    BottomCross(Rc<RefCell<Box<BottomCrossSolver>>>),
    BottomCorner(Rc<RefCell<Box<BottomCornerSolver>>>),
    MiddleEdge(Rc<RefCell<Box<MiddleSolver>>>),
    TopCross(Rc<RefCell<Box<TopCrossSolver>>>),
    TopFace(Rc<RefCell<Box<TopFaceSolver>>>),
    TopEdge(Rc<RefCell<Box<TopEdgeSolver>>>),
    TopCorner(Rc<RefCell<Box<TopCornerSolver>>>),
}

impl Solver for SolverEnum {
    fn cube(&self) -> Cube {
        match self {
            SolverEnum::BottomCross(ref_cell) => ref_cell.borrow().cube(),
            SolverEnum::BottomCorner(ref_cell) => ref_cell.borrow().cube(),
            SolverEnum::MiddleEdge(ref_cell) => ref_cell.borrow().cube(),
            SolverEnum::TopCross(ref_cell) => ref_cell.borrow().cube(),
            SolverEnum::TopFace(ref_cell) => ref_cell.borrow().cube(),
            SolverEnum::TopEdge(ref_cell) => ref_cell.borrow().cube(),
            SolverEnum::TopCorner(ref_cell) => ref_cell.borrow().cube(),
        }
    }
    fn target(&self) -> SolveTarget {
        match self {
            SolverEnum::BottomCross(ref_cell) => ref_cell.borrow_mut().target(),
            SolverEnum::BottomCorner(ref_cell) => ref_cell.borrow_mut().target(),
            SolverEnum::MiddleEdge(ref_cell) => ref_cell.borrow_mut().target(),
            SolverEnum::TopCross(ref_cell) => ref_cell.borrow_mut().target(),
            SolverEnum::TopFace(ref_cell) => ref_cell.borrow_mut().target(),
            SolverEnum::TopEdge(ref_cell) => ref_cell.borrow_mut().target(),
            SolverEnum::TopCorner(ref_cell) => ref_cell.borrow_mut().target(),
        }
    }

    fn solve_target(&mut self) -> Vec<char> {
        match self {
            SolverEnum::BottomCross(ref_cell) => ref_cell.borrow_mut().solve_target(),
            SolverEnum::BottomCorner(ref_cell) => ref_cell.borrow_mut().solve_target(),
            SolverEnum::MiddleEdge(ref_cell) => ref_cell.borrow_mut().solve_target(),
            SolverEnum::TopCross(ref_cell) => ref_cell.borrow_mut().solve_target(),
            SolverEnum::TopFace(ref_cell) => ref_cell.borrow_mut().solve_target(),
            SolverEnum::TopEdge(ref_cell) => ref_cell.borrow_mut().solve_target(),
            SolverEnum::TopCorner(ref_cell) => ref_cell.borrow_mut().solve_target(),
        }
    }

    fn is_target_solved(&self) -> bool {
        match self {
            SolverEnum::BottomCross(ref_cell) => ref_cell.borrow().is_target_solved(),
            SolverEnum::BottomCorner(ref_cell) => ref_cell.borrow().is_target_solved(),
            SolverEnum::MiddleEdge(ref_cell) => ref_cell.borrow().is_target_solved(),
            SolverEnum::TopCross(ref_cell) => ref_cell.borrow().is_target_solved(),
            SolverEnum::TopFace(ref_cell) => ref_cell.borrow().is_target_solved(),
            SolverEnum::TopEdge(ref_cell) => ref_cell.borrow().is_target_solved(),
            SolverEnum::TopCorner(ref_cell) => ref_cell.borrow().is_target_solved(),
        }
    }

    fn next_solver(&self) -> Option<SolverEnum> {
        match self {
            SolverEnum::BottomCross(ref_cell) => ref_cell.borrow_mut().next_solver(),
            SolverEnum::BottomCorner(ref_cell) => ref_cell.borrow_mut().next_solver(),
            SolverEnum::MiddleEdge(ref_cell) => ref_cell.borrow_mut().next_solver(),
            SolverEnum::TopCross(ref_cell) => ref_cell.borrow_mut().next_solver(),
            SolverEnum::TopFace(ref_cell) => ref_cell.borrow_mut().next_solver(),
            SolverEnum::TopEdge(ref_cell) => ref_cell.borrow_mut().next_solver(),
            SolverEnum::TopCorner(ref_cell) => ref_cell.borrow_mut().next_solver(),
        }
    }
}

pub trait Solver {
    fn target(&self) -> SolveTarget;

    fn solve_target(&mut self) -> Vec<char>;

    fn is_target_solved(&self) -> bool;

    fn next_solver(&self) -> Option<SolverEnum>;

    fn cube(&self) -> Cube;
}

#[cfg(test)]
mod tests {
    use crate::rubiks::shuffler::CubeShuffler;

    use super::*;

    #[test]
    fn test_solve() {
        let mut cube = Cube::new();
        let mut shuffler = CubeShuffler::new(&mut cube);
        shuffler.shuffle(20);
        println!("cube is {:?}", cube);
        let seq = execute(&mut cube, SolveTarget::BottomCross);
        println!("执行完成后的魔方状态: {:?}", cube);
        println!("解决方案序列: {:?}", seq);
    }
}
