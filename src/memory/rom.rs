use std::ops::Deref;
use std::{io, fs, path};
use std::io::{Read, ErrorKind};
use std::num::FromStrRadix;

use memory::{ROM_SIZE, Word};

pub struct Rom([Word; ROM_SIZE]);

impl Rom {
    pub fn from_file<P: path::AsPath>(filename: &P) -> io::Result<Rom> {
        let f = &mut try!(fs::File::open(filename.as_path()));
        let s = &mut String::new();
        try!(f.read_to_string(s));
        Rom::from_str(s)
    }

    pub fn from_str(s: &str) -> io::Result<Rom> {
        let data = s.trim()
                    .lines()
                    // FIXME: Ensure linelength 16
                    .filter(|l| l.len() == 16 && (&l[0..1] != "0" || &l[0..1] != "1"))
                    .map(|l| FromStrRadix::from_str_radix(l, 2).unwrap())
                    .collect::<Vec<Word>>();

        if data.len() > ROM_SIZE {
            return Err(io::Error::new(ErrorKind::Other,
                                      "ROM cannot fit program",
                                      Some(format!("{} is the maximum instruction count", ROM_SIZE))))
        }

        let mut buf = [0; ROM_SIZE];
        for (idx, word) in data.into_iter().enumerate() {
            buf[idx] = word
        }

        Ok(Rom(buf))
    }
}

// Only implement Deref for Rom, so it can be indexed as if it was &[u8]
// This ensures it can never be mutated after load.
impl Deref for Rom {
    type Target = [Word];

    fn deref<'a>(&'a self) -> &'a [Word] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    extern crate tempdir;
    use super::*;
    use std::io;
    use std::io::Write;
    use std::fs::File;
    use self::tempdir::TempDir;

    fn rom_with_data(data: &str) -> io::Result<Rom> {
        let dir = TempDir::new("hack-interpretter").unwrap();
        let dest = dir.path().join("test.hack");

        let f = &mut try!(File::create(&dest));
        try!(f.write_all(data.as_bytes()));

        Rom::from_file(&dest)
    }

    #[test]
    fn loads_raw() {
        let data = "0000000000000000\n\
                    1111111111111111\n\
                    0101010101010101\n\
                    1010101010101010\n\
                    0111111111111111\n";
        let rom = rom_with_data(data).unwrap();

        assert_eq!(&rom[..5], &[0x00,
                               0xFFFF,
                               0b0101010101010101,
                               0b1010101010101010,
                               0b0111111111111111])
    }

    #[test]
    fn assert_size() {
        let rom = rom_with_data("1111111111111111").unwrap();
        assert_eq!(rom.len(), 8192);
        assert_eq!(rom[0], 0xFFFF);

        for &b in &rom[1..] {
            assert_eq!(b, 0)
        }
    }

    #[test]
    fn accept_trailing_blanklines() {
        let rom = rom_with_data("1111000010100101\n\n\n\n").unwrap();
        assert_eq!(rom[0], 0b1111000010100101);
        assert_eq!(rom.len(), 8192);
    }

    #[test]
    fn accept_inline_comments() {
        let rom = rom_with_data("0000000000100000\n\
                                // JMP\n\
                                1000000000000111\n").unwrap();
        assert_eq!(rom[0], 0b0000000000100000);
        assert_eq!(rom[1], 0b1000000000000111);
        assert_eq!(rom.len(), 8192);
    }

    #[test]
    #[should_fail(expected="ROM cannot fit program")]
    fn too_large() {
        let data = (0..20000).map(|_|"0000000000000000\n")
                             .collect::<String>();
        let rom = rom_with_data(&data).unwrap();

        assert_eq!(&rom[..4], &[0x00,
                                0xFFFF,
                                0b0101010101010101,
                                0b1010101010101010])
    }
}
