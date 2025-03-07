use super::color::Color;

pub struct CubeFace {
    pub color: Color,
    pub orientation: FaceOrientation,
}

impl CubeFace {
    pub fn new(orientation: FaceOrientation) -> Self {
        let color = match orientation {
            FaceOrientation::Up => Color::Yellow,
            FaceOrientation::Down => Color::White,
            FaceOrientation::Front => Color::Blue,
            FaceOrientation::Back => Color::Green,
            FaceOrientation::Left => Color::Orange,
            FaceOrientation::Right => Color::Red,
        };
        CubeFace { color, orientation }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum FaceOrientation {
    Up,
    Down,
    Front,
    Back,
    Left,
    Right,
}

impl FaceOrientation {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => FaceOrientation::Up,
            1 => FaceOrientation::Down,
            2 => FaceOrientation::Front,
            3 => FaceOrientation::Back,
            4 => FaceOrientation::Left,
            5 => FaceOrientation::Right,
            _ => panic!("Invalid face orientation value: {}", value),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TwistDirection {
    Clockwise,
    CounterClockwise,
}