use std::collections::HashMap;

use super::base::{new_type, Type};

const WASM: Type = new_type("application/wasm", "wasm");
const DEX: Type = new_type("application/vnd.android.dex", "dex");
const DEY: Type = new_type("application/vnd.android.dey", "dey");

fn is_wasm(buf: Vec<u8>) -> bool {
    buf.len() >= 8
        && buf[0] == 0x00
        && buf[1] == 0x61
        && buf[2] == 0x73
        && buf[3] == 0x6D
        && buf[4] == 0x01
        && buf[5] == 0x00
        && buf[6] == 0x00
        && buf[7] == 0x00
}

fn is_dex(buf: Vec<u8>) -> bool {
    buf.len() > 36 &&
		// magic
		buf[0] == 0x64 && buf[1] == 0x65 && buf[2] == 0x78 && buf[3] == 0x0A &&
		// file sise
		buf[36] == 0x70
}

fn is_dey(buf: Vec<u8>) -> bool {
    buf.len() > 100 &&
    // dey magic
    buf[0] == 0x64 && buf[1] == 0x65 && buf[2] == 0x79 && buf[3] == 0x0A &&
    // dex
    is_dex(buf[40..100].to_vec())
}

pub fn sum() -> HashMap<Type<'static>, fn(Vec<u8>) -> bool> {
    let mut ret = HashMap::<Type, fn(Vec<u8>) -> bool>::new();

    ret.insert(WASM, is_wasm);
    ret.insert(DEX, is_dex);
    ret.insert(DEY, is_dey);

    ret
}