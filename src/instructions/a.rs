use memory::Word;

pub struct AInstruction(pub Word);

impl AInstruction {
    pub fn address(&self) -> Word {
        self.0
    }
}
