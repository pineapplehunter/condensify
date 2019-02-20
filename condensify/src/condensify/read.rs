use crate::error::Error;
use crate::error::ErrorKind;
use crate::hash::Hash;
use crate::hash::HashType;
use crate::Condensify;
use std::io;
use std::io::Read;

#[derive(Debug)]
pub struct CondensifyFileReader<'a> {
    condensify: &'a Condensify,
    hash_type: HashType,
    hash: Hash,
}

impl CondensifyFileReader {
    pub fn hash_type() -> HashType {
        unimplemented!()
    }

    pub fn get_linked_hashs(&self) -> Result<Vec<Hash>, Error> {
        if HashType::Link != self.hash_type {
            return Err(Error::from(ErrorKind::HashNotALink));
        }
        unimplemented!()
    }
}

impl Read for CondensifyFileReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        unimplemented!()
    }
}
