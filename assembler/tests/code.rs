extern crate hack_assembler;

#[macro_use] mod macros;

check! {
    dest for {
        empty   "" => "000",
        m       "M" => "001",
        d       "D" => "010",
        md      "MD" => "011",
        a       "A" => "100",
        am      "AM" => "101",
        ad      "AD" => "110",
        amd     "AMD" => "111"
    } do |mnemonic, bits| {
        assert_eq!(Code::dest(mnemonic), bits)
    }
}

check! {
    comp for {
        zero        "0" => "0101010",
        one         "1" => "0111111",
        minus_one   "-1" => "0111010",
        d           "D" => "0001100",
        a           "A" => "0110000",
        not_d       "!D" => "0001101",
        not_a       "!A" => "0110001",
        minus_d     "-D" => "0001111",
        minus_a     "-A" => "0110011",
        inc_d       "D+1" => "0011111",
        inc_a       "A+1" => "0110111",
        dec_d       "D-1" => "0001110",
        dec_a       "A-1" => "0110010",
        d_plus_a    "D+A" => "0000010",
        d_minus_a   "D-A" => "0010011",
        a_minus_d   "A-D" => "0000111",
        d_and_a     "D&A" => "0000000",
        d_or_a      "D|A" => "0010101",
        m           "M" => "1110000",
        not_m       "!M" => "1110001",
        minus_m     "-M" => "1110011",
        inc_m       "M+1" => "1110111",
        dec_m       "M-1" => "1110010",
        d_plus_m    "D+M" => "1000010",
        d_minus_m   "D-M" => "1010011",
        m_minus_d   "M-D" => "1000111",
        d_and_m     "D&M" => "1000000",
        d_or_m      "D|M" => "1010101"
    } do |mnemonic, bits| {
        assert_eq!(Code::comp(mnemonic).unwrap(), bits)
    }
}

check! {
    jump for {
        empty   "" => "000",
        jgt     "JGT" => "001",
        jeq     "JEQ" => "010",
        jge     "JGE" => "011",
        jlt     "JLT" => "100",
        jne     "JNE" => "101",
        jle     "JLE" => "110",
        jmp     "JMP" => "111"
    } do |mnemonic, bits| {
        assert_eq!(Code::jump(mnemonic), bits)
    }
}
