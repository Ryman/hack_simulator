use memory::{Ram, Rom, Word};
use instructions::Instruction;

pub type Register = Word;
pub type ProgramCounter = Word;

pub struct Cpu {
    ram: Ram,
    rom: Rom,
    pc: ProgramCounter,
    ra: Register,
    rd: Register,
}

impl Cpu {
    pub fn new(program: Rom) -> Cpu {
        Cpu {
            rom: program,
            ram: Ram::new(),
            pc: 0,
            ra: 0,
            rd: 0,
        }
    }

    pub fn step(&mut self) {
        // Destructure for easier field access
        let Cpu { ref mut ram, ref mut ra, ref mut rd, ref mut pc, .. } = *self;

        // Fetch
        let raw_instruction = self.rom[*pc as usize];

        // Decode
        let instruction = Instruction::new(raw_instruction);
        println!("current instruction: {:?}", instruction);

        // Execute
        match instruction {
            Instruction::A(ins) => *ra = ins.address(),
            Instruction::C(ins) => {
                let jump = ins.execute(ram, ra, rd);
                if jump {
                    *pc = *ra;
                    return
                }
            }
        }

        // Set the program counter to the next instruction
        *pc += 1;
    }
}

#[cfg(test)]
mod smoke {
    /// Compare expected values for each cpu step
    /// Returns the `cpu`
    macro_rules! step_cpu(
        // TODO: asm -> call out and compile

        // Compile a hack program then step
        (hack $program:expr; $($name:ident: $($expected:expr),+);*) => {{
            use cpu::*;
            use memory::Rom;
            println!("Executing program:\n{}", $program);

            let program = Rom::from_str($program).unwrap();
            let mut cpu = Cpu::new(program);

            step_cpu!(cpu; $($name: $($expected),+);*)
        }};
        // Core macro
        ($cpu:ident; $($name:ident: $($expected:expr),+);*) => {{
            let mut _max_len = 0;

            $(
                let $name = [$($expected,)+];
                if $name.len() > _max_len {
                    _max_len = $name.len();
                }
            )*

            for _i in 0.._max_len {
                $(
                    if $name.len() > _i && $cpu.$name != $name[_i]{
                        panic!("unexpected `{}` value after `{}` steps. \
                                expected `{}`, got `{}`",
                                stringify!($name), _i, $name[_i], $cpu.$name)
                    }
                )*

                $cpu.step();
            }

            $cpu
        }}
    );

