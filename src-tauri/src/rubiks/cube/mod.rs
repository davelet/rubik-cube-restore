//! Cube module

use color::Color;
use face::{CubeFace, FaceOrientation};

pub mod color;
pub mod face;

#[derive(Debug, Clone)]
pub struct Cube {
    pub state: [[[Color; 3]; 3]; 6],
}

impl Cube {
    pub fn new() -> Cube {
        let mut faces = Vec::new();
        for i in 0..6 {
            let face = match i {
                0 => CubeFace::new(FaceOrientation::Up),
                1 => CubeFace::new(FaceOrientation::Down),
                2 => CubeFace::new(FaceOrientation::Front),
                3 => CubeFace::new(FaceOrientation::Back),
                4 => CubeFace::new(FaceOrientation::Left),
                5 => CubeFace::new(FaceOrientation::Right),
                _ => panic!(),
            };
            faces.push((face, i));
        }

        let mut state = [[[Color::White; 3]; 3]; 6];

        // 通过faces初始化每个面的颜色
        for face in &faces {
            for i in 0..3 {
                for j in 0..3 {
                    state[face.1 as usize][i][j] = face.0.color;
                }
            }
        }

        Cube { state }
    }

    pub fn from_state(state: [[[Color; 3]; 3]; 6]) -> Cube {
        Cube { state }
    }

    pub fn is_solved(&self) -> bool {
        for face in self.state {
            let color = face[1][1];
            for i in 0..3 {
                for j in 0..3 {
                    if i == 1 && j == 1 {
                        continue;
                    }
                    if face[i][j] != color {
                        return false;
                    }
                }
            }
        }
        true
    }
    
    pub fn get_color(&self, face: usize, i: usize, j: usize) -> Color {
        self.state[face][i][j]
    }

    pub fn set_color(&mut self, face: usize, i: usize, j: usize, color: Color) {
        self.state[face][i][j] = color;
    }
}
