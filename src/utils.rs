use std::{
    fs::File,
    io::Read,
};

use crate::enums::InputType;

const NUM_SIGNATURE_BYTES: usize = 8192;

pub fn get_signature_bytes(path: &str) -> Vec<u8> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => panic!("Unable to open file {}: {}", path, e),
    };

    let mut buffer = [0_u8; NUM_SIGNATURE_BYTES];
    match file.read_exact(&mut buffer) {
        Ok(_) => buffer.to_vec(),
        Err(e) => panic!("Unable to read file {}: {}", path, e),
    }
}

pub fn signature(array: Vec<u8>) -> Vec<u8> {
    let index = array.len().min(NUM_SIGNATURE_BYTES);
    array[..index].to_vec()
}

pub fn get_bytes(obj: InputType) -> Vec<u8> {
    match obj {
        InputType::File(file) => get_signature_bytes(file),
        InputType::Bytes(bytes) => signature(bytes),
    }
}
