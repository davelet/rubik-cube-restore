//! Rubik's Cube implementation module

use prelude::*;

pub mod cube;
pub mod shuffler;
pub mod solver;
pub mod utils;

mod prelude {
    pub use super::cube::{
        color::Color,
        face::{FaceOrientation, TwistDirection},
        Cube,
    };
    pub use super::shuffler::*;
    pub use super::solver::{execute, SolveTarget};
    pub use super::utils::*;
}

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
        FaceOrientation::from_u8(face),
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
    let target = solver::SolveTarget::from_u8(target);
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
