use super::i3::*;
use std::borrow::Cow;

mod clock;
mod shell;
mod florp_blarg;

pub use self::clock::*;
pub use self::shell::*;
pub use self::florp_blarg::*;

/// A type that produces blocks of data.
/// BlockProducer can respond to mouse-events.
pub trait Widget {
    /// Updates the state of the producer and returns the new block data.
    fn update<'a>(&'a mut self) -> Cow<'a, Block>;

    /// Gets the name of the block, if available.
    fn get_name(&self) -> Option<&str> {
        None
    }

    /// Gets the instance name of the block, if available.
    fn get_instance(&self) -> Option<&str> {
        None
    }

    /// Handles a click event.
    fn handle_event(&mut self, event: Button) {}
}

impl Widget for Block {
    fn update<'a>(&'a mut self) -> Cow<'a, Block> {
        Cow::Borrowed(self)
    }
}

impl<T: Into<String> + Clone + Send + 'static> Widget for T {
    fn update(&mut self) -> Cow<'static, Block> {
        Cow::Owned(Block {
            full_text: self.clone().into(),
            ..Block::default()
        })
    }
}
