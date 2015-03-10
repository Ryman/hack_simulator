pub const ROM_SIZE: usize = 1024 * 32;
pub const RAM_SIZE: usize = 24577;

pub type Word = u16;

pub use self::ram::Ram;
pub use self::rom::Rom;

mod ram;
mod rom;
