//! by dtolnay
//! A struct used to deserialize an infinite array of JSON objects.

use serde::de::{DeserializeOwned};

use serde_json;
use std::io;
use std::marker::PhantomData;

/// A struct used to deserialize an infinite array of JSON objects of type T,
/// coming from stream of type R.
pub struct InfiniteArray<R, T> {
    reader: R,
    skip: Option<u8>,
    marker: PhantomData<T>,
}

impl<R, T> InfiniteArray<R, T> {
    /// Craete a new infinite-array deserializer using the given reader.
    pub fn new(reader: R) -> Self {
        InfiniteArray {
            reader: reader,
            skip: Some(b'['),
            marker: PhantomData,
        }
    }
}

impl<R, T> InfiniteArray<R, T>
    where R: io::Read
{
    fn skip_past_byte(&mut self, byte: u8) -> io::Result<bool> {
        let mut one_byte = [0];
        loop {
            if self.reader.read_exact(&mut one_byte).is_err() {
                return Ok(false);
            }
            if one_byte[0] == byte {
                return Ok(true);
            }
            if !(one_byte[0] as char).is_whitespace() {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, format!("byte {}", one_byte[0])));
            }
        }
    }
}

impl<R, T> Iterator for InfiniteArray<R, T>
    where R: io::Read,
          T: DeserializeOwned
{
    type Item = io::Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(skip) = self.skip {
            match self.skip_past_byte(skip) {
                Ok(true) => {}
                Ok(false) => {
                    return None;
                }
                Err(err) => {
                    return Some(Err(err));
                }
            }
            self.skip = None;
        }
        let de = serde_json::Deserializer::from_reader(&mut self.reader);
        match de.into_iter().next() {
            Some(Ok(v)) => {
                self.skip = Some(b',');
                Some(Ok(v))
            }
            Some(Err(err)) => {
                Some(Err(err.into()))
            }
            None => None,
        }
    }
}