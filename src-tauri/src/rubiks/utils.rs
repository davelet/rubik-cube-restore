use super::cube::{color::Color, Cube};

pub fn color_state_to_u8(cube: &Cube) -> [[[u8; 3]; 3]; 6] {
    let color_state: [[[Color; 3]; 3]; 6] = cube.state;
    let mut result = [[[0u8; 3]; 3]; 6];
    for face in 0..6 {
        for row in 0..3 {
            for col in 0..3 {
                result[face][row][col] = color_state[face][row][col] as u8;
            }
        }
    }
    println!("魔方状态数组:");
    for face in 0..6 {
        println!("面 {}:", face);
        for row in 0..3 {
            println!("{:?}", cube.state[face][row]);
        }
    }
    result
}

pub fn u8_to_color_state(state: [[[u8; 3]; 3]; 6]) -> Cube {
    let mut color_state = [[[Color::White; 3]; 3]; 6];
    for face in 0..6 {
        for row in 0..3 {
            for col in 0..3 {
                color_state[face][row][col] = Color::from_u8(state[face][row][col]);
            }
        }
    }
    Cube::from_state(color_state)
}
