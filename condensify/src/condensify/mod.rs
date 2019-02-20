use std::io::{self, Read, Write};
use std::path::Path;
use std::str::FromStr;

use crate::error::Error;
use crate::error::ErrorKind;
use crate::hash::Hash;
use crate::hash::HashType;

mod read;
mod write;
use read::CondensifyFileReader;
use write::CondensifyFileWriter;

#[derive(Debug)]
pub struct Condensify {
    path: Path,
}

impl Condencify {
    pub fn new(path: Path) -> Self {
        Self { path }
    }

    pub fn new_file(&self) -> CondensifyFileWriter {
        CondensifyFileWriter {
            condensify: self,
            hashes: vec![],
            buf_size: 1024 * 1024,
            tmp: self.path.join("tmp"),
        }
    }

    pub fn get_file(&self, hash: &str) -> Result<CondensifyFileReader, Error> {
        Ok(CondensifyFileReader {
            condensify: self,
            hash_type: HashType::File,
            hash: hash.parse()?,
        })
    }
}
