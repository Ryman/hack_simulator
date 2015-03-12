pub struct Parser;

#[derive(Debug, PartialEq)]
pub enum Command { A, C, L }

impl Parser {
    pub fn new(input: &str) -> Parser { unimplemented!() }
    pub fn has_more_commands(&self) -> bool { unimplemented!() }
    pub fn advance(&mut self) { unimplemented!() }
    pub fn command_type(&self) -> Command { unimplemented!() }
    pub fn symbol(&self) -> &'static str { unimplemented!() }
    pub fn dest(&self) -> &'static str { unimplemented!() }
    pub fn comp(&self) -> &'static str { unimplemented!() }
    pub fn jump(&self) -> &'static str { unimplemented!() }
}
