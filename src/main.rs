#![cfg(not(test))]

extern crate hack_interpreter;
extern crate rustc_serialize;
extern crate docopt;
extern crate piston;
extern crate image;
extern crate graphics;
extern crate opengl_graphics;
extern crate sdl2_window;

use hack_interpreter::runner;
use docopt::Docopt;
use simulator::run_simulator;

mod simulator;

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
        match runner(input) {
            Ok(()) => println!("Test completed successfully"),
            Err(e) => panic!("{}", e),
        }
    } else {
        run_simulator(input);
    }
}
