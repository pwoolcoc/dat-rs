use std::convert::AsRef;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::Read;

use errors::Result;
use header::{self, FileType, Header};

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

    pub fn filetype(&self) -> &FileType {
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
        let end = (self.entry_size() as usize) + start;
        &self.entries[start..end]
    }

    pub fn entry_iter<'a>(&'a self) -> EntryIter<'a> {
        EntryIter {
            file: self,
            curr: 0,
        }
    }
}

pub struct EntryIter<'a> {
    file: &'a File,
    curr: usize,
}

impl<'a> Iterator for EntryIter<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr == self.file.len() {
            None
        } else {
            let idx = self.curr;
            self.curr += 1;
            Some(self.file.entry(idx))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_entries() {
        let f = File::from_path("./dat-files/metadata.signatures").expect("Could not open file");
        let entries = f.entry_iter().collect::<Vec<_>>();
        println!("{:?}", entries);
    }
}
