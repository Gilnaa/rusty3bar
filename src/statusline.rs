use super::blocks::{BlockProducer};

use std::thread;

use json;
use json::JsonValue;
use std::time::Duration;

pub struct StatusLine<'a> {
	blocks: Vec<Box<BlockProducer + 'a>>
}

impl<'a> StatusLine<'a> {
	pub fn new() -> Self {
		StatusLine{
			blocks: Vec::new()
		}
	}

	pub fn add<T: BlockProducer + 'a>(&mut self, block: T) {
		self.blocks.push(Box::new(block));
	}

	pub fn run(mut self, interval: Duration) {
		// We must print this in order for i3bar to recieve JSON.
		// Otherwise, anything sent will be interpreted as plain-text.
		println!("{{ \"version\": 1 }} [ ");

		loop {
			let array = JsonValue::Array(self.blocks
											.iter_mut()
											.map(|b| { b.update().to_json().into() })
											.collect());

			println!("{},", json::stringify(array));

			thread::sleep(interval);
		}	
	}
}

pub fn run(blocks: &mut [&mut BlockProducer], interval: Duration) {
	println!("{{ \"version\": 1 }} [ ");

	loop {
		let array = JsonValue::Array(blocks
										.iter_mut()
										.map(|b| { b.update().to_json().into() })
										.collect());

		println!("{},", json::stringify(array));

		thread::sleep(interval);
	}
}