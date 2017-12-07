//! SLEEP
//!
//! This is an implementation of the SLEEP file format. Details of the foromat can be found at
//! [https://datproject.org/paper](https://datproject.org/paper).
//!
//! # Example
//!
//! ```rust,no_run
//! use sleep::{Signatures, Sleep, Error};
//!
//! # fn run() -> Result<(), Error> {
//! let sigs: Signatures = Sleep::open("metadata.signatures")?.into_inner();
//! #   Ok(())
//! # }
//! ```

#![feature(try_from)]

#[macro_use] extern crate nom;

use std::convert::{AsRef, From};
use std::path::Path;

use header::FileType;
pub use file::File;
pub use errors::*;

mod header;
mod file;

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
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Sleep> {
        let file = File::from_path(path)?;
        Ok(match file.filetype() {
            &FileType::Signatures => {
                Sleep::Signatures(Signatures::from_file(&file)?)
            },
            &FileType::Bitfield => {
                Sleep::Bitfield(Bitfield::from_file(&file)?)
            },
            &FileType::Tree => {
                Sleep::Tree(Tree::from_file(&file)?)
            },
        })
    }

    pub fn into_inner<S: From<Sleep>>(self) -> S {
        From::from(self)
    }
}

/// Signatures file
pub struct Signatures;

impl Signatures {
    fn from_file(file: &File) -> Result<Signatures> {
        Err(Error::GenericError)
    }
}

impl From<Sleep> for Signatures {
    fn from(sleep: Sleep) -> Signatures {
        match sleep {
            Sleep::Signatures(s) => s,
            _ => unreachable!{},
        }
    }
}

/// Bitfield file
pub struct Bitfield;

impl Bitfield {
    fn from_file(file: &File) -> Result<Bitfield> {
        Err(Error::GenericError)
    }
}

impl From<Sleep> for Bitfield {
    fn from(sleep: Sleep) -> Bitfield {
        match sleep {
            Sleep::Bitfield(b) => b,
            _ => unreachable!{},
        }
    }
}

/// Tree file
pub struct Tree;

impl Tree {
    fn from_file(file: &File) -> Result<Tree> {
        Err(Error::GenericError)
    }
}

impl From<Sleep> for Tree {
    fn from(sleep: Sleep) -> Tree {
        match sleep {
            Sleep::Tree(t) => t,
            _ => unreachable!{},
        }
    }
}


