use std::ops::Deref;
use std::{io, fs};
use std::path::Path;
use std::io::{Read, ErrorKind};
use std::fmt::{self, Display};
use std::error::Error;

use memory::{ROM_SIZE, Word};

// FIXME: Unnecessary after rust#23979
#[derive(Debug)]
struct StringError(String);

impl Error for StringError {
    fn description(&self) -> &str { &self.0 }
}

impl Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}
// end remove after rust#23979

pub struct Rom(Vec<Word>);

impl Rom {
    pub fn from_file<P: AsRef<Path>>(filename: &P) -> io::Result<Rom> {
        let f = &mut try!(fs::File::open(filename.as_ref()));
        let s = &mut String::new();
        try!(f.read_to_string(s));
        Rom::from_str(s)
    }

    pub fn from_str(s: &str) -> io::Result<Rom> {
        let mut buf = s.trim()
                      .lines()
                      .filter(|l| l.len() == 16 && (&l[0..1] != "0" || &l[0..1] != "1"))
                      .map(|l| u16::from_str_radix(l, 2).unwrap())
                      .collect::<Vec<_>>();

        let instructions = buf.len();

        if instructions > ROM_SIZE {
            return Err(io::Error::new(ErrorKind::Other,
                                      StringError(format!("ROM cannot fit program: {} is \
                                                           the maximum instruction count",
                                                            ROM_SIZE))))
        }

        // Zero the remaining buffer
        buf.extend((0..ROM_SIZE - instructions).map(|_| 0));
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
    use memory::ROM_SIZE;
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
        assert_eq!(rom.len(), ROM_SIZE);
        assert_eq!(rom[0], 0xFFFF);

        for &b in &rom[1..] {
            assert_eq!(b, 0)
        }
    }

    #[test]
    fn accept_trailing_blanklines() {
        let rom = rom_with_data("1111000010100101\n\n\n\n").unwrap();
        assert_eq!(rom[0], 0b1111000010100101);
        assert_eq!(rom.len(), ROM_SIZE);
    }

    #[test]
    fn accept_inline_comments() {
        let rom = rom_with_data("0000000000100000\n\
                                // JMP\n\
                                1000000000000111\n").unwrap();
        assert_eq!(rom[0], 0b0000000000100000);
        assert_eq!(rom[1], 0b1000000000000111);
        assert_eq!(rom.len(), ROM_SIZE);
    }

    #[test]
    #[should_panic(expected="ROM cannot fit program")]
    fn too_large() {
        let data = (0..ROM_SIZE+1).map(|_|"0000000000000000\n")
                             .collect::<String>();
        let rom = rom_with_data(&data).unwrap();

        assert_eq!(&rom[..4], &[0x00,
                                0xFFFF,
                                0b0101010101010101,
                                0b1010101010101010])
    }
}
