use std::str::FromStr;
use std::ascii::AsciiExt;
use std::hash::{Hash, Hasher};
use serde::de::{Deserialize, Deserializer};
use std::fmt::{Display, Formatter, Error};

/// A simple RGB color representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color(pub u8, pub u8, pub u8);

impl Color {
    /// Constructs a color from its components.
    pub fn from_parts(r: u8, g: u8, b: u8) -> Self {
        Color(r, g, b)
    }

    /// Constructs a color from its color name.
    pub fn from_name(name: &str) -> Option<Color> {
        COLORS.get(&CaseInsensitiveString(name)).map(|c| **c)
    }
}

impl ::serde::Serialize for Color {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: ::serde::Serializer
    {
        format!("#{:02x}{:02x}{:02x}", self.0, self.1, self.2).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Color {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?; 
        Color::from_str(&s).map_err(::serde::de::Error::custom)
    }
}

impl FromStr for Color {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Color, &'static str> {
        if s.starts_with("#") {
            if s.len() != "#RRGGBB".len() {
                Err("Invalid length for hex color string")
            }
            else {
                let r = u8::from_str_radix(&s[1..3], 16).map_err(|_| "Failed to parse # color component.")?;
                let g = u8::from_str_radix(&s[3..5], 16).map_err(|_| "Failed to parse # color component.")?;
                let b = u8::from_str_radix(&s[5..7], 16).map_err(|_| "Failed to parse # color component.")?;

                Ok(Color::from_parts(r, g, b))
            }
        }
        else {
            Color::from_name(s).ok_or("Unrecognized color name")
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        f.write_str(&format!("#{:02x}{:02x}{:02x}", self.0, self.1, self.2))
    }
}

pub struct CaseInsensitiveString<'a>(pub &'a str);

impl<'a> ::std::hash::Hash for CaseInsensitiveString<'a> {
    fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
        for ch in self.0.chars() {
            for upper_ch in ch.to_uppercase() {
                state.write_u32(upper_ch as u32);
            }
        }
    }
}

impl<'a> PartialEq for CaseInsensitiveString<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(other.0)
    }
}

impl<'a> Eq for CaseInsensitiveString<'a> { }

/// Creates a set of named constants of a ceratin types, along with a mapping of name => value.
macro_rules! named_map {
    { map $col_name:ident<$const_type:ident> = { $($name:ident => $data:expr),+ } } => {
        use std::collections::HashMap;
        
        $(pub static $name: $const_type = $data;)+

        lazy_static! {
            pub static ref $col_name: HashMap<CaseInsensitiveString<'static>, &'static Color> = {
                let mut map = HashMap::new();
                $(map.insert(CaseInsensitiveString(stringify!($name)), &$name);)+
                map
            };
        }
    }
}

