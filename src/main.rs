#![feature(path, io)]
#![cfg(not(test))]

extern crate hack_interpreter;
extern crate "rustc-serialize" as rustc_serialize;
extern crate docopt;
extern crate drawille;

use std::sync::mpsc::{channel, TryRecvError};
use std::thread;
use std::io::{stdin, Read};
use std::path::AsPath;
use hack_interpreter::{Rom, Cpu, runner};
use docopt::Docopt;
use drawille::braille::Canvas;


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
        render_simulation(input);
    }
}

fn render_simulation(input: &String) {
    println!("Running program file: '{}'", input);

    let program = Rom::from_file(input).unwrap();
    let mut cpu = Cpu::new(program);

    let (tx, rx) = channel();

    // Place stdin on other channel
    thread::spawn(move|| {
        let mut input = stdin();

        loop {
            // Read the keyboard
            let c = &mut [0; 2];
            let bytes_read = input.read(c).unwrap();

            tx.send((c[0] as u16) << 8 | c[1] as u16).unwrap();
        }
    });

    // Memory starts at 16384
    // The screen has resolution 512 x 256
    // Each bit of the screen represents
    const WIDTH: usize = 512;
    const HEIGHT: usize = 256;
    const VIDEO_MEM_BYTES: usize = (WIDTH * HEIGHT) / 16;
    let mut canvas = Canvas::new(WIDTH, HEIGHT);

    for steps in (0..) {
        cpu.step();

        if steps % 10000000 == 0 {
            println!("{}", canvas.frame());

            cpu.ram[24576] = match rx.try_recv() {
                Ok(key) => key,
                Err(TryRecvError::Empty) => 0,
                Err(TryRecvError::Disconnected) => panic!("Stdio thread died")
            };

            let video_mem = &cpu.ram[16384..16384 + VIDEO_MEM_BYTES];

            for (idx, word) in video_mem.iter().enumerate() {
                // For each word of memory, draw 16 pixels
                let idx = idx * 16;
                for (bit, i) in (idx..idx+16).enumerate() {
                    let (x, y) = (i % WIDTH, i / WIDTH);
                    if word & (1 << bit) != 0 {
                        canvas.set(x, y);
                    } else {
                        canvas.unset(x, y);
                    }
                }
            }
        }
    }
}
