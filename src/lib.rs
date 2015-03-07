#![feature(io, path, core)]

pub use cpu::Cpu;
pub use memory::{Ram, Rom};

mod memory;
mod instructions;
mod cpu;
