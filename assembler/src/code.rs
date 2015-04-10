pub struct Code;

impl Code {
    pub fn dest(mnemonic: &str) -> &'static str {
        let d1 = mnemonic.contains('A');
        let d2 = mnemonic.contains('D');
        let d3 = mnemonic.contains('M');

        tribit_str(d1, d2, d3)
    }

    pub fn comp(mnemonic: &str) -> Result<&'static str, String> {
        // Non-standard but convenient remappings
        let canonical = match mnemonic {
            "A+D" => "D+A",
            "A&D" => "D&A",
            "A|D" => "D|A",
            "M+D" => "D+M",
            "M&D" => "D&M",
            "M|D" => "D|M",
            mnemonic => mnemonic
        };

        let code = match canonical {
            "0" => "0101010",
            "1" => "0111111",
            "-1" => "0111010",
            "D" => "0001100",
            "A" => "0110000",
            "!D" => "0001101",
            "!A" => "0110001",
            "-D" => "0001111",
            "-A" => "0110011",
            "D+1" => "0011111",
            "A+1" => "0110111",
            "D-1" => "0001110",
            "A-1" => "0110010",
            "D+A" => "0000010",
            "D-A" => "0010011",
            "A-D" => "0000111",
            "D&A" => "0000000",
            "D|A" => "0010101",
            "M" => "1110000",
            "!M" => "1110001",
            "-M" => "1110011",
            "M+1" => "1110111",
            "M-1" => "1110010",
            "D+M" => "1000010",
            "D-M" => "1010011",
            "M-D" => "1000111",
            "D&M" => "1000000",
            "D|M" => "1010101",
            mnemonic => return Err(format!("Unknown mnemonic: {}", mnemonic))
        };

        Ok(code)
    }

    pub fn jump(mnemonic: &str) -> &'static str {
        if mnemonic == "JMP" { return "111" }
        if mnemonic == "JNE" { return "101" }

        let j1 = mnemonic.contains('L');
        let j2 = mnemonic.contains('E');
        let j3 = mnemonic.contains('G');
        tribit_str(j1, j2, j3)
    }
}

#[inline]
fn tribit_str(b1: bool, b2: bool, b3: bool) -> &'static str {
    const T: bool = true;
    const F: bool = false;

    match (b1, b2, b3) {
        (F, F, F) => "000",
        (F, F, T) => "001",
        (F, T, F) => "010",
        (F, T, T) => "011",
        (T, F, F) => "100",
        (T, F, T) => "101",
        (T, T, F) => "110",
        (T, T, T) => "111",
    }
}
