//! Rubik's Cube implementation module

use cube::Cube;

mod cube;

#[tauri::command]
pub fn init_get_get_state() -> [[[u8; 3]; 3]; 6] {
    let cube = Cube::new();
    // 将魔方状态转换为u8数组
    let mut result = [[[0u8; 3]; 3]; 6];
    for face in 0..6 {
        for row in 0..3 {
            for col in 0..3 {
                result[face][row][col] = cube.state[face][row][col] as u8;
            }
        }
    }
    // 打印魔方状态数组
    println!("魔方状态数组:");
    for face in 0..6 {
        println!("面 {}:", face);
        for row in 0..3 {
            println!("{:?}", cube.state[face][row]);
        }
    }
    result
}
