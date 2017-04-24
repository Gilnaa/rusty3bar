use json;

mod clock;
mod ip;
mod shell;

pub use self::clock::*;
pub use self::ip::*;
pub use self::shell::*;

use super::color::*;

pub trait BlockProducer {
	fn update(&mut self) -> Block;
	// fn is_event_associated(&self, name: String) -> bool;
	// fn handle_event(&mut self) -> bool;
}

#[derive(Debug, Clone)]
pub enum Padding {
	PixelSize(usize),
	StringSize(String)
}

#[derive(Debug, Clone)]
pub enum Alignement {
	Left,
	Right,
	Center
}

#[derive(Debug, Clone)]
pub enum MarkupType {
	None,
	Pango
}

#[derive(Debug, Clone)]
pub struct Block {
	pub full_text: String,
	pub short_text: Option<String>,
	pub foreground_color: Option<Color>,
	pub background_color: Option<Color>,
	pub border_color: Option<Color>,
	pub min_width: Option<Padding>,
	pub align: Alignement,

	pub is_urgent: bool,
	pub separator: bool,
	pub separator_width: usize,

	pub markup_type: MarkupType
}

impl ToString for Padding {
	fn to_string(&self) -> String {
		match self {
			&Padding::PixelSize(size) => size.to_string(),
			&Padding::StringSize(ref text) => text.clone()
		}
	}
}


impl ToString for Alignement {
    fn to_string(&self) -> String {
    	format!("{:?}", self)
    }
}

impl Block {
	pub fn to_json(self) -> json::object::Object {
		let mut obj = json::object::Object::new();
		
		obj["full_text"] = self.full_text.into();

		if let Some(short_text) = self.short_text {
			obj["short_text"] = short_text.into();
		}

		if let Some(color) = self.foreground_color {
			obj["color"] = color.to_string().into();
		}

		if let Some(color) = self.background_color {
			obj["background"] = color.to_string().into();
		}

		if let Some(color) = self.border_color {
			obj["border"] = color.to_string().into();
		}

		if let Some(w) = self.min_width{
			obj["min_width"] = match w {
				Padding::PixelSize(size) => size.into(),
				Padding::StringSize(text) => text.into(),
			}
		}

		obj["align"] = format!("{:?}", self.align).into();
		// Alignement

		if self.is_urgent {
			obj["urgent"] = json::JsonValue::Boolean(true);
		}

		obj["separator"] =json::JsonValue::Boolean(self.separator);
		if self.separator {
			obj["separator_block_width"] = self.separator_width.into();
		}

		obj["markup"] = match self.markup_type {
			MarkupType::None => "none".into(),
		    MarkupType::Pango => "pango".into()
		};

		obj
	}
}

impl Default for Block {
	fn default() -> Self {
		Block {
			full_text: "".into(),
			short_text: None,

			foreground_color: None,
			background_color: None,
			border_color: None,

			min_width: None,
			align: Alignement::Left,
			is_urgent: false,
			separator: true,
			separator_width: 9,
			markup_type: MarkupType::None
		}
	}
}

pub struct BlockBuilder {
	data: Block
}

impl Into<Block> for BlockBuilder {
	fn into(self) -> Block {
		self.data
	}
}

impl BlockBuilder {
	pub fn new() -> BlockBuilder {
		BlockBuilder { data: Block::default() }
	}

	pub fn finish(self) -> Block {
		self.data
	}

	pub fn full_text<T: Into<String>>(mut self, text: T) -> Self {
		self.data.full_text = text.into();
		self
	}

	pub fn short_text<T: Into<String>>(mut self, text: T) -> Self {
		self.data.short_text = Some(text.into());
		self
	}

	pub fn foreground_color(mut self, color: Color) -> Self {
		self.data.foreground_color = Some(color);
		self
	}

	pub fn background_color(mut self, color: Color) -> Self {
		self.data.background_color = Some(color);
		self
	}

	pub fn border_color(mut self, color: Color) -> Self {
		self.data.border_color = Some(color);
		self
	}

	pub fn min_width(mut self, min_width: Padding) -> Self {
		self.data.min_width = Some(min_width);
		self
	}

	pub fn align(mut self, align: Alignement) -> Self {
		self.data.align = align;
		self
	}

	pub fn is_urgent(mut self, is_urgent: bool) -> Self {
		self.data.is_urgent = is_urgent;
		self
	}

	pub fn separator(mut self, separator: bool) -> Self {
		self.data.separator = separator;
		self
	}

	pub fn separator_width(mut self, width: usize) -> Self {
		self.data.separator_width = width;
		self
	}

	pub fn markup_type(mut self, markup: MarkupType) -> Self {
		self.data.markup_type = markup;
		self
	}
}

impl BlockProducer for Block {
	fn update(&mut self) -> Block {
		self.clone()
	}
}

impl<T: Into<String> + Clone> BlockProducer for T {
	fn update(&mut self) -> Block {
		Block {
			full_text: self.clone().into(),
			..Block::default()
		}
	}	
}