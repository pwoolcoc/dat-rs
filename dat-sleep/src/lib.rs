//! SLEEP
//!
//! This is an implementation of the SLEEP file format. Details of the foromat can be found at
//! [https://datproject.org/paper](https://datproject.org/paper).
//!
//! # Example
//!
//! ```rust,ignore
//! extern crate sleep;
//!
//! use sleep::Sleep;
//!
//! # fn run() -> Result<(), !> {
//! let signatures = Sleep::open("metadata.signatures").into_inner();
//! # }
//! ```

#![feature(try_from)]

#[macro_use] extern crate nom;

use std::convert::{AsRef, From};
use std::path::Path;
use std::io::Read;
use std::fs::OpenOptions;

use header::Header;
pub use errors::*;

mod header;

mod errors {
    use header;
    use std::convert::From;

    pub type Result<T> = ::std::result::Result<T, Error>;

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
}

pub enum Sleep {
    Signatures(Signatures),
    Bitfield(Bitfield),
    Tree(Tree),
}

impl Sleep {
    pub fn open<P: AsRef<Path>>(path: P) -> Sleep {
        Sleep::Signatures(Signatures)
    }

    pub fn into_inner<S: FromSleep>(self) -> S {
        FromSleep::from_sleep(self)
    }
}

pub trait FromSleep {
    fn from_sleep(sleep: Sleep) -> Self;
}

/// Signatures file
pub struct Signatures;

impl FromSleep for Signatures {
    fn from_sleep(sleep: Sleep) -> Signatures {
        match sleep {
            Sleep::Signatures(s) => s,
            _ => unreachable!{},
        }
    }
}

/// Bitfield file
pub struct Bitfield;

impl FromSleep for Bitfield {
    fn from_sleep(sleep: Sleep) -> Bitfield {
        match sleep {
            Sleep::Bitfield(b) => b,
            _ => unreachable!{},
        }
    }
}


/// Tree file
pub struct Tree;

impl FromSleep for Tree {
    fn from_sleep(sleep: Sleep) -> Tree {
        match sleep {
            Sleep::Tree(t) => t,
            _ => unreachable!{},
        }
    }
}


pub struct File {
    header: Header,
    entries: Vec<u8>,
}

impl File {
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<File> {
        let mut f = OpenOptions::new().read(true).open(path)?;
        File::from_reader(&mut f)
    }

    pub fn from_reader<R: Read>(mut reader: R) -> Result<File> {
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
        (self.entry_size() as usize) * num
    }

    pub fn len(&self) -> usize {
        self.entries.len() / (self.entry_size() as usize)
    }

    pub fn entry(&self, num: usize) -> &[u8] {
        let start = self.entry_start(num);
        let end = self.entry_size() as usize;
        &self.entries[start..end]
    }
}
