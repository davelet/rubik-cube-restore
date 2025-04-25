//! Rubik's Cube implementation module

use rubik_cube_core::cube::{
    face::{Face, TwistDirection},
    Cube,
};
use rubik_cube_shuffler::{CubeScrambler, CubeShuffler};
use rubik_cube_solver::{
    solver::{execute, SolveTarget},
    utils::{color_state_to_u8, u8_to_color_state},
};

#[tauri::command]
pub fn init_get_get_state() -> [[[u8; 3]; 3]; 6] {
    // 将魔方状态转换为u8数组
    color_state_to_u8(&Cube::new())
}

#[tauri::command]
pub fn shuffle(state: [[[u8; 3]; 3]; 6], times: u32) -> [[[u8; 3]; 3]; 6] {
    let mut cube = u8_to_color_state(state);
    let mut shuffler = CubeShuffler::new(&mut cube);
    shuffler.shuffle(times);
    color_state_to_u8(&cube)
}

#[tauri::command]
pub fn turn(state: [[[u8; 3]; 3]; 6], face: u8, direction: bool) -> [[[u8; 3]; 3]; 6] {
    println!("turn param; face: {}, direction: {}", face, direction);
    let mut cube = u8_to_color_state(state);
    let mut shuffler = CubeScrambler::new(&mut cube);
    shuffler.scramble(
        Face::from(face),
        if direction {
            TwistDirection::Clockwise
        } else {
            TwistDirection::CounterClockwise
        },
    );
    color_state_to_u8(&cube)
}

#[tauri::command]
pub fn solve(state: [[[u8; 3]; 3]; 6], target: u8) -> SolveSolution {
    let mut cube = u8_to_color_state(state);
    let target = SolveTarget::from_u8(target);
    let result = execute(&mut cube, target);
    println!("cube after solve: {:?}", cube);
    SolveSolution {
        seq: result.0,
        cube: color_state_to_u8(&result.1),
    }
}

#[derive(serde::Serialize)]
pub struct SolveSolution {
    seq: Vec<char>,
    cube: [[[u8; 3]; 3]; 6],
}

#[cfg(test)]
mod tests {
    use rubik_cube_core::cube::Cube;
    use rubik_cube_shuffler::CubeShuffler;

    use super::*;

    #[test]
    fn test_solve() {
        let mut cube = Cube::new();
        let mut shuffler = CubeShuffler::new(&mut cube);
        shuffler.shuffle(20);
        // print_cube(&cube);
        let res = execute(&mut cube, SolveTarget::TopEdge);
        assert!(res.1.is_solved());
    }
}
