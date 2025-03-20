//! Cube module

use color::Color;
use face::FaceOrientation;

pub mod color;
pub mod face;

#[derive(Debug, Clone)]
pub struct Cube {
    pub state: [[[Color; 3]; 3]; 6],
}

impl Cube {
    pub fn new() -> Cube {
        let mut state = [[[Color::White; 3]; 3]; 6];

        for face in FaceOrientation::values() {
            for i in 0..3 {
                for j in 0..3 {
                    state[face.index()][i][j] = face.color();
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

    pub fn get_block_color(&self, face: usize, i: usize, j: usize) -> Color {
        self.state[face][i][j]
    }

    pub fn set_block_color(&mut self, face: usize, i: usize, j: usize, color: Color) {
        self.state[face][i][j] = color;
    }

    pub fn get_face_state(&self, face: usize) -> [[Color; 3]; 3] {
        self.state[face]
    }
}