named_map! {
    map COLORS<Color> = {
        // Pink
        PINK => Color(255, 192, 203),
        LIGHT_PINK => Color(255, 182, 193),
        HOT_PINK => Color(255, 105, 180),
        DEEP_PINK => Color(255, 20, 147),
        PALE_VIOLET_RED => Color(219, 112, 147),
        MEDIUM_VIOLET_RED => Color(199, 21, 133),

        // Red
        LIGHT_SALMON => Color(255, 160, 122),
        SALMON => Color(250, 128, 114),
        DARK_SALMON => Color(233, 150, 122),
        LIGHT_CORAL => Color(240, 128, 128),
        INDIAN_RED => Color(205, 92, 92),
        CRIMSON => Color(220, 20, 60),
        FIRE_BRICK => Color(178, 34, 34),
        DARK_RED => Color(139, 0, 0),
        RED => Color(255, 0, 0),

        // Orange
        ORANGE_RED => Color(255, 69, 0),
        TOMATO => Color(255, 99, 71),
        CORAL => Color(255, 127, 80),
        DARK_ORANGE => Color(255, 140, 0),
        ORANGE => Color(255, 165, 0),

        // Yellow
        YELLOW => Color(255, 255, 0),
        LIGHT_YELLOW => Color(255, 255, 224),
        LEMON_CHIFFON => Color(255, 250, 205),
        LIGHT_GOLDENROD_YELLOW => Color(250, 250, 210),
        PAPAYA_WHIP => Color(255, 239, 213),
        MOCCASIN => Color(255, 228, 181),
        PEACH_PUFF => Color(255, 218, 185),
        PALE_GOLDENROD => Color(238, 232, 170),
        KHAKI => Color(240, 230, 140),
        DARK_KHAKI => Color(189, 183, 107),
        GOLD => Color(255, 215, 0),

        // Brown
        CORNSILK => Color(255, 248, 220),
        BLANCHED_ALMOND => Color(255, 235, 205),
        BISQUE => Color(255, 228, 196),
        NAVAJO_WHITE => Color(255, 222, 173),
        WHEAT => Color(245, 222, 179),
        BURLY_WOOD => Color(222, 184, 135),
        TAN => Color(210, 180, 140),
        ROSY_BROWN => Color(188, 143, 143),
        SANDY_BROWN => Color(244, 164, 96),
        GOLDENROD => Color(218, 165, 32),
        DARK_GOLDENROD => Color(184, 134, 11),
        PERU => Color(205, 133, 63),
        CHOCOLATE => Color(210, 105, 30),
        SADDLE_BROWN => Color(139, 69, 19),
        SIENNA => Color(160, 82, 45),
        BROWN => Color(165, 42, 42),
        MAROON => Color(128, 0, 0),

        // Green
        DARK_OLIVE_GREEN => Color(85, 107, 47),
        OLIVE => Color(128, 128, 0),
        OLIVE_DRAB => Color(107, 142, 35),
        YELLOW_GREEN => Color(154, 205, 50),
        LIME_GREEN => Color(50, 205, 50),
        LIME => Color(0, 255, 0),
        LAWN_GREEN => Color(124, 252, 0),
        CHARTREUSE => Color(127, 255, 0),
        GREEN_YELLOW => Color(173, 255, 47),
        SPRING_GREEN => Color(0, 255, 127),
        MEDIUM_SPRING_GREEN => Color(0, 250, 154),
        LIGHT_GREEN => Color(144, 238, 144),
        PALE_GREEN => Color(152, 251, 152),
        DARK_SEA_GREEN => Color(143, 188, 143),
        MEDIUM_AQUAMARINE => Color(102, 205, 170),
        MEDIUM_SEA_GREEN => Color(60, 179, 113),
        SEA_GREEN => Color(46, 139, 87),
        FOREST_GREEN => Color(34, 139, 34),
        GREEN => Color(0, 128, 0),
        DARK_GREEN => Color(0, 100, 0),

        // Cyan
        AQUA => Color(0, 255, 255),
        CYAN => Color(0, 255, 255),
        LIGHT_CYAN => Color(224, 255, 255),
        PALE_TURQUOISE => Color(175, 238, 238),
        AQUAMARINE => Color(127, 255, 212),
        TURQUOISE => Color(64, 224, 208),
        MEDIUM_TURQUOISE => Color(72, 209, 204),
        DARK_TURQUOISE => Color(0, 206, 209),
        LIGHT_SEA_GREEN => Color(32, 178, 170),
        CADET_BLUE => Color(95, 158, 160),
        DARK_CYAN => Color(0, 139, 139),
        TEAL => Color(0, 128, 128),

        // Blue
        LIGHT_STEEL_BLUE => Color(176, 196, 222),
        POWDER_BLUE => Color(176, 224, 230),
        LIGHT_BLUE => Color(173, 216, 230),
        SKY_BLUE => Color(135, 206, 235),
        LIGHT_SKY_BLUE => Color(135, 206, 250),
        DEEP_SKY_BLUE => Color(0, 191, 255),
        DODGER_BLUE => Color(30, 144, 255),
        CORNFLOWER_BLUE => Color(100, 149, 237),
        STEEL_BLUE => Color(70, 130, 180),
        ROYAL_BLUE => Color(65, 105, 225),
        BLUE => Color(0, 0, 255),
        MEDIUM_BLUE => Color(0, 0, 205),
        DARK_BLUE => Color(0, 0, 139),
        NAVY => Color(0, 0, 128),
        MIDNIGHT_BLUE => Color(25, 25, 112),

        // Purple, violet, & magenta
        LAVENDER => Color(230, 230, 250),
        THISTLE => Color(216, 191, 216),
        PLUM => Color(221, 160, 221),
        VIOLET => Color(238, 130, 238),
        ORCHID => Color(218, 112, 214),
        FUCHSIA => Color(255, 0, 255),
        MAGENTA => Color(255, 0, 255),
        MEDIUM_ORCHID => Color(186, 85, 211),
        MEDIUM_PURPLE => Color(147, 112, 219),
        BLUE_VIOLET => Color(138, 43, 226),
        DARK_VIOLET => Color(148, 0, 211),
        DARK_ORCHID => Color(153, 50, 204),
        DARK_MAGENTA => Color(139, 0, 139),
        PURPLE => Color(128, 0, 128),
        INDIGO => Color(75, 0, 130),
        DARK_SLATE_BLUE => Color(72, 61, 139),
        SLATE_BLUE => Color(106, 90, 205),
        MEDIUM_SLATE_BLUE => Color(123, 104, 238),

        // White
        WHITE => Color(255, 255, 255),
        SNOW => Color(255, 250, 250),
        HONEYDEW => Color(240, 255, 240),
        MINT_CREAM => Color(245, 255, 250),
        AZURE => Color(240, 255, 255),
        ALICE_BLUE => Color(240, 248, 255),
        GHOST_WHITE => Color(248, 248, 255),
        WHITE_SMOKE => Color(245, 245, 245),
        SEASHELL => Color(255, 245, 238),
        BEIGE => Color(245, 245, 220),
        OLD_LACE => Color(253, 245, 230),
        FLORAL_WHITE => Color(255, 250, 240),
        IVORY => Color(255, 255, 240),
        ANTIQUE_WHITE => Color(250, 235, 215),
        LINEN => Color(250, 240, 230),
        LAVENDER_BLUSH => Color(255, 240, 245),
        MISTY_ROSE => Color(255, 228, 225),

        // Black
        GAINSBORO => Color(220, 220, 220),
        LIGHT_GRAY => Color(211, 211, 211),
        SILVER => Color(192, 192, 192),
        DARK_GRAY => Color(169, 169, 169),
        GRAY => Color(128, 128, 128),
        DIM_GRAY => Color(105, 105, 105),
        LIGHT_SLATE_GRAY => Color(119, 136, 153),
        SLATE_GRAY => Color(112, 128, 144),
        DARK_SLATE_GRAY => Color(47, 79, 79),
        BLACK => Color(0, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_parse_name() {
        assert_eq!(Color::from_name("BLACK"), Some(BLACK));
        assert_eq!(Color::from_name("BLaCk"), Some(BLACK));
        assert_eq!(Color::from_name("black"), Some(BLACK));
        assert_eq!(Color::from_name("b l a c     k"), None);
    }

    #[test]
    fn color_parse_hex() {
        assert_eq!(Color::from_str("#00"), Err(()));
        assert_eq!(Color::from_str("#000000"), Ok(BLACK));
        assert_eq!(Color::from_str("#00FF13"), Ok(Color(0, 0xFF, 0x13)));
    }
}