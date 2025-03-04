use super::color::Color;

pub struct Face {
    color: Color,
}

impl Face {
    pub fn new(orientation: FaceOrientation) -> Self {
        let color = match orientation {
            FaceOrientation::Up => Color::Yellow,
            FaceOrientation::Down => Color::White,
            FaceOrientation::Front => Color::Blue,
            FaceOrientation::Back => Color::Green,
            FaceOrientation::Left => Color::Orange,
            FaceOrientation::Right => Color::Red,
        };
        Face { color }
    }

    pub fn get_color(&self) -> Color {
        self.color
    }
}

pub enum FaceOrientation {
    Up,
    Down,
    Front,
    Back,
    Left,
    Right,
}
