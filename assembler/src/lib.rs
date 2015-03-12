pub use code::Code;
pub use parser::{Command, Parser};
pub use symbol_table::SymbolTable;

mod code;
mod parser;
mod symbol_table;

pub fn assemble(s: &str) -> Result<String, String> {
    unimplemented!()
}
