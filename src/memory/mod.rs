pub const ROM_SIZE: usize = 1024 * 8;
pub const RAM_SIZE: usize = 1024 * 8;

pub type Word = u16;

pub use self::ram::Ram;
pub use self::rom::Rom;

mod ram;
mod rom;
