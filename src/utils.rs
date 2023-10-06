use std::path::Path;
use std::{fs::File, io::Read};

pub fn get_signature_bytes(path: &Path, mut buffer: &mut [u8]) -> bool {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Unable to open file {}: {}", path.display(), e);
            return false;
        }
    };

    // let mut buffer = [0_u8; NUM_SIGNATURE_BYTES];
    match file.read(&mut buffer) {
        Ok(_) => true,
        Err(e) => {
            eprintln!("Unable to read file {}: {}", path.display(), e);
            return false;
        }
    }
}
