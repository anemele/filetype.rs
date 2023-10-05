use filetype::match_all;
use glob::glob;
use std::env::{self};

fn main() {
    let mut args = env::args();
    let _ = args.next(); // The program itself

    for arg in args {
        // println!("{arg}");
        if let Ok(paths) = glob(&arg) {
            for entry in paths {
                if let Ok(path) = entry {
                    println!("{:?}", path.display());
                    match_all(path)
                }
            }
        }
    }
}
