use super::color::Color;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum FaceOrientation {
    Up(Color),
    Down(Color),
    Front(Color),
    Back(Color),
    Left(Color),
    Right(Color),
}

impl FaceOrientation {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => FaceOrientation::Up(Color::Yellow),
            1 => FaceOrientation::Down(Color::White),
            2 => FaceOrientation::Front(Color::Blue),
            3 => FaceOrientation::Back(Color::Green),
            4 => FaceOrientation::Left(Color::Orange),
            5 => FaceOrientation::Right(Color::Red),
            _ => panic!("Invalid face orientation value: {}", value),
        }
    }

    pub fn ordinal(&self) -> usize {
        match self {
            FaceOrientation::Up(_) => 0,
            FaceOrientation::Down(_) => 1,
            FaceOrientation::Front(_) => 2,
            FaceOrientation::Back(_) => 3,
            FaceOrientation::Left(_) => 4,
            FaceOrientation::Right(_) => 5,
        }
    }

    pub fn values() -> Vec<Self> {
        (0..6).map(|i| Self::from_u8(i as u8)).collect()
    }

    pub fn color(&self) -> Color {
        match self {
            FaceOrientation::Up(c) => *c,
            FaceOrientation::Down(c) => *c,
            FaceOrientation::Front(c) => *c,
            FaceOrientation::Back(c) => *c,
            FaceOrientation::Left(c) => *c,
            FaceOrientation::Right(c) => *c,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TwistDirection {
    Clockwise,
    CounterClockwise,
}
