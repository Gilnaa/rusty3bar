extern crate json;
extern crate time;

use blocks::*;

use std::time::Duration;

pub mod blocks;
pub mod statusline;
pub mod color;

fn main() {
	let mut line = statusline::StatusLine::new();

	// line.add("Simple block");

	line.add(Block {
		full_text: "complicated block".into(),
		foreground_color: Some(color::named::CRIMSON),
		..Default::default()
	});

	line.add(BlockBuilder::new().full_text("Cash me ousside how 'bout dat")
								.foreground_color(color::named::BLACK)
								.background_color(color::named::WHITE)
								.border_color(color::named::RED).finish());

	line.add(Shell::new("echo $(whoami) @ $(hostname)"));

	line.add(Clock::new("%d-%m-%Y").unwrap());

	line.add(Clock::new("%H:%M").unwrap());
	
	line.run(Duration::from_secs(1));
}