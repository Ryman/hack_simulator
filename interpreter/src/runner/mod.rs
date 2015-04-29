use std::str::FromStr;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::fs::File;
use {Rom, Cpu};
use hack_assembler::assemble;

use self::commands::{Command, Commands};
#[macro_use] mod macros;
mod commands;

pub fn runner<P: AsRef<Path>>(base: P) -> Result<(), String> {
    let tst = file_to_string!(base.as_ref());
    let commands = Commands::new(&tst);
    let program = try_s!(Rom::from_str(""));

    let mut runner = Runner {
        base_path: base.as_ref(),
        cpu: Cpu::new(program),
        output_path: None,
        comparison: String::new(),
        formats: vec![],
        output: String::new(),
    };

    for cmd in commands {
        let cmd = try!(cmd);
        match runner.step(&cmd) {
            Ok(..) => {},
            Err(e) => {
                let _ = runner.flush_output();
                return Err(format!("Failure running '{}':\n{}",
                                    base.as_ref().to_string_lossy(),
                                    e))
            }
        }
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
        let path = self.base_path.with_file_name(&filename);
        let rom = if filename.ends_with(".asm") {
            let assembly = file_to_string!(&path);
            let program = try!(assemble(&assembly));
            Rom::from_str(&program)
        } else if filename.ends_with(".hack") {
            Rom::from_file(&path)
        } else {
            return Err(format!("Unsupported file type: {}", filename))
        };

        self.cpu = Cpu::new(try_s!(rom));
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

        for (lineno, (line_a, line_b)) in actual.zip(expected).enumerate() {
            // Only care about mismatches if the actual cell content is different
            for (a, b) in line_a.split('|').zip(line_b.split('|')) {
                if a.trim() != b.trim() {
                    return Err(format!("Comparison failed at line: {}\n\
                                        Got: '{}'\nExpected: '{}'",
                                        lineno, line_a, line_b))
                }
            }
        }
        Ok(())
    }

    fn set_formatting(&mut self, formats: &[&'a str]) -> Result<(), String> {
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

            let mut val = if is_header {
                try!(header_name(format))
            } else {
                try!(format_val(&left_fmt[0..1], val as i16))
            };

            let lpad: isize = try_s!(left_fmt[1..].parse());
            let len: isize = try_s!(expect!(parts, "length for format").parse());
            let rpad = try_s!(expect!(parts, "right padding for format").parse());
            let max_len = lpad + len + rpad;
            let val_len = val.len() as isize;

            debug!("lpad: {}, val_len: {}, len: {}, rpad: {}",
                    lpad, val_len, len, rpad);

            let padding = |l| (0..l).map(|_| ' ');
            self.output.push('|');

            // The JVM version first uses the left pad
            // then the right pad and then truncates silently.
            let lpad = lpad + (len - val_len);
            if lpad > 0 {
                self.output.extend(padding(lpad))
            }

            let rpad = if val_len > max_len {
                if is_header {
                    // Silent truncation for headers \o/
                    val.truncate(max_len as usize);
                    0
                } else {
                    return Err(format!("value '{}' could not fit in the specified\
                                        formatting: '{}", val, format))
                }
            } else if val_len + rpad > max_len {
                max_len - val_len
            } else {
                rpad
            };

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

        fn format_val(format: &str, val: i16) -> Result<String, String> {
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
