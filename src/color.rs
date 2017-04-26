#[derive(Debug, Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8);

impl ToString for Color {
	fn to_string(&self) -> String {
		format!("#{:02x}{:02x}{:02x}", self.0, self.1, self.2)
	}
}

impl ::serde::Serialize for Color {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
		self.to_string().serialize(serializer)
	}
}

pub mod named {
	use super::Color;

	// Pink
	pub static PINK: Color 					= Color(255, 192, 203);
	pub static LIGHT_PINK: Color 			= Color(255, 182, 193);
	pub static HOT_PINK: Color 				= Color(255, 105, 180);
	pub static DEEP_PINK: Color 			= Color(255,  20, 147);
	pub static PALE_VIOLET_RED: Color 		= Color(219, 112, 147);
	pub static MEDIUM_VIOLET_RED: Color 	= Color(199,  21, 133);

	// Red
	pub static LIGHT_SALMON: Color 			= Color(255, 160, 122);
	pub static SALMON: Color 				= Color(250, 128, 114);
	pub static DARK_SALMON: Color 			= Color(233, 150, 122);
	pub static LIGHT_CORAL: Color 			= Color(240, 128, 128);
	pub static INDIAN_RED: Color 			= Color(205,  92,  92);
	pub static CRIMSON: Color 				= Color(220,  20,  60);
	pub static FIRE_BRICK: Color 			= Color(178,  34,  34);
	pub static DARK_RED: Color 				= Color(139,   0,   0);
	pub static RED: Color 					= Color(255,   0,   0);

	// Orange
	pub static ORANGE_RED: Color 			= Color(255,  69,   0);
	pub static TOMATO: Color 				= Color(255,  99,  71);
	pub static CORAL: Color 				= Color(255, 127,  80);
	pub static DARK_ORANGE: Color 			= Color(255, 140,   0);
	pub static ORANGE: Color 				= Color(255, 165,   0);

	// Yellow
	pub static YELLOW: Color				= Color(255, 255,   0);
	pub static LIGHT_YELLOW: Color			= Color(255, 255, 224);
	pub static LEMON_CHIFFON: Color			= Color(255, 250, 205);
	pub static LIGHT_GOLDENROD_YELLOW: Color= Color(250, 250, 210);
	pub static PAPAYA_WHIP: Color			= Color(255, 239, 213);
	pub static MOCCASIN: Color				= Color(255, 228, 181);
	pub static PEACH_PUFF: Color			= Color(255, 218, 185);
	pub static PALE_GOLDENROD: Color		= Color(238, 232, 170);
	pub static KHAKI: Color					= Color(240, 230, 140);
	pub static DARK_KHAKI: Color			= Color(189, 183, 107);
	pub static GOLD: Color					= Color(255, 215,   0);

	// Brown
	pub static CORNSILK: Color 				= Color(255, 248, 220);
	pub static BLANCHED_ALMOND: Color 		= Color(255, 235, 205);
	pub static BISQUE: Color 				= Color(255, 228, 196);
	pub static NAVAJO_WHITE: Color 			= Color(255, 222, 173);
	pub static WHEAT: Color 				= Color(245, 222, 179);
	pub static BURLY_WOOD: Color 			= Color(222, 184, 135);
	pub static TAN: Color 					= Color(210, 180, 140);
	pub static ROSY_BROWN: Color 			= Color(188, 143, 143);
	pub static SANDY_BROWN: Color 			= Color(244, 164,  96);
	pub static GOLDENROD: Color 			= Color(218, 165,  32);
	pub static DARK_GOLDENROD: Color 		= Color(184, 134,  11);
	pub static PERU: Color 					= Color(205, 133,  63);
	pub static CHOCOLATE: Color 			= Color(210, 105,  30);
	pub static SADDLE_BROWN: Color 			= Color(139,  69,  19);
	pub static SIENNA: Color 				= Color(160,  82,  45);
	pub static BROWN: Color 				= Color(165,  42,  42);
	pub static MAROON: Color 				= Color(128,   0,   0);

	// Green
	pub static DARK_OLIVE_GREEN: Color 		= Color( 85, 107,  47);
	pub static OLIVE: Color 				= Color(128, 128,   0);
	pub static OLIVE_DRAB: Color 			= Color(107, 142,  35);
	pub static YELLOW_GREEN: Color 			= Color(154, 205,  50);
	pub static LIME_GREEN: Color 			= Color( 50, 205,  50);
	pub static LIME: Color 					= Color(  0, 255,   0);
	pub static LAWN_GREEN: Color 			= Color(124, 252,   0);
	pub static CHARTREUSE: Color 			= Color(127, 255,   0);
	pub static GREEN_YELLOW: Color 			= Color(173, 255,  47);
	pub static SPRING_GREEN: Color 			= Color(  0, 255, 127);
	pub static MEDIUM_SPRING_GREEN: Color 	= Color(  0, 250, 154);
	pub static LIGHT_GREEN: Color 			= Color(144, 238, 144);
	pub static PALE_GREEN: Color 			= Color(152, 251, 152);
	pub static DARK_SEA_GREEN: Color 		= Color(143, 188, 143);
	pub static MEDIUM_AQUAMARINE: Color 	= Color(102, 205, 170);
	pub static MEDIUM_SEA_GREEN: Color 		= Color( 60, 179, 113);
	pub static SEA_GREEN: Color 			= Color( 46, 139,  87);
	pub static FOREST_GREEN: Color 			= Color( 34, 139,  34);
	pub static GREEN: Color 				= Color(  0, 128,   0);
	pub static DARK_GREEN: Color 			= Color(  0, 100,   0);

