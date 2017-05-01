use super::blocks::Widget;
use serde_json;
use std::thread;
use std;
use std::time::Duration;

use std::sync::Arc;
use std::sync::Mutex;
use std::sync::Condvar;
use infinite_array::InfiniteArray;

pub struct StatusLine {
    blocks: Arc<Mutex<Vec<Box<Widget + Send + 'static>>>>,
}

impl StatusLine {
    /// Creates a new StatusLine
    pub fn new() -> Self {
        StatusLine { blocks: Arc::new(Mutex::new(Vec::new())) }
    }

    /// Add a new block to the status-line
    pub fn add<T: Widget + Send + 'static>(&mut self, block: T) {
        let mut lock = self.blocks.lock().unwrap();
        lock.push(Box::new(block));
    }

    /// Consumes the status line and start outputing the data blocks.
    pub fn run(self, interval: Duration) {
        // We must print this header, telling i3bar to recieve JSON.
        // Otherwise, anything sent will be interpreted as plain-text.
        println!("{{ \"version\": 1, \"click_events\": true }} [ ");

        let force_refresh = Arc::new(Condvar::new());
        let force_refresh_blocker = force_refresh.clone();

        // Spawn a thread that refreshes the status bar
        let blocks = self.blocks.clone();
        thread::spawn(move || loop {
			let mut lock = blocks.lock().unwrap();
			{
				let array = lock.iter_mut().map(|b| b.update()).collect::<Vec<_>>();
				println!("{},", serde_json::to_string(&array).unwrap());
			}

			force_refresh_blocker
				.wait_timeout(lock, interval)
				.unwrap();
		});

        // Wait for mouse-events from the bar process.
        let stdin = std::io::stdin();

        for event in InfiniteArray::<_, ::i3::I3BarEvent>::new(stdin.lock()) {
            let event = event.unwrap();

            // No support for no instance right now.
            if event.name.is_none() || event.instance.is_none() {
                continue;
            }

            let name = event.name.unwrap();
            let instance = event.instance.unwrap();

            let mut blocks = self.blocks.lock().unwrap();
            let source = blocks
                .iter_mut()
                .find(|block| {
                          block.get_name() == Some(&name) && block.get_instance() == Some(&instance)
                      });

            if let Some(source) = source {
                force_refresh.notify_one();
                source.handle_event(event.button)
            }
        }
    }
}
