use super::{BlockProducer, Block};

pub struct Ip;

impl BlockProducer for Ip {
	fn update(&mut self) -> Block {
		Block { 
            ..Block::default() 
        }
	}
}