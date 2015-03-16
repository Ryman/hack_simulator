#![feature(io, path)]

extern crate hack_assembler;
extern crate glob;

use std::fs::File;
use std::thread;
use std::io::Read;
use glob::glob;
use hack_assembler::*;

#[test]
fn assembles_expected_hack() {
    let files = glob("tests/data/*.asm").unwrap();
    let guards : Vec<_> = files.map(|f|
        thread::scoped(|| {
            let ref mut s = String::new();
            let filename = f.unwrap();
            File::open(&filename)
                 .and_then(|mut f| f.read_to_string(s))
                 .map(|_| assemble(s).unwrap())
                 .map(|generated| {
                    let ref mut expected = String::new();
                    File::open(&filename.with_extension("hack"))
                         .and_then(|mut f| f.read_to_string(expected))
                         .unwrap();

                    assert_eq!(generated, *expected);
                 })
        })
    ).collect();

    for guard in guards.into_iter() {
        match guard.join() {
            Ok(()) => {},
            Err(e) => panic!("{}", e)
        }
    }
}
