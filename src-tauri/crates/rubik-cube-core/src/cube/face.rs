use super::color::Color;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
#[repr(u8)]
enum FaceOrientation {
    Up(Color),
    Down(Color),
    Front(Color),
    Back(Color),
    Left(Color),
    Right(Color),
}
impl From<u8> for FaceOrientation {
    fn from(value: u8) -> Self {
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
}
impl FaceOrientation {
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
        (0..6).map(|i| Self::from(i as u8)).collect()
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

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Face {
    Up,
    Down,
    Front,
    Back,
    Left,
    Right,
}

impl From<&Face> for FaceOrientation {
    fn from(face: &Face) -> Self {
        match face {
            Face::Up => FaceOrientation::Up(Color::Yellow),
            Face::Down => FaceOrientation::Down(Color::White),
            Face::Front => FaceOrientation::Front(Color::Blue),
            Face::Right => FaceOrientation::Right(Color::Red),
            Face::Back => FaceOrientation::Back(Color::Green),
            Face::Left => FaceOrientation::Left(Color::Orange),
        }
    }
}

impl From<&FaceOrientation> for Face {
    fn from(orientation: &FaceOrientation) -> Self {
        match orientation {
            FaceOrientation::Up(_) => Face::Up,
            FaceOrientation::Down(_) => Face::Down,
            FaceOrientation::Front(_) => Face::Front,
            FaceOrientation::Back(_) => Face::Back,
            FaceOrientation::Left(_) => Face::Left,
            FaceOrientation::Right(_) => Face::Right,
        }
    }
}

impl Face {
    pub fn color(&self) -> Color {
        let orientation: FaceOrientation = self.into();
        orientation.color()
    }

    pub fn ordinal(&self) -> usize {
        let orientation: FaceOrientation = self.into();
        orientation.ordinal()
    }

    pub fn values() -> Vec<Self> {
        FaceOrientation::values().iter().map(|f| f.into()).collect()
    }
}

impl From<u8> for Face {
    fn from(value: u8) -> Self {
        (&FaceOrientation::from(value)).into()
    }
}
