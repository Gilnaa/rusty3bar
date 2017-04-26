mod clock;
mod ip;
mod shell;

pub use self::clock::*;
pub use self::ip::*;
pub use self::shell::*;

use super::color::*;

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(from="usize")] 
pub enum Button {
	Left = 1,
	Middle,
	Right,
	ScrollUp,
	ScrollDown
}

impl From<usize> for Button {
	fn from(raw: usize) -> Button {
		match raw {
            1 => Button::Left,
            2 => Button::Middle,
            3 => Button::Right,
			4 => Button::ScrollUp,
			5 => Button::ScrollDown,
            _ => unimplemented!(),
        }
	}
}

pub trait BlockProducer {
	fn update(&mut self) -> Block;

	fn get_name(&self) -> Option<&str> { None }
	fn get_instance(&self) -> Option<&str> { None }
	fn handle_event(&mut self, event: Button) { }
}

#[derive(Debug, Clone, Serialize)]
pub enum Padding {
	PixelSize(usize),
	StringSize(String)
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")] 
pub enum Alignement {
	Left,
	Right,
	Center
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")] 
pub enum MarkupType {
	None,
	Pango
}

#[derive(Debug, Clone, Serialize)]
pub struct Block {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub instance: Option<String>,

	pub full_text: String,

	#[serde(skip_serializing_if = "Option::is_none")]
	pub short_text: Option<String>,

	#[serde(skip_serializing_if = "Option::is_none", rename = "color")]
	pub foreground_color: Option<Color>,

	#[serde(skip_serializing_if = "Option::is_none", rename = "background")]
	pub background_color: Option<Color>,
		
	#[serde(skip_serializing_if = "Option::is_none", rename = "border")]
	pub border_color: Option<Color>,
	
	#[serde(skip_serializing_if = "Option::is_none")]
	pub min_width: Option<Padding>,

	pub align: Alignement,

	#[serde(rename = "urgent")]
	pub is_urgent: bool,

	pub separator: bool,
	
	#[serde(rename = "separator_block_width")]
	pub separator_width: usize,

	#[serde(rename = "markup")]
	pub markup_type: MarkupType
}

impl Default for Block {
	fn default() -> Self {
		Block {
			name: None,
			instance: None,
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