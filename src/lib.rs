use std::fmt;

#[derive(Copy, Clone)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Reset
}

impl Color {
    pub fn fgcode(self) -> u8 {
        match self {
            Color::Black => 30,
            Color::Red => 31,
            Color::Green => 32,
            Color::Yellow => 33,
            Color::Blue => 34,
            Color::Magenta => 35,
            Color::Cyan => 36,
            Color::White => 37,
            Color::BrightBlack => 90,
            Color::BrightRed => 91,
            Color::BrightGreen => 92,
            Color::BrightYellow => 93,
            Color::BrightBlue => 94,
            Color::BrightMagenta => 95,
            Color::BrightCyan => 96,
            Color::BrightWhite => 97,
            Color::Reset => 39
        }
    }

    pub fn bgcode(self) -> u8 {
        match self {
            Color::Black => 40,
            Color::Red => 41,
            Color::Green => 42,
            Color::Yellow => 43,
            Color::Blue => 44,
            Color::Magenta => 45,
            Color::Cyan => 46,
            Color::White => 47,
            Color::BrightBlack => 100,
            Color::BrightRed => 101,
            Color::BrightGreen => 102,
            Color::BrightYellow => 103,
            Color::BrightBlue => 104,
            Color::BrightMagenta => 105,
            Color::BrightCyan => 106,
            Color::BrightWhite => 107,
            Color::Reset => 49
        }
    }
}

pub enum Draw {
    Background(Color),
    Foreground(Color),
    
    /// The first one is background and the last one is foreground.
    Both(Color, Color)
}

impl fmt::Display for Draw {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Draw::Background(color) => write!(f, "\x1b[{}m", color.bgcode()),
            Draw::Foreground(color) => write!(f, "\x1b[{}m", color.fgcode()),
            Draw::Both(bg, fg) => write!(f, "\x1b[{};{}m", bg.bgcode(), fg.fgcode()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const RED_BACKGROUND: Draw = Draw::Background(Color::Red);
    const GREEN_FOREGROUND: Draw = Draw::Foreground(Color::Green);
    const RESET: Draw = Draw::Both(Color::Reset, Color::Reset);

    #[test]
    fn test_format() {
        assert_eq!(
            format!("{red}Red!{reset}", red = RED_BACKGROUND, reset = RESET),
            "\x1b[41mRed!\x1b[49;39m"
        );

        assert_eq!(
            format!("{green}Green!{reset}", green = GREEN_FOREGROUND, reset = RESET),
            "\x1b[32mGreen!\x1b[49;39m"
        );
    }
}
