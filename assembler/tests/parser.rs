extern crate hack_assembler;

#[macro_use] mod macros;

static MAX_PROGRAM : &'static str = r#"
    // This file is part of www.nand2tetris.org
    // and the book "The Elements of Computing Systems"
    // by Nisan and Schocken, MIT Press.
    // File name: projects/06/max/Max.asm

    // Computes R2 = max(R0, R1)  (R0,R1,R2 refer to  RAM[0],RAM[1],RAM[2])

       @R0
       D=M              // D = first number
       @R1
       D=D-M            // D = first number - second number
       @OUTPUT_FIRST
       D;JGT            // if D>0 (first is greater) goto output_first
       @R1
       D=M              // D = second number
       @OUTPUT_D
       0;JMP            // goto output_d
    (OUTPUT_FIRST)
       @R0
       D=M              // D = first number
    (OUTPUT_D)
       @R2
       M=D              // M[2] = D (greatest number)
    (INFINITE_LOOP)
       @INFINITE_LOOP
       0;JMP            // infinite loop
"#;

mod has_more_commands {
    use hack_assembler::*;
    use super::MAX_PROGRAM;

    #[test]
    fn has_more() {
       let parser = Parser::new(MAX_PROGRAM);

       assert!(parser.has_more_commands())
    }

    mod no_commands {
        use hack_assembler::*;

        #[test]
        fn empty() {
           let parser = Parser::new("");

           assert!(!parser.has_more_commands())
        }

        #[test]
        fn only_whitespace() {
           let parser = Parser::new("  \n  ");

           assert!(!parser.has_more_commands())
        }

        #[test]
        fn only_comments() {
           let parser = Parser::new("// a comment");

           assert!(!parser.has_more_commands())
        }

        #[test]
        fn whitespace_and_comments() {
           let parser = Parser::new("  \n  // a comment\n  // another comment\n  ");

           assert!(!parser.has_more_commands())
        }
    }
}

mod advance {
    use hack_assembler::*;
    use super::MAX_PROGRAM;

    #[test]
    fn fewer() {
        let mut parser = Parser::new(MAX_PROGRAM);
        for _ in 0..10 { parser.advance() }

        assert!(parser.has_more_commands())
    }

    #[test]
    fn as_many() {
        let mut parser = Parser::new(MAX_PROGRAM);
        for _ in 0..19 { parser.advance() }

        assert!(!parser.has_more_commands())
    }

    #[test]
    fn interleaved_comments() {
        let input = r#"
            @R0
            // An interleaved comment
            D=M
        "#;
        let mut parser = Parser::new(input);

        assert!(parser.has_more_commands());
        parser.advance();
        assert!(parser.has_more_commands());
        parser.advance();
        assert!(!parser.has_more_commands());
    }
}

mod command_type {
    use hack_assembler::*;

    #[test]
    fn a_instruction() {
        let mut parser = Parser::new("@3");
        parser.advance();

        assert_eq!(parser.command_type(), Command::A);
    }

    #[test]
    fn c_instruction() {
        let mut parser = Parser::new("D=D+A");
        parser.advance();

        assert_eq!(parser.command_type(), Command::C);
    }

    #[test]
    fn label() {
        let mut parser = Parser::new("(INFINITE_LOOP)");
        parser.advance();

        assert_eq!(parser.command_type(), Command::L);
    }
}

mod symbol {
    use hack_assembler::*;

    #[test]
    fn a_symbol() {
        let mut parser = Parser::new("@INFINITE_LOOP");
        parser.advance();

        assert_eq!(parser.symbol(), "INFINITE_LOOP");
    }

    #[test]
    fn a_decimal() {
        let mut parser = Parser::new("@42");
        parser.advance();

        assert_eq!(parser.symbol(), "42");
    }

    #[test]
    fn label() {
        let mut parser = Parser::new("(INFINITE_LOOP)");
        parser.advance();

        assert_eq!(parser.symbol(), "INFINITE_LOOP");
    }
}

check! {
    dest for {
        d_jle         "D;JLE" => "",
        m_eq_d        "M=D" => "M",
        d_eq_d_plus_a "D=D+A" => "D",
        md_eq_dec_m   "MD=M-1" => "MD",
        a_eq_m        "A=M" => "A",
        am_eq_inc_m   "AM=M+1" => "AM",
        ad_eq_inc_a   "AD=A+1" => "AD",
        amd_eq_inc_m  "AMD=M+1" => "AMD"
    } do |command, mnemonic| {
        let mut parser = Parser::new(command);
        parser.advance();
        assert_eq!(parser.dest(), mnemonic);
    }
}

check! {
    comp for {
        zero        "0;JMP" => "0",
        one         "M=1" => "1",
        minus_one   "M=-1" => "-1",
        d           "M=D" => "D",
        a           "D=A" => "A",
        not_d       "D=!D" => "!D",
        not_a       "D=!A" => "!A",
        minus_d     "D=-D" => "-D",
        minus_a     "D=-A" => "-A",
        inc_d       "M=D+1" => "D+1",
        inc_a       "AD=A+1" => "A+1",
        dec_d       "AM=D-1" => "D-1",
        dec_a       "A=A-1" => "A-1",
        d_plus_a    "A=D+A" => "D+A",
        d_minus_a   "A=D-A" => "D-A",
        a_minus_d   "M=A-D" => "A-D",
        d_and_a     "M=D&A" => "D&A",
        d_or_a      "M=D|A" => "D|A",
        m           "D=M" => "M",
        not_m       "M=!M" => "!M",
        minus_m     "A=-M" => "-M",
        inc_m       "AM=M+1" => "M+1",
        dec_m       "A=M-1" => "M-1",
        d_plus_m    "M=D+M" => "D+M",
        d_minus_m   "D=D-M" => "D-M",
        m_minus_d   "M=M-D" => "M-D",
        d_and_m     "M=D&M" => "D&M",
        d_or_m      "M=D|M" => "D|M",
        eq_zero     "D=0;JMP" => "0"
    } do |command, mnemonic| {
        let mut parser = Parser::new(command);
        parser.advance();
        assert_eq!(parser.comp(), mnemonic);
    }
}

check! {
    jump for {
        empty   "D=D+A" => "",
        jgt     "D;JGT" => "JGT",
        jeq     "A;JEQ" => "JEQ",
        jge     "A-1;JGE" => "JGE",
        jlt     "M;JLT" => "JLT",
        jne     "!D;JNE" => "JNE",
        jle     "D+1;JLE" => "JLE",
        jmp     "0;JMP" => "JMP"
    } do |command, mnemonic| {
        let mut parser = Parser::new(command);
        parser.advance();
        assert_eq!(parser.jump(), mnemonic);
    }
}
