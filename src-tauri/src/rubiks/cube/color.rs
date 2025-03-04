use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    White = 1,
    Yellow = 0,
    Red = 5,
    Orange = 4,
    Blue = 2,
    Green = 3,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Color::White => write!(f, "白"),
            Color::Yellow => write!(f, "黄"),
            Color::Red => write!(f, "红"),
            Color::Orange => write!(f, "橙"),
            Color::Blue => write!(f, "蓝"),
            Color::Green => write!(f, "绿"),
        }
    }
}