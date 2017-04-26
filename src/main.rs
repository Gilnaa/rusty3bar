extern crate time;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use blocks::*;

use std::time::Duration;

pub mod blocks;
pub mod statusline;
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
		}
		else {
			self.data.foreground_color = Some(::color::named::BLACK);
			self.data.background_color = Some(::color::named::CRIMSON);
		}
		self.flag = !self.flag;
		// std::process::Command::new("notify-send").arg(format!("event {:?}", event)).spawn();
	}
}

fn main() {
	let mut line = statusline::StatusLine::new();

	line.add("Simple block");

	line.add(Block {
		full_text: "complicated block".into(),
		foreground_color: Some(color::named::CRIMSON),
		..Default::default()
	});

	line.add(Funky::new());
	
	line.add(BlockBuilder::new().full_text("Cash me ousside how 'bout dat")
								.foreground_color(color::named::BLACK)
								.background_color(color::named::WHITE)
								.border_color(color::named::RED).finish());

	line.add(Shell::new("echo $(whoami) @ $(hostname)"));

	line.add(Clock::new("%d-%m-%Y").unwrap());

	line.add(Clock::new("%H:%M").unwrap());
	
	line.run(Duration::from_secs(1));
}