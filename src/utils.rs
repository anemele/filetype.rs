use std::path::Path;
use std::{fs::File, io::Read};

use crate::constants::NUM_SIGNATURE_BYTES;

pub fn get_signature_bytes(path: &Path) -> Vec<u8> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("Unable to open file {}: {}", path.display(), e),
    };

    let mut buffer = [0_u8; NUM_SIGNATURE_BYTES];
    match file.read(&mut buffer) {
        Ok(_) => buffer.to_vec(),
        Err(e) => panic!("Unable to read file {}: {}", path.display(), e),
    }
}
