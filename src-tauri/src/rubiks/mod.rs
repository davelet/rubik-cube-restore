//! Rubik's Cube implementation module

use cube::{
    face::{CubeFace, FaceOrientation, TwistDirection},
    Cube,
};
use shuffler::CubeShuffler;
use utils::*;

mod cube;
mod shuffler;
mod utils;

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
    let mut shuffler = CubeShuffler::new(&mut cube);
    shuffler.rotate_face(
        &CubeFace::new(FaceOrientation::from_u8(face)),
        if direction {
            TwistDirection::Clockwise
        } else {
            TwistDirection::CounterClockwise
        },
    );
    color_state_to_u8(&cube)
}
