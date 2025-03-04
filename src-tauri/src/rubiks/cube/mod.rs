//! Cube module

use color::Color;
use face::{Face, FaceOrientation};

pub mod color;
pub mod face;

pub struct Cube {
    pub state: [[[Color; 3]; 3]; 6],
}

impl Cube {
    pub fn new() -> Cube {
        let mut faces = Vec::new();
        for i in 0..6 {
            let face = match i {
                0 => Face::new(FaceOrientation::Up),
                1 => Face::new(FaceOrientation::Down),
                2 => Face::new(FaceOrientation::Front),
                3 => Face::new(FaceOrientation::Back),
                4 => Face::new(FaceOrientation::Left),
                5 => Face::new(FaceOrientation::Right),
                _ => panic!(),
            };
            faces.push((face, i));
        }

        let mut state = [[[Color::White; 3]; 3]; 6];
        
        // 通过faces初始化每个面的颜色
        for face in &faces {
            for i in 0..3 {
                for j in 0..3 {
                    state[face.1 as usize][i][j] = face.0.get_color();
                }
            }
        }
        
        Cube { state }
    }
}