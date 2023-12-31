use filetypes::matcher::match_all;
use glob::glob;
use std::env;
use std::path::MAIN_SEPARATOR_STR;

fn main() {
    let mut args = env::args();
    let prog = args.next().unwrap();
    let prog = match prog.rsplit_once(MAIN_SEPARATOR_STR) {
        Some(p) => p.1,
        None => prog.as_str(),
    }; // The program itself

    if args.len() == 0 {
        println!(
            "Determine file type via magic numbers, wildcards support.\nUsage: {} <FILE>...",
            prog
        );
        return;
    }

    for arg in args {
        // println!("{arg}");
        if let Ok(paths) = glob(&arg) {
            for entry in paths {
                if let Ok(path) = entry {
                    if path.is_file() {
                        let t = match_all(path.as_path());
                        println!("{}: {}", path.display(), t);
                    }
                }
            }
        }
    }
}
