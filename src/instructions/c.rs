use memory::{Ram, Word};
use cpu::Register;

pub struct CInstruction(pub Word);

impl CInstruction {
    /// Returns true if a jump should occur
    pub fn execute(self, ram: &mut Ram,
                           a: &mut Register,
                           d: &mut Register) -> bool {
        // determine inputs
        let x = *d;
        let y = if self.a_is_address() { ram[*a as usize] } else { *a };

        // do computation
        let (result, zero, negative) = self.computation(x, y);

        // save to destinations
        self.store(result, ram, a, d);

        // return jump intention
        self.jump(zero, negative)
    }

    fn a_is_address(&self) -> bool {
        self.0 & (1 << 12) == 0
    }

    fn computation(&self, _x: Word, _y: Word) -> (Word, bool, bool) {
        let bit = |i: usize| ((self.0 & (1<<i) != 0) as u8);

        let result: i16 = match (bit(11), bit(10), bit(9), bit(8), bit(7), bit(6)) {
            (1, 0, 1, 0, 1, 0) => 0,
            (0, 1, 0, 1, 0, 1) => 1,
            (1, 1, 1, 0, 1, 0) => -1,
            (c1, c2, c3, c4, c5, c6) => panic!("Unhandled computation: {}{}{}{}{}{}",
                                                c1, c2, c3, c4, c5, c6)
        };

        (result as u16, result == 0, result < 0)
    }

    fn store(&self, result: Word,
                    ram: &mut Ram,
                    a: &mut Register,
                    d: &mut Register) {
        let bit = |i: usize| ((self.0 & (1<<i) != 0));

        if bit(3) { ram[*a as usize] = result }
        if bit(4) { *d = result }
        if bit(5) { *a = result }
    }

    fn jump(&self, zero: bool, negative: bool) -> bool {
        let bit = |i: usize| ((self.0 & (1<<i) != 0) as u8);

        match (bit(2), bit(1), bit(0)) {
            (0, 0, 0) => false, // null
            (0, 0, 1) => !(negative || zero), // JGT
            (0, 1, 0) => zero, // JEQ
            (0, 1, 1) => !negative, // JGE
            (1, 0, 1) => !zero, // JNE
            (1, 0, 0) => negative, // JLT
            (1, 1, 0) => negative || zero, // JLE
            (1, 1, 1) => true, // JMP

            // This would be dead code if we match on booleans
            // but 0s and 1s are more readable
            (j1, j2, j3) => panic!("Unhandled jump {}{}{}", j1, j2, j3)
        }
    }
}
