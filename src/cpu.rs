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
        // @32, 0;JMP
        step_cpu!(hack "0000000000100000\n\
                        1000101010000111\n";
            ra: 0, 32, 32, 0;
            pc: 0, 1, 32, 33;

            // Ensure it doesn't affect rd
            rd: 0, 0, 0, 0, 0
        );

        // @32, 1;JMP
        step_cpu!(hack "0000000000100000\n\
                        1000111111000111\n";
            ra: 0, 32, 32, 0;
            pc: 0, 1, 32, 33;

            // Ensure it doesn't affect rd
            rd: 0, 0, 0, 0, 0
        );

        // @32, -1;JMP
        step_cpu!(hack "0000000000100000\n\
                        1000111010000111\n";
            ra: 0, 32, 32, 0;
            pc: 0, 1, 32, 33;

            // Ensure it doesn't affect rd
            rd: 0, 0, 0, 0, 0
        );
    }

    #[test]
    fn jeq() {
        // @32, 0;JEQ
        step_cpu!(hack "0000000000100000\n\
                        1001101010000010\n";
            ra: 0, 32, 32, 0;
            pc: 0, 1, 32, 33;

            // Ensure it doesn't affect rd
            rd: 0, 0, 0, 0, 0
        );
    }

    #[test]
    fn jne() {
        // @32, 1;JNE
        step_cpu!(hack "0000000000100000\n\
                        1000111111000101\n";
            ra: 0, 32, 32, 0;
            pc: 0, 1, 32, 33;

            // Ensure it doesn't affect rd
            rd: 0, 0, 0, 0, 0
        );
    }

    #[test]
    fn jlt() {
        // @32, -1;JLT
        step_cpu!(hack "0000000000100000\n\
                        1000111010000100\n";
            ra: 0, 32, 32, 0;
            pc: 0, 1, 32, 33;

            // Ensure it doesn't affect rd
            rd: 0, 0, 0, 0, 0
        );
    }

    #[test]
    fn jle() {
        // @32, 0;JLE
        step_cpu!(hack "0000000000100000\n\
                        1000101010000110\n";
            ra: 0, 32, 32, 0;
            pc: 0, 1, 32, 33;

            // Ensure it doesn't affect rd
            rd: 0, 0, 0, 0, 0
        );

        // @32, -1;JLE
        step_cpu!(hack "0000000000100000\n\
                        1000111010000110\n";

            ra: 0, 32, 32, 0;
            pc: 0, 1, 32, 33;

            // Ensure it doesn't affect rd
            rd: 0, 0, 0, 0, 0
        );
    }

    #[test]
    fn jgt() {
         // @32, 1;JGT
        step_cpu!(hack "0000000000100000\n\
                        1000111111000001\n";
            ra: 0, 32, 32, 0;
            pc: 0, 1, 32, 33;

            // Ensure it doesn't affect rd
            rd: 0, 0, 0, 0, 0
        );
    }

    #[test]
    fn jge() {
        // @32, 0;JGE
        step_cpu!(hack "0000000000100000\n\
                        1000101010000011\n";
            ra: 0, 32, 32, 0;
            pc: 0, 1, 32, 33;

            // Ensure it doesn't affect rd
            rd: 0, 0, 0, 0, 0
        );

        // @32, 1;JGE
        step_cpu!(hack "0000000000100000\n\
                        1000111111000011\n";
            ra: 0, 32, 32, 0;
            pc: 0, 1, 32, 33;

            // Ensure it doesn't affect rd
            rd: 0, 0, 0, 0, 0
        );
    }

    mod compute {
        #[test]
        fn d() {
            // D=1, A=D
            step_cpu!(hack "1000111111010000\n\
                            1000001100100000\n";
                ra: 0, 0, 1;
                pc: 0, 1, 2, 3;
                rd: 0, 1, 1, 1
            );
        }

        #[test]
        fn a() {
            // A=8, D=A
            step_cpu!(hack "0000000000001000\n\
                            1000110000010000\n";
                ra: 0, 8, 8;
                pc: 0, 1, 2, 3;
                rd: 0, 0, 8, 8
            );
        }

        #[test]
        fn m() {
            // D=1, D=M[0]
            step_cpu!(hack "1000111111010000\n\
                            1001110000010000\n";
                ra: 0, 0, 0, 0;
                pc: 0, 1, 2, 3;
                rd: 0, 1, 0, 0
            );
        }

        #[test]
        fn not_d() {
            // D=!D
            step_cpu!(hack "1000001101010000";
                ra: 0, 0, 0, 0;
                pc: 0, 1, 2, 3;
                rd: 0, 0xFFFF, 0xFFFF, 0xFFFF
            );
        }

        #[test]
        fn not_a() {
            // A=!A
            step_cpu!(hack "1000110001100000";
                ra: 0, 0xFFFF, 0, 0;
                pc: 0, 1, 2, 3;
                rd: 0, 0, 0, 0
            );
        }

        #[test]
        fn not_m() {
            // M=1, D=!M
            let mut cpu = step_cpu!(hack "1000111111001000\n\
                                          1001110001010000";);
            assert_eq!(cpu.ram[0], 0);
            assert_eq!(cpu.ra, 0);
            cpu.step();
            assert_eq!(cpu.ram[0], 1);
            assert_eq!(cpu.ra, 0);
            cpu.step();
            assert_eq!(cpu.ram[0], 1);
            assert_eq!(cpu.rd, 0xFFFE);
        }

        #[test]
        fn negate_d() {
            // D=1, D=-D
            step_cpu!(hack "1000111111010000\n\
                            1000001111010000";
                ra: 0, 0, 0, 0;
                pc: 0, 1, 2, 3;
                rd: 0, 1, -1, -1
            );
        }

        #[test]
        fn negate_a() {
            // A=1, D=-A
            step_cpu!(hack "1000111111100000\n\
                            1000110011010000";
                ra: 0, 1, 1, 0;
                pc: 0, 1, 2, 3;
                rd: 0, 0, -1, -1
            );
        }

        #[test]
        fn negate_m() {
            // M=1, D=-M
            let mut cpu = step_cpu!(hack "1000111111001000\n\
                                          1001110011010000";);
            assert_eq!(cpu.ram[0], 0);
            assert_eq!(cpu.ra, 0);
            cpu.step();
            assert_eq!(cpu.ram[0], 1);
            assert_eq!(cpu.ra, 0);
            cpu.step();
            assert_eq!(cpu.ram[0], 1);
            assert_eq!(cpu.rd, -1);
        }

        #[test]
        fn inc_d() {
            // D=D+1, A=D+1
            step_cpu!(hack "1000011111010000\n\
                            1000011111100000";
                ra: 0, 0, 2;
                pc: 0, 1, 2;
                rd: 0, 1, 1
            );
        }

        #[test]
        fn inc_a() {
            // A=A+1, D=A+1
            step_cpu!(hack "1000110111100000\n\
                            1000110111010000";
                ra: 0, 1, 1, 0;
                pc: 0, 1, 2, 3;
                rd: 0, 0, 2, 2
            );
        }

        #[test]
        fn inc_m() {
            // M=M+1, D=M+1
            let mut cpu = step_cpu!(hack "1001110111001000\n\
                                          1001110111010000";);
            assert_eq!(cpu.ram[0], 0);
            assert_eq!(cpu.ra, 0);
            cpu.step();
            assert_eq!(cpu.ram[0], 1);
            assert_eq!(cpu.ra, 0);
            cpu.step();
            assert_eq!(cpu.ram[0], 1);
            assert_eq!(cpu.rd, 2);
        }

        #[test]
        fn dec_d() {
            // D=D-1, A=D-1
            step_cpu!(hack "1000001110010000\n\
                            1000001110100000";
                ra: 0, 0, -2;
                pc: 0, 1, 2;
                rd: 0, -1, -1
            );
        }

        #[test]
        fn dec_a() {
            // A=A-1, D=A-1
            step_cpu!(hack "1000110010100000\n\
                            1000110010010000";
                ra: 0, -1, -1, 0;
                pc: 0, 1, 2, 3;
                rd: 0, 0, -2, -2
            );
        }

        #[test]
        fn dec_m() {
            // M=M-1, D=M-1
            let mut cpu = step_cpu!(hack "1001110010001000\n\
                                          1001110010010000";);
            assert_eq!(cpu.ram[0], 0);
            assert_eq!(cpu.ra, 0);
            cpu.step();
            assert_eq!(cpu.ram[0], -1);
            assert_eq!(cpu.ra, 0);
            cpu.step();
            assert_eq!(cpu.ram[0], -1);
            assert_eq!(cpu.rd, -2);
        }

        #[test]
        fn add_da() {
            // A=5, D=D+A, A=D+A
            step_cpu!(hack "0000000000000101\n\
                            1000000010010000\n\
                            1000000010100000";
                ra: 0, 5, 5, 10;
                pc: 0, 1, 2, 3;
                rd: 0, 0, 5, 5
            );
        }

        #[test]
        fn add_dm() {
            // A=5, M=D+A, D=D+M
            let mut cpu = step_cpu!(hack "0000000000000101\n\
                                          1000000010001000\n\
                                          1001000010010000";);
            assert_eq!(cpu.ram[0], 0);
            assert_eq!(cpu.ra, 0);
            cpu.step();
            assert_eq!(cpu.ram[5], 0);
            assert_eq!(cpu.ra, 5);
            assert_eq!(cpu.rd, 0);
            cpu.step();
            assert_eq!(cpu.ram[5], 5);
            assert_eq!(cpu.ra, 5);
            assert_eq!(cpu.rd, 0);
            cpu.step();
            assert_eq!(cpu.ram[5], 5);
            assert_eq!(cpu.ra, 5);
            assert_eq!(cpu.rd, 5);
        }

        #[test]
        fn d_minus_a() {
            // A=5, D=D-A, A=D-A
            step_cpu!(hack "0000000000000101\n\
                            1000010011010000\n\
                            1000010011100000";
                ra: 0, 5, 5, -10;
                pc: 0, 1, 2, 3;
                rd: 0, 0, -5, -5
            );
        }

        #[test]
        fn d_minus_m() {
            // A=5, M=D-A, D=D-M
            let mut cpu = step_cpu!(hack "0000000000000101\n\
                                          1000010011001000\n\
                                          1001010011010000\n\
                                          1001010011010000";);
            assert_eq!(cpu.ram[0], 0);
            assert_eq!(cpu.ra, 0);
            cpu.step();
            assert_eq!(cpu.ram[5], 0);
            assert_eq!(cpu.ra, 5);
            assert_eq!(cpu.rd, 0);
            cpu.step();
            assert_eq!(cpu.ram[5], -5);
            assert_eq!(cpu.ra, 5);
            assert_eq!(cpu.rd, 0);
            cpu.step();
            assert_eq!(cpu.ram[5], -5);
            assert_eq!(cpu.ra, 5);
            assert_eq!(cpu.rd, 5);
            cpu.step();
            assert_eq!(cpu.ram[5], -5);
            assert_eq!(cpu.ra, 5);
            assert_eq!(cpu.rd, 10);
        }

        #[test]
        fn a_minus_d() {
            // A=5, D=A-D, A=A-D
            step_cpu!(hack "0000000000000101\n\
                            1000000111010000\n\
                            1000000111100000";
                ra: 0, 5, 5, 0;
                pc: 0, 1, 2, 3;
                rd: 0, 0, 5, 5
            );
        }

        #[test]
        fn m_minus_d() {
            // A=5, M=A-D, D=M-D
            let mut cpu = step_cpu!(hack "0000000000000101\n\
                                          1000000111001000\n\
                                          1001000111010000\n\
                                          1001000111010000";);
            assert_eq!(cpu.ram[0], 0);
            assert_eq!(cpu.ra, 0);
            cpu.step();
            assert_eq!(cpu.ram[5], 0);
            assert_eq!(cpu.ra, 5);
            assert_eq!(cpu.rd, 0);
            cpu.step();
            assert_eq!(cpu.ram[5], 5);
            assert_eq!(cpu.ra, 5);
            assert_eq!(cpu.rd, 0);
            cpu.step();
            assert_eq!(cpu.ram[5], 5);
            assert_eq!(cpu.ra, 5);
            assert_eq!(cpu.rd, 5);
            cpu.step();
            assert_eq!(cpu.ram[5], 5);
            assert_eq!(cpu.ra, 5);
            assert_eq!(cpu.rd, 0);
        }

        #[test]
        fn d_and_a() {
            // A=5, D=D&A, A=D&A
            step_cpu!(hack "0000000000000101\n\
                            1000000000010000\n\
                            1000000000100000";
                ra: 0, 5, 5, 0;
                pc: 0, 1, 2, 3;
                rd: 0, 0, 0, 0
            );
        }

        #[test]
        fn d_and_m() {
            // A=5, D=A, M=1, A=D&M
            let mut cpu = step_cpu!(hack "0000000000000101\n\
                                          1000110000010000\n\
                                          1000111111001000\n\
                                          1001000000100000";);
            assert_eq!(cpu.ram[0], 0);
            assert_eq!(cpu.ra, 0);
            cpu.step();
            assert_eq!(cpu.ram[5], 0);
            assert_eq!(cpu.ra, 5);
            assert_eq!(cpu.rd, 0);
            cpu.step();
            assert_eq!(cpu.ram[5], 0);
            assert_eq!(cpu.ra, 5);
            assert_eq!(cpu.rd, 5);
            cpu.step();
            assert_eq!(cpu.ram[5], 1);
            assert_eq!(cpu.ra, 5);
            assert_eq!(cpu.rd, 5);
            cpu.step();
            assert_eq!(cpu.ram[5], 1);
            assert_eq!(cpu.ra, 1);
            assert_eq!(cpu.rd, 5);
        }

        #[test]
        fn d_or_a() {
            // A=5, D=D|A, A=D|A
            step_cpu!(hack "0000000000000101\n\
                            1000010101010000\n\
                            1000010101100000";
                ra: 0, 5, 5, 5;
                pc: 0, 1, 2, 3;
                rd: 0, 0, 5, 5
            );
        }

        #[test]
        fn d_or_m() {
            // A=4, D=A, M=1, A=D|M
            let mut cpu = step_cpu!(hack "0000000000000100\n\
                                          1000110000010000\n\
                                          1000111111001000\n\
                                          1001010101100000";);
            assert_eq!(cpu.ram[0], 0);
            assert_eq!(cpu.ra, 0);
            cpu.step();
            assert_eq!(cpu.ram[4], 0);
            assert_eq!(cpu.ra, 4);
            assert_eq!(cpu.rd, 0);
            cpu.step();
            assert_eq!(cpu.ram[4], 0);
            assert_eq!(cpu.ra, 4);
            assert_eq!(cpu.rd, 4);
            cpu.step();
            assert_eq!(cpu.ram[4], 1);
            assert_eq!(cpu.ra, 4);
            assert_eq!(cpu.rd, 4);
            cpu.step();
            assert_eq!(cpu.ram[4], 1);
            assert_eq!(cpu.ra, 5);
            assert_eq!(cpu.rd, 4);
        }
    }

    mod store {
        #[test]
        fn d() {
            // D=1
            step_cpu!(hack "1110111111010000\n";
                rd: 0, 1, 1, 1, 1;

                // Ensure it doesn't affect pc or ra
                pc: 0, 1, 2, 3, 4;
                ra: 0, 0, 0, 0, 0
            );
        }

        #[test]
        fn a() {
            // A=1
            step_cpu!(hack "1110111111100000\n";
                // zeroed instructions after the program cause A to become zero
                ra: 0, 1, 0, 0;

                // Ensure it doesn't affect pc or rd
                pc: 0, 1, 2, 3, 4;
                rd: 0, 0, 0, 0, 0
            );
        }

        #[test]
        fn m() {
            // M=1
            let mut cpu = step_cpu!(hack "0000000000100000\n\
                                          1110111111001000\n";);
            // TODO: Support ram[foo] in macro
            assert_eq!(cpu.ram[32], 0);
            assert_eq!(cpu.rd, 0);
            assert_eq!(cpu.ra, 0);
            cpu.step();
            assert_eq!(cpu.ram[32], 0);
            assert_eq!(cpu.ra, 32);
            assert_eq!(cpu.rd, 0);
            cpu.step();
            assert_eq!(cpu.ram[32], 1);
            assert_eq!(cpu.ra, 32);
            assert_eq!(cpu.rd, 0);
        }

        #[test]
        fn ad() {
            // AD=1
            step_cpu!(hack "1110111111110000\n";
                ra: 0, 1, 0, 0;

                // Ensure it doesn't affect pc or rd
                pc: 0, 1, 2, 3, 4;
                rd: 0, 1, 1, 1, 1
            );
        }

        #[test]
        fn am() {
            // AM=1
            let mut cpu = step_cpu!(hack "0000000000100000\n\
                                          1110111111101000\n";);
            // TODO: Support ram[foo] in macro
            assert_eq!(cpu.ram[32], 0);
            assert_eq!(cpu.ra, 0);
            assert_eq!(cpu.rd, 0);
            cpu.step();
            assert_eq!(cpu.ram[32], 0);
            assert_eq!(cpu.ra, 32);
            assert_eq!(cpu.rd, 0);
            cpu.step();
            assert_eq!(cpu.ram[32], 1);
            assert_eq!(cpu.ra, 1);
            assert_eq!(cpu.rd, 0);
        }

        #[test]
        fn md() {
            // MD=1
            let mut cpu = step_cpu!(hack "0000000000100000\n\
                                          1110111111011000\n";);
            // TODO: Support ram[foo] in macro
            assert_eq!(cpu.ram[32], 0);
            assert_eq!(cpu.ra, 0);
            assert_eq!(cpu.rd, 0);
            cpu.step();
            assert_eq!(cpu.ram[32], 0);
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
            // AMD=1
            let mut cpu = step_cpu!(hack "0000000000100000\n\
                                          1110111111111000\n";);
            // TODO: Support ram[foo] in macro
            assert_eq!(cpu.ram[32], 0);
            assert_eq!(cpu.ra, 0);
            assert_eq!(cpu.rd, 0);
            cpu.step();
            assert_eq!(cpu.ram[32], 0);
            assert_eq!(cpu.ra, 32);
            assert_eq!(cpu.rd, 0);
            cpu.step();
            assert_eq!(cpu.ram[32], 1);
            assert_eq!(cpu.rd, 1);
            assert_eq!(cpu.ra, 1);
        }
    }
}
