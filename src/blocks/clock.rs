use super::{BlockProducer, Block};

use std;
use time;
use std::borrow::Cow;

pub struct Clock {
    format: String,
    fallback_settings: Block,
}

impl std::ops::Deref for Clock {
    type Target = Block;

    fn deref(&self) -> &Block {
        &self.fallback_settings
    }
}

impl std::ops::DerefMut for Clock {
    fn deref_mut(&mut self) -> &mut Block {
        &mut self.fallback_settings
    }
}

impl Clock {
    pub fn new(format: &str) -> Result<Clock, ()> {
        Self::new_with_settings(format, Block::default())
    }

    pub fn new_with_settings(format: &str, settings: Block) -> Result<Clock, ()> {
        if let Err(_) = time::strftime(format, &time::now()) {
            Err(())
        } else {
            Ok(Clock {
                   format: format.into(),
                   fallback_settings: settings,
               })
        }
    }
}

impl BlockProducer for Clock {
    fn update<'a>(&'a mut self) -> Cow<'a, Block> {
        self.fallback_settings.full_text = time::strftime(&self.format, &time::now()).unwrap();

        Cow::Borrowed(&self.fallback_settings)
    }
}
