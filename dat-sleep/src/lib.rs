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
extern crate ed25519_dalek as ed25519;

use std::convert::{AsRef, From, TryFrom};
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
        Ed25519SignatureError(String),
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
        let filetype = *file.filetype();
        Ok(match filetype {
            FileType::Signatures => {
                Sleep::Signatures(Signatures::try_from(file)?)
            },
            FileType::Bitfield => {
                Sleep::Bitfield(Bitfield::from_file(file)?)
            },
            FileType::Tree => {
                Sleep::Tree(Tree::from_file(file)?)
            },
        })
    }

    pub fn into_inner<S: From<Sleep>>(self) -> S {
        From::from(self)
    }
}

/// Signatures file
pub struct Signatures {
    pub sigs: Vec<ed25519::Signature>,
    file: File,
}

impl TryFrom<File> for Signatures {
    type Error = Error;

    fn try_from(file: File) -> Result<Signatures> {
        Ok(Signatures {
            sigs: file.entry_iter().map(|entry| {
                          ed25519::Signature::from_bytes(entry)
                                             .map_err(|s| Error::Ed25519SignatureError(s.into()))
            }).collect::<Result<Vec<_>>>()?,
            file: file
        })
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
    fn from_file(file: File) -> Result<Bitfield> {
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
    fn from_file(file: File) -> Result<Tree> {
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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_signatures_file() {
        let f = Sleep::open("./dat-files/metadata.signatures").expect("ERROR").into_inner::<Signatures>();

    }
}
