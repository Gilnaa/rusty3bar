use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use super::blocks::{BlockProducer};
use serde_json;
use std::thread;
use std;
use std::io::{Read, BufRead};
use std::time::Duration;

use std::sync::Arc;
use std::sync::Mutex;

pub struct StatusLine {
	blocks: Arc<Mutex<Vec<Box<BlockProducer + Send + 'static>>>>
}

#[derive(Deserialize, Clone, Debug)]
struct I3BarEvent {
	pub name: Option<String>,
	pub instance: Option<String>,
	pub button: ::blocks::Button,
	pub x: usize,
	pub y: usize
}

impl StatusLine {
	pub fn new() -> Self {
		StatusLine{
			blocks: Arc::new(Mutex::new(Vec::new()))
		}
	}

	pub fn add<T: BlockProducer + Send + 'static>(&mut self, block: T) {
		let mut lock = self.blocks.lock().unwrap();
		lock.push(Box::new(block));
	}

	pub fn run(mut self, interval: Duration) {
		// We must print this in order for i3bar to recieve JSON.
		// Otherwise, anything sent will be interpreted as plain-text.
		println!("{{ \"version\": 1, \"click_events\": true }} [ ");
		
		let blocks = self.blocks.clone();
		thread::spawn(move || {
			loop {
				
				{
					let mut blocks = blocks.lock().unwrap();
					let array = blocks.iter_mut().map(|b| { b.update() }).collect::<Vec<_>>();
					println!("{},", serde_json::to_string(&array).unwrap());
				}

				thread::sleep(interval);
			}	
		});

		let mut log = File::create("/home/gilnaa/rlog.txt").unwrap();

		
		let stdin = std::io::stdin();
		let mut lock = stdin.lock();

		for event in ::infinite_array::InfiniteArray::<_, I3BarEvent>::new(lock) {
			// let s = match event {
			// 	Ok(evt) => format!("'{:?}'-'{:?}': {}", evt.name, evt.instance, evt.button),
			// 	_ => format!("err")
			// };
			// std::process::Command::new("notify-send").arg(s).spawn();

			if let Ok(event) = event {
				writeln!(log, "1 {:?}", event);
				let mut blocks = self.blocks.lock().unwrap();
				writeln!(log, "2 {:?}", event);
				
				let source = blocks.iter_mut().find(|block| block.get_name() == event.name.as_ref().map(|x| &**x ) &&
														block.get_instance() == event.instance.as_ref().map(|x| &**x));
				if let Some(source) = source {
					source.handle_event(event.button)
				}
			}
			else {
				// std::process::Command::new("notify-send").arg("Error").spawn();
			}
		}
	}
}

// let mut log = File::create("/home/gilnaa/rlog.txt").unwrap();