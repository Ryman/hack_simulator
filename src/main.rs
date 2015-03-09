#![feature(path)]
#![cfg(not(test))]

extern crate hack_interpreter;
extern crate "rustc-serialize" as rustc_serialize;
extern crate docopt;

use std::path::AsPath;
use hack_interpreter::{Rom, Cpu, runner};
use docopt::Docopt;

static USAGE: &'static str = "
Usage: hack-interpreter [-r] <input>

Options:
    -r, --runner  Run a .tst file
";

#[derive(RustcDecodable, Debug)]
struct Args {
    arg_input: String,
    flag_runner: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode())
                                       .unwrap_or_else(|e| e.exit());
    let input = &args.arg_input;

    if args.flag_runner {
        println!("Running test file: '{}'", input);
        match runner(input.as_path()) {
            Ok(()) => println!("Test completed successfully"),
            Err(e) => panic!("{}", e),
        }
    } else {
        println!("Running program file: '{}'", input);

        let program = Rom::from_file(input).unwrap();
        let mut cpu = Cpu::new(program);

        loop {
            cpu.step()

            //TODO: Render
        }
    }
}
