use std::str::FromStr;
use thiserror::Error;

pub enum NamedColor {
    Black,
    DarkBlue,
    DarkGreen,
    DarkAqua,
    DarkRed,
    DarkPurple,
    Gold,
    Gray,
    DarkGray,
    Blue,
    Green,
    Aqua,
    Red,
    LightPurple,
    Yellow,
    White,
}



#[derive(Debug, Error)]
pub enum NamedColorParseError {
    #[error("unknown legacy color code `{0}`")]
    UnknownChar(char),
    #[error("unknown rgb color `{0}`")]
    UnknownRGB(u32),
    #[error("unknown color name `{0}`")]
    UnknownName(String),
}

impl TryFrom<char> for NamedColor {
    type Error = NamedColorParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '0' => NamedColor::Black,
            '1' => NamedColor::DarkBlue,
            '2' => NamedColor::DarkGreen,
            '3' => NamedColor::DarkAqua,
            '4' => NamedColor::DarkRed,
            '5' => NamedColor::DarkPurple,
            '6' => NamedColor::Gold,
            '7' => NamedColor::Gray,
            '8' => NamedColor::DarkGray,
            '9' => NamedColor::Blue,
            'a' => NamedColor::Green,
            'b' => NamedColor::Aqua,
            'c' => NamedColor::Red,
            'd' => NamedColor::LightPurple,
            'e' => NamedColor::Yellow,
            'f' => NamedColor::White,
            _ => return Err(NamedColorParseError::UnknownChar(value)),
        })
    }
}

impl Into<char> for NamedColor {
    fn into(self) -> char {
        match self {
            NamedColor::Black => '0',
            NamedColor::DarkBlue => '1',
            NamedColor::DarkGreen => '2',
            NamedColor::DarkAqua => '3',
            NamedColor::DarkRed => '4',
            NamedColor::DarkPurple => '5',
            NamedColor::Gold => '6',
            NamedColor::Gray => '7',
            NamedColor::DarkGray => '8',
            NamedColor::Blue => '9',
            NamedColor::Green => 'a',
            NamedColor::Aqua => 'b',
            NamedColor::Red => 'c',
            NamedColor::LightPurple => 'd',
            NamedColor::Yellow => 'e',
            NamedColor::White => 'f',
        }
    }
}

impl TryFrom<u32> for NamedColor {
    type Error = NamedColorParseError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(match value {
            0x000000 => NamedColor::Black,
            0x0000aa => NamedColor::DarkBlue,
            0x00aa00 => NamedColor::DarkGreen,
            0x00aaaa => NamedColor::DarkAqua,
            0xaa0000 => NamedColor::DarkRed,
            0xaa00aa => NamedColor::DarkPurple,
            0xffaa00 => NamedColor::Gold,
            0xaaaaaa => NamedColor::Gray,
            0x555555 => NamedColor::DarkGray,
            0x5555ff => NamedColor::Blue,
            0x55ff55 => NamedColor::Green,
            0x55ffff => NamedColor::Aqua,
            0xff5555 => NamedColor::Red,
            0xff55ff => NamedColor::LightPurple,
            0xffff55 => NamedColor::Yellow,
            0xffffff => NamedColor::White,
            _ => return Err(NamedColorParseError::UnknownRGB(value)),
        })
    }
}

impl Into<u32> for NamedColor {
    fn into(self) -> u32 {
        match self {
            NamedColor::Black => 0x000000,
            NamedColor::DarkBlue => 0x0000aa,
            NamedColor::DarkGreen => 0x00aa00,
            NamedColor::DarkAqua => 0x00aaaa,
            NamedColor::DarkRed => 0xaa0000,
            NamedColor::DarkPurple => 0xaa00aa,
            NamedColor::Gold => 0xffaa00,
            NamedColor::Gray => 0xaaaaaa,
            NamedColor::DarkGray => 0x555555,
            NamedColor::Blue => 0x5555ff,
            NamedColor::Green => 0x55ff55,
            NamedColor::Aqua => 0x55ffff,
            NamedColor::Red => 0xff5555,
            NamedColor::LightPurple => 0xff55ff,
            NamedColor::Yellow => 0xffff55,
            NamedColor::White => 0xffffff,
        }
    }
}

impl FromStr for NamedColor {
    type Err = NamedColorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "BLACK" => NamedColor::Black,
            "DARK_BLUE" => NamedColor::DarkBlue,
            "DARK_GREEN" => NamedColor::DarkGreen,
            "DARK_AQUA" => NamedColor::DarkAqua,
            "DARK_RED" => NamedColor::DarkRed,
            "DARK_PURPLE" => NamedColor::DarkPurple,
            "GOLD" => NamedColor::Gold,
            "GRAY" => NamedColor::Gray,
            "DARK_GRAY" => NamedColor::DarkGray,
            "BLUE" => NamedColor::Blue,
            "GREEN" => NamedColor::Green,
            "AQUA" => NamedColor::Aqua,
            "RED" => NamedColor::Red,
            "LIGHT_PURPLE" => NamedColor::LightPurple,
            "YELLOW" => NamedColor::Yellow,
            "WHITE" => NamedColor::White,
            _ => return Err(NamedColorParseError::UnknownName(s.to_string())),
        })
    }
}

impl Into<&'static str> for NamedColor {
    fn into(self) -> &'static str {
        match self {
            NamedColor::Black => "BLACK",
            NamedColor::DarkBlue => "DARK_BLUE",
            NamedColor::DarkGreen => "DARK_GREEN",
            NamedColor::DarkAqua => "DARK_AQUA",
            NamedColor::DarkRed => "DARK_RED",
            NamedColor::DarkPurple => "DARK_PURPLE",
            NamedColor::Gold => "GOLD",
            NamedColor::Gray => "GRAY",
            NamedColor::DarkGray => "DARK_GRAY",
            NamedColor::Blue => "BLUE",
            NamedColor::Green => "GREEN",
            NamedColor::Aqua => "AQUA",
            NamedColor::Red => "RED",
            NamedColor::LightPurple => "LIGHT_PURPLE",
            NamedColor::Yellow => "YELLOW",
            NamedColor::White => "WHITE",
        }
    }
}