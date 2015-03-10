extern crate hack_interpreter;
extern crate glob;

use std::thread;
use glob::glob;
use hack_interpreter::runner;

#[test]
fn run_tst_files() {
    let files = glob("tests/data/*.tst").unwrap();
    let guards : Vec<_> = files.map(|f|
        thread::scoped(|| runner(&f.unwrap()))
    ).collect();

    for guard in guards.into_iter() {
        match guard.join() {
            Ok(()) => {},
            Err(e) => panic!("{}", e)
        }
    }
}