    #[test]
    fn pc_increments() {
        step_cpu!(hack ""; pc: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    }

    #[test]
    fn a_instruction() {
        step_cpu!(hack "0000000000000001\n\
                        0000000000000010\n\
                        0000000000000100\n\
                        0000000000100000\n\
                        0000000001000000\n\
                        0000000001111111\n";
            ra: 0, 1, 2, 4, 32, 64, 127;

            // Ensure it doesn't affect pc or rd
            pc: 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10;
            rd: 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
        );
    }

    #[test]
    fn jmp() {
        step_cpu!(hack "0000000000100000\n\
                        1000101010000111\n"; // JMP
            ra: 0, 32, 32, 0;
            pc: 0, 1, 32, 33;

            // Ensure it doesn't affect rd
            rd: 0, 0, 0, 0, 0
        );
    }

    #[test]
    fn jeq() {
        step_cpu!(hack "0000000000100000\n\
                        1001101010000010\n"; // 0;JEQ
            ra: 0, 32, 32, 0;
            pc: 0, 1, 32, 33;

            // Ensure it doesn't affect rd
            rd: 0, 0, 0, 0, 0
        );
    }

    #[test]
    fn jne() {
        step_cpu!(hack "0000000000100000\n\
                        1000010101000101\n"; // 1;JNE
            ra: 0, 32, 32, 0;
            pc: 0, 1, 32, 33;

            // Ensure it doesn't affect rd
            rd: 0, 0, 0, 0, 0
        );
    }

    #[test]
    fn jlt() {
        step_cpu!(hack "0000000000100000\n\
                        1000111010000100\n"; // -1; JLT
            ra: 0, 32, 32, 0;
            pc: 0, 1, 32, 33;

            // Ensure it doesn't affect rd
            rd: 0, 0, 0, 0, 0
        );
    }

    #[test]
    fn jle() {
        step_cpu!(hack "0000000000100000\n\
                        1000101010000110\n"; // 0; JLE
            ra: 0, 32, 32, 0;
            pc: 0, 1, 32, 33;

            // Ensure it doesn't affect rd
            rd: 0, 0, 0, 0, 0
        );

        step_cpu!(hack "0000000000100000\n\
                        1000111010000110\n"; //  -1; JLE

            ra: 0, 32, 32, 0;
            pc: 0, 1, 32, 33;

            // Ensure it doesn't affect rd
            rd: 0, 0, 0, 0, 0
        );
    }

    #[test]
    fn jgt() {
        step_cpu!(hack "0000000000100000\n\
                        1000010101000001\n"; // 1; JGT
            ra: 0, 32, 32, 0;
            pc: 0, 1, 32, 33;

            // Ensure it doesn't affect rd
            rd: 0, 0, 0, 0, 0
        );
    }

    #[test]
    fn jge() {
        step_cpu!(hack "0000000000100000\n\
                        1000101010000011\n"; // 0; JGE
            ra: 0, 32, 32, 0;
            pc: 0, 1, 32, 33;

            // Ensure it doesn't affect rd
            rd: 0, 0, 0, 0, 0
        );

        step_cpu!(hack "0000000000100000\n\
                        1000010101000011\n"; //  1; JGE
            ra: 0, 32, 32, 0;
            pc: 0, 1, 32, 33;

            // Ensure it doesn't affect rd
            rd: 0, 0, 0, 0, 0
        );
    }

    mod store {
        #[test]
        fn d() {
            step_cpu!(hack "1110010101010000\n"; // D = 1
                rd: 0, 1, 1, 1, 1;

                // Ensure it doesn't affect pc or ra
                pc: 0, 1, 2, 3, 4;
                ra: 0, 0, 0, 0, 0
            );
        }

        #[test]
        fn a() {
            step_cpu!(hack "1110010101100000\n"; // A = 1
                // zeroed instructions after the program cause A to become zero
                ra: 0, 1, 0, 0;

                // Ensure it doesn't affect pc or rd
                pc: 0, 1, 2, 3, 4;
                rd: 0, 0, 0, 0, 0
            );
        }

        #[test]
        fn m() {
            let mut cpu = step_cpu!(hack "0000000000100000\n\
                                          1110010101001000\n";); // M = 1
            // TODO: Support ram[foo] in macro
            assert_eq!(cpu.ram[32], 0);
            cpu.step();
            assert_eq!(cpu.ra, 32);
            cpu.step();
            assert_eq!(cpu.ram[32], 1);
        }

        #[test]
        fn ad() {
            step_cpu!(hack "1110010101110000\n"; // AD = 1
                ra: 0, 1, 0, 0;

                // Ensure it doesn't affect pc or rd
                pc: 0, 1, 2, 3, 4;
                rd: 0, 1, 1, 1, 1
            );
        }

        #[test]
        fn am() {
            let mut cpu = step_cpu!(hack "0000000000100000\n\
                                          1110010101101000\n";); // AM = 1
            // TODO: Support ram[foo] in macro
            assert_eq!(cpu.ram[32], 0);
            cpu.step();
            assert_eq!(cpu.ra, 32);
            cpu.step();
            assert_eq!(cpu.ram[32], 1);
            assert_eq!(cpu.ra, 1);
        }

        #[test]
        fn md() {
            let mut cpu = step_cpu!(hack "0000000000100000\n\
                                          1110010101011000\n";); // MD = 1
            // TODO: Support ram[foo] in macro
            assert_eq!(cpu.ram[32], 0);
            cpu.step();
            assert_eq!(cpu.ra, 32);
            assert_eq!(cpu.rd, 0);
            cpu.step();
            assert_eq!(cpu.ram[32], 1);
            assert_eq!(cpu.rd, 1);
            // Ensure ra is not affected
            assert_eq!(cpu.ra, 32);
        }

        #[test]
        fn amd() {
            let mut cpu = step_cpu!(hack "0000000000100000\n\
                                          1110010101111000\n";); // AMD = 1
            // TODO: Support ram[foo] in macro
            assert_eq!(cpu.ram[32], 0);
            cpu.step();
            assert_eq!(cpu.ra, 32);
            assert_eq!(cpu.rd, 0);
            cpu.step();
            assert_eq!(cpu.ram[32], 1);
            assert_eq!(cpu.rd, 1);
            assert_eq!(cpu.ra, 1);
        }
    }
}
