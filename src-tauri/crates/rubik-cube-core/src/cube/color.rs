
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Yellow = 0,
    White = 1,
    Blue = 2,
    Green = 3,
    Orange = 4,
    Red = 5,
}

// impl fmt::Display for Color {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(
//             f,
//             "{}",
//             match self {
//                 Color::Yellow => "黄",
//                 Color::White => "白",
//                 Color::Blue => "蓝",
//                 Color::Green => "绿",
//                 Color::Orange => "橙",
//                 Color::Red => "红",
//             }
//         )
//     }
// }

impl Color {
    pub fn from_u8(value: u8) -> Color {
        match value {
            0..=5 => unsafe { std::mem::transmute(value) },
            _ => panic!("Invalid color value: {}", value),
        }
    }
}
