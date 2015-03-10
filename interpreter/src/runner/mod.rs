use std::str::FromStr;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::fs::{File, PathExt};
use {Rom, Cpu};

use self::commands::{Command, Commands};
#[macro_use] mod macros;
mod commands;

pub fn runner(base: &Path) -> Result<(), String> {
    let tst = file_to_string!(base);
    let commands = Commands::new(&tst);
    let program = try_s!(Rom::from_str(""));

    let mut runner = Runner {
        base_path: base,
        cpu: Cpu::new(program),
        output_path: None,
        comparison: String::new(),
        formats: vec![],
        output: String::new(),
    };

    for cmd in commands {
        let cmd = try!(cmd);
        try!(runner.step(&cmd))
    }

    runner.flush_output()
}

struct Runner<'a> {
    base_path: &'a Path,
    cpu: Cpu,
    output_path: Option<PathBuf>,
    comparison: String,
    formats: Vec<&'a str>,
    output: String
}

impl<'a> Runner<'a> {
    fn step(&mut self, cmd: &Command<'a>) -> Result<(), String> {
        use self::commands::Command::*;

        match *cmd {
            Repeat(times, ref commands) => {
                for _ in 0..times {
                    for c in commands {
                        try!(self.step(c))
                    }
                }
                Ok(())
            }
            OutputFile(ref filename) => {
                self.output_path = Some(self.base_path.with_file_name(&filename));
                Ok(())
            }
            OutputList(ref formats) => self.set_formatting(formats),
            TickTock => { self.cpu.step(); Ok(()) },
            Output => self.check_output_line(),
            Load(ref filename) => self.load_program(filename),
            CompareTo(ref filename) => self.compare_with(filename),
            Set(ref dest, val) => self.set(dest, val),
        }
    }

    fn compare_with(&mut self, filename: &str) -> Result<(), String> {
        let path = self.base_path.with_file_name(filename);
        self.comparison = file_to_string!(&path);
        Ok(())
    }

    fn load_program(&mut self, filename: &str) -> Result<(), String> {
        if !filename.ends_with(".hack") {
            return Err(format!("Unsupported file type: {}", filename))
        }

        let rom = try_s!(Rom::from_file(&self.base_path.with_file_name(&filename)));
        self.cpu = Cpu::new(rom);
        Ok(())
    }

    fn set(&mut self, dest: &str, val: i16) -> Result<(), String> {
        let mut parts = dest.split(|c| c == ' ' || c == '[' || c == ']');

        match expect!(parts, "destination for set") {
            "PC" | "pc" => self.cpu.pc = val as u16,
            "RAM" | "ram" => {
                let index = expect!(parts, "index for set RAM[?]");
                let index : usize = try_s!(index.parse());

                self.cpu.ram[index] = val as u16;
            }
            dest => return Err(format!("Unhandled set destination: {}", dest))
        }

        Ok(())
    }

    fn flush_output(&mut self) -> Result<(), String> {
        let path = try!(self.output_path.as_ref()
                            .ok_or(format!("No output file specified")));
        let mut f = try_s!(File::create(path));
        let _ = try_s!(f.write_all(self.output.as_bytes()));
        Ok(())
    }

    fn check_output_line(&mut self) -> Result<(), String> {
        try!(self.write_output_line(false));

        let expected = self.comparison.lines();
        let actual = self.output.lines();

        for (lineno, (a, b)) in actual.zip(expected).enumerate() {
            if a.trim() != b.trim() {
                return Err(format!("Comparison failed at line: {}\n\
                                    Got: '{}'\nExpected: '{}'",
                                    lineno, a, b))
            }
        }
        Ok(())
    }

    fn set_formatting(&mut self, formats: &[&'a str]) -> Result<(), String> {
        if self.output.len() > 0 {
            return Err("Format string changed after output command".to_string())
        }

        self.formats = formats.to_vec();
        self.write_output_line(true)
    }

    fn write_output_line(&mut self, is_header: bool) -> Result<(), String> {
        // TODO: Default formatting as specified in appendix if none given
        for format in &self.formats {
            let parts = &mut format.split(|c| c == '%' || c == '.' ||
                                              c == '[' || c == ']');

            let val = try!(get_val(parts, &self.cpu));

            let left_fmt = expect!(parts, "type for format");
            debug!("fmt {:?}, left_fmt {:?}", format, left_fmt);

            let val = if is_header {
                try!(header_name(format))
            } else {
                try!(format_val(&left_fmt[0..1], val))
            };

            let lpad = try_s!(left_fmt[1..].parse());
            let len = try_s!(expect!(parts, "length for format").parse());
            let rpad = try_s!(expect!(parts, "right padding for format").parse());

            debug!("lpad: {}, val_len: {}, len: {}, rpad: {}",
                    lpad, val.len(), len, rpad);

            let padding = |l| (0..l).map(|_| ' ');
            self.output.push('|');
            self.output.extend(padding(lpad + (len - val.len())));
            self.output.push_str(&val);
            self.output.extend(padding(rpad));
        }

        self.output.push_str("|\n");

        return Ok(());

        fn get_val<'a, I: Iterator<Item=&'a str>>(parts: &mut I, cpu: &Cpu) -> Result<u16, String> {
            let val = match expect!(parts, "destination for format") {
                "PC" | "pc" => cpu.pc,
                "RAM" | "ram" => {
                    let index = expect!(parts, "index for set RAM[?]");
                    let index : usize = try_s!(index.parse());

                    // discard the space between RAM[..] and '%' from splits
                    assert_eq!(expect!(parts, "skipping blank space"), "");
                    cpu.ram[index]
                }
                dest => return Err(format!("Unhandled set destination: {}", dest))
            };

            Ok(val)
        }

        fn header_name(format: &str) -> Result<String, String> {
            Ok(expect!(format.split('%'),
                        "destination for format header").to_string())
        }

        fn format_val(format: &str, val: u16) -> Result<String, String> {
            let s = match format {
                // String
                "S" => unimplemented!(),
                // Binary
                "B" => format!("{:b}", val),
                // Hex
                "X" => format!("{:X}", val),
                // Decimal
                "D" => format!("{}", val),
                fmt => return Err(format!("Unknown format: {}", fmt)),
            };

            Ok(s)
        }
    }
}
