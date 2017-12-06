#![feature(try_from)]

#[macro_use] extern crate nom;

use std::convert::{AsRef, From};
use std::path::Path;
use std::io::Read;
use std::fs::OpenOptions;

use header::Header;

mod header;

#[derive(Debug)]
pub enum Error {
    IoError(::std::io::Error),
    HeaderError(header::Error),
    GenericError,
}

impl From<::std::io::Error> for Error {
    fn from(io_error: ::std::io::Error) -> Error {
        Error::IoError(io_error)
    }
}

impl From<header::Error> for Error {
    fn from(header_error: header::Error) -> Error {
        Error::HeaderError(header_error)
    }
}

pub struct File {
    header: Header,
    entries: Vec<u8>,
}

impl File {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<File, Error> {
        let mut f = OpenOptions::new().read(true).open(path)?;
        File::from_reader(&mut f)
    }

    pub fn from_reader<R: Read>(mut reader: R) -> Result<File, Error> {
        let mut buf = [0u8; 32];
        reader.read_exact(&mut buf)?;
        let h = header::parse(&buf)?;
        let mut rest = vec![];
        reader.read_to_end(&mut rest)?;
        Ok(File {
            header: h,
            entries: rest,
        })
    }

    pub fn filetype(&self) -> &header::FileType {
        &self.header.filetype
    }

    pub fn entry_size(&self) -> u16 {
        self.header.entry_size
    }

    pub fn key_alg(&self) -> &str {
        &self.header.key_alg
    }

    pub fn entry_start(&self, num: usize) -> usize {
        32 + (self.entry_size() as usize) * num
    }

    pub fn entry(&self, num: usize) -> &[u8] {
        let start = self.entry_start(num);
        let end = self.entry_size() as usize;
        &self.entries[start..end]
    }
}
