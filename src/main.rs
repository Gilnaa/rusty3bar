#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

extern crate time;

use blocks::*;
use i3::*;
use std::time::Duration;

mod i3;
mod blocks;
mod statusline;
pub mod color;
mod infinite_array;

struct Funky {
	data: Block,
	flag: bool
}

impl Funky {
	pub fn new() -> Self {
		Funky {
			flag: false,
			data: Block{
				name: Some("funky".into()),
				instance: Some("funky".into()),
				full_text: "BLARG".into(),
				foreground_color: Some(::color::named::CRIMSON),
				background_color: Some(::color::named::BLACK),
				..Default::default()
			}
		}
	}
}

impl BlockProducer for Funky {
	fn update(&mut self) -> Block {
		self.data.clone()
	}

	fn get_name(&self) -> Option<&str> { 
		Some("funky")
	}

	fn get_instance(&self) -> Option<&str> { 
		Some("funky")
	}

	fn handle_event(&mut self, event: Button) { 
		if self.flag {
			self.data.foreground_color = Some(::color::named::CRIMSON);
			self.data.background_color = Some(::color::named::BLACK);
			self.data.full_text = "BLARG".into();
		}
		else {
			self.data.foreground_color = Some(::color::named::BLACK);
			self.data.background_color = Some(::color::named::CRIMSON);
			self.data.full_text = "FLORP".into();
			
		}
		self.flag = !self.flag;
	}
}

fn main() {
	let mut line = statusline::StatusLine::new();

	line.add("Simple block");

	line.add(Block {
		full_text: "<b>E</b> = M&#169;<sup>2</sup>".into(),
		foreground_color: Some(color::named::CRIMSON),
		markup_type: MarkupType::Pango,
		..Default::default()
	});

	line.add(Funky::new());

	// line.add(BlockBuilder::new().full_text("Cash me ousside how 'bout dat")
	// 							.foreground_color(color::named::BLACK)
	// 							.background_color(color::named::WHITE)
	// 							.border_color(color::named::RED).finish());

	line.add(Shell::new("echo $(whoami) @ $(hostname)"));

	line.add(Clock::new("%d-%m-%Y").unwrap());

	line.add(Clock::new("%H:%M").unwrap());
	
	line.run(Duration::from_secs(1));
}