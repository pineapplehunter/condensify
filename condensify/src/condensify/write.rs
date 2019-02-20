use crate::hash::Hash;
use crate::Condensify;
use std::io;
use std::io::Write;
use std::path::Path;

#[derive(Debug)]
pub struct CondensifyFileWriter<'a> {
    condensify: &'a Condensify,
    hashes: Vec<Hash>,
    buf_size: u64,
    tmp: Path,
}

impl Write for CondensifyFileWriter {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        unimplemented!()
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        unimplemented!()
    }
}

impl CondensifyFileWriter {
    fn get_tmp_hash() -> Hash {
        unimplemented!()
    }

    pub fn save() -> Hash {
        unimplemented!()
    }
}
