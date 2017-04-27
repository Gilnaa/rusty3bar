use super::blocks::{BlockProducer};
use serde_json;
use std::thread;
use std;
use std::time::Duration;

use std::sync::Arc;
use std::sync::Mutex;
use ::infinite_array::InfiniteArray;

pub struct StatusLine {
	blocks: Arc<Mutex<Vec<Box<BlockProducer + Send + 'static>>>>
}

impl StatusLine {
	/// Creates a new StatusLine
	pub fn new() -> Self {
		StatusLine{
			blocks: Arc::new(Mutex::new(Vec::new()))
		}
	}

	/// Add a new block to the status-line
	pub fn add<T: BlockProducer + Send + 'static>(&mut self, block: T) {
		let mut lock = self.blocks.lock().unwrap();
		lock.push(Box::new(block));
	}

	/// Consumes the status line and start outputing the data blocks.
	pub fn run(self, interval: Duration) {

		// We must print this in order for i3bar to recieve JSON.
		// Otherwise, anything sent will be interpreted as plain-text.
		println!("{{ \"version\": 1, \"click_events\": true }} [ ");
		
		// Spawn a thread that refreshes the status bar
		let blocks = self.blocks.clone();
		thread::spawn(move || {
			loop {
				let array = blocks.lock().unwrap().iter_mut().map(|b| { b.update() }).collect::<Vec<_>>();
				println!("{},", serde_json::to_string(&array).unwrap());

				thread::sleep(interval);
			}	
		});


		// Wait for mouse-events from the bar process.
		let stdin = std::io::stdin();
		for event in InfiniteArray::<_, ::i3::I3BarEvent>::new(stdin.lock()) {
			let event = event.unwrap();
			
			let mut blocks = self.blocks.lock().unwrap();
			
			let source = blocks.iter_mut().find(|block| block.get_name() == event.name.as_ref().map(|x| &**x ) &&
														block.get_instance() == event.instance.as_ref().map(|x| &**x));
			if let Some(source) = source {
				source.handle_event(event.button)
			}
		}
	}
}