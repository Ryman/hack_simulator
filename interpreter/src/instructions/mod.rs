use std::fmt;
use memory::Word;
use instructions::a::AInstruction;
use instructions::c::CInstruction;

mod a;
mod c;

pub enum Instruction {
    A(AInstruction),
    C(CInstruction)
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Instruction::A(ref ins) => write!(f, "A({:016b})", ins.0),
            Instruction::C(ref ins) => write!(f, "C({:016b})", ins.0)
        }

    }
}

impl Instruction {
    pub fn new(raw: Word) -> Instruction {
        if raw & (1 << 15) == 0 {
            Instruction::A(AInstruction(raw))
        } else {
            Instruction::C(CInstruction(raw))
        }
    }
}
