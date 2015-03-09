#![feature(io, path, path_ext, core)]

#[macro_use] extern crate log;

pub use cpu::Cpu;
pub use memory::{Ram, Rom};
pub use runner::runner;

mod memory;
mod instructions;
mod cpu;
mod runner;