	// Cyan
	pub static AQUA: Color 					= Color(  0, 255, 255);
	pub static CYAN: Color 					= Color(  0, 255, 255);
	pub static LIGHT_CYAN: Color 			= Color(224, 255, 255);
	pub static PALE_TURQUOISE: Color 		= Color(175, 238, 238);
	pub static AQUAMARINE: Color 			= Color(127, 255, 212);
	pub static TURQUOISE: Color 			= Color( 64, 224, 208);
	pub static MEDIUM_TURQUOISE: Color 		= Color( 72, 209, 204);
	pub static DARK_TURQUOISE: Color 		= Color(  0, 206, 209);
	pub static LIGHT_SEA_GREEN: Color 		= Color( 32, 178, 170);
	pub static CADET_BLUE: Color 			= Color( 95, 158, 160);
	pub static DARK_CYAN: Color 				= Color(  0, 139, 139);
	pub static TEAL: Color 					= Color(  0, 128, 128);

	// Blue
	pub static LIGHT_STEEL_BLUE: Color 		= Color(176, 196, 222);
	pub static POWDER_BLUE: Color 			= Color(176, 224, 230);
	pub static LIGHT_BLUE: Color 			= Color(173, 216, 230);
	pub static SKY_BLUE: Color 				= Color(135, 206, 235);
	pub static LIGHT_SKY_BLUE: Color 			= Color(135, 206, 250);
	pub static DEEP_SKY_BLUE: Color 			= Color(  0, 191, 255);
	pub static DODGER_BLUE: Color 			= Color( 30, 144, 255);
	pub static CORNFLOWER_BLUE: Color 		= Color(100, 149, 237);
	pub static STEEL_BLUE: Color 			= Color( 70, 130, 180);
	pub static ROYAL_BLUE: Color 			= Color( 65, 105, 225);
	pub static BLUE: Color 					= Color(  0,   0, 255);
	pub static MEDIUM_BLUE: Color 			= Color(  0,   0, 205);
	pub static DARK_BLUE: Color 				= Color(  0,   0, 139);
	pub static NAVY: Color 					= Color(  0,   0, 128);
	pub static MIDNIGHT_BLUE: Color 			= Color( 25,  25, 112);

	// Purple, violet, & magenta 
	pub static LAVENDER: Color 				= Color (230, 230, 250);
	pub static THISTLE: Color 				= Color (216, 191, 216);
	pub static PLUM: Color 					= Color (221, 160, 221);
	pub static VIOLET: Color 				= Color (238, 130, 238);
	pub static ORCHID: Color 				= Color (218, 112, 214);
	pub static FUCHSIA: Color 				= Color (255,   0, 255);
	pub static MAGENTA: Color 				= Color (255,   0, 255);
	pub static MEDIUM_ORCHID: Color 			= Color (186,  85, 211);
	pub static MEDIUM_PURPLE: Color 			= Color (147, 112, 219);
	pub static BLUE_VIOLET: Color 			= Color (138,  43, 226);
	pub static DARK_VIOLET: Color 			= Color (148,   0, 211);
	pub static DARK_ORCHID: Color 			= Color (153,  50, 204);
	pub static DARK_MAGENTA: Color 			= Color (139,   0, 139);
	pub static PURPLE: Color 				= Color (128,   0, 128);
	pub static INDIGO: Color 				= Color ( 75,   0, 130);
	pub static DARK_SLATE_BLUE: Color 		= Color ( 72,  61, 139);
	pub static SLATE_BLUE: Color 			= Color (106,  90, 205);
	pub static MEDIUM_SLATE_BLUE: Color 		= Color (123, 104, 238);

	// White
	pub static WHITE: Color 				= Color(255, 255, 255);
	pub static SNOW: Color 					= Color(255, 250, 250);
	pub static HONEYDEW: Color 				= Color(240, 255, 240);
	pub static MINT_CREAM: Color 			= Color(245, 255, 250);
	pub static AZURE: Color 				= Color(240, 255, 255);
	pub static ALICE_BLUE: Color 			= Color(240, 248, 255);
	pub static GHOST_WHITE: Color 			= Color(248, 248, 255);
	pub static WHITE_SMOKE: Color 			= Color(245, 245, 245);
	pub static SEASHELL: Color 				= Color(255, 245, 238);
	pub static BEIGE: Color 				= Color(245, 245, 220);
	pub static OLD_LACE: Color 				= Color(253, 245, 230);
	pub static FLORAL_WHITE: Color 			= Color(255, 250, 240);
	pub static IVORY: Color 				= Color(255, 255, 240);
	pub static ANTIQUE_WHITE: Color 		= Color(250, 235, 215);
	pub static LINEN: Color 				= Color(250, 240, 230);
	pub static LAVENDER_BLUSH: Color 		= Color(255, 240, 245);
	pub static MISTY_ROSE: Color 			= Color(255, 228, 225);

	// Black
	pub static GAINSBORO: Color 			= Color(220, 220, 220);
	pub static LIGHT_GRAY: Color 			= Color(211, 211, 211);
	pub static SILVER: Color 				= Color(192, 192, 192);
	pub static DARK_GRAY: Color 			= Color(169, 169, 169);
	pub static GRAY: Color 					= Color(128, 128, 128);
	pub static DIM_GRAY: Color 				= Color(105, 105, 105);
	pub static LIGHT_SLATE_GRAY: Color 		= Color(119, 136, 153);
	pub static SLATE_GRAY: Color 			= Color(112, 128, 144);
	pub static DARK_SLATE_GRAY: Color 		= Color( 47,  79,  79);
	pub static BLACK: Color 				= Color(  0,   0,   0);
}