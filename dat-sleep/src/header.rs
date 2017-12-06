use std::str;
use std::convert::TryFrom;

use nom::{be_u8, be_u16, rest, IResult};

/// Enum of possible errors
#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    IncorrectFileType,
    InvalidVersion,
    ParseError,
}

/// type of SLEEP file
#[derive(Debug, Clone, PartialEq)]
pub enum FileType {
    Bitfield,
    Signatures,
    Tree,
}

impl TryFrom<u8> for FileType {
    type Error = Error;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        Ok(match byte {
            0u8 => FileType::Bitfield,
            1 => FileType::Signatures,
            2 => FileType::Tree,
            _ => return Err(Error::IncorrectFileType),
        })
    }
}

/// Version of the SLEEP header format.
/// 0 is the only currently available version
#[derive(Debug, Clone, PartialEq)]
pub enum Version {
    V0,
}

impl TryFrom<u8> for Version {
    type Error = Error;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        Ok(match byte {
            0 => Version::V0,
            _ => return Err(Error::InvalidVersion),
        })
    }
}

/// SLEEP Header structure
#[derive(Debug, Clone, PartialEq)]
pub struct Header {
    pub filetype: FileType,
    pub version: Version,
    pub entry_size: u16,
    pub key_alg: String,
}

pub fn parse(bytes: &[u8]) -> Result<Header, Error> {
    match _parse_header(&bytes[..32]) {
        IResult::Done(_, h) => Ok(h),
        _ => Err(Error::ParseError),
    }
}

named!(_parse_header<Header>, do_parse!(
            tag!(&[5, 2, 87])                                           >>
            filetype: map_res!(be_u8, TryFrom::try_from)                >>
            version: map_res!(be_u8, TryFrom::try_from)                 >>
            entry_size: be_u16                                          >>
            key_alg: length_value!(be_u8, map_res!(rest, str::from_utf8))  >>
            rest                                                        >>
            (Header {
                filetype: filetype,
                version: version,
                entry_size: entry_size,
                key_alg: key_alg.into(),
            })
));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_header() {
        let data = [
            5u8, 2, 87, 1,  // magic byte sequence. `1` means it's a .signatures file
            0,              // version of the header format
            0, 64,          // be_16 indicating the block size
            7,              // length of the string describing the key algorithm
                            // remainder is the string describing the key algorithm
            69, 100, 50, 53, 53, 49, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let header = parse(&data).expect("Could not parse header");
        assert_eq!(
                Header {
                    filetype: FileType::Signatures,
                    version: Version::V0,
                    entry_size: 64,
                    key_alg: "Ed25519".into(),
                },
                header);
    }
}
