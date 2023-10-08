use super::{
    base::{new_type, HashMapTypeMatcher, Type},
    utils::compare_bytes,
};

const TYPE_WASM: Type = new_type("application/wasm", "wasm");
const TYPE_DEX: Type = new_type("application/vnd.android.dex", "dex");
const TYPE_DEY: Type = new_type("application/vnd.android.dey", "dey");

fn is_wasm(buf: &[u8]) -> bool {
    let signatures = [0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00];
    compare_bytes(buf, &signatures, 0)
}

fn is_dex(buf: &[u8]) -> bool {
    buf.len() > 36 &&
		// magic
		buf[..4] ==[0x64, 0x65 , 0x78 , 0x0A ]&&
		// file sise
		buf[36] == 0x70
}

fn is_dey(buf: &[u8]) -> bool {
    buf.len() > 100 &&
    // dey magic
    buf[..4] == [0x64 , 0x65, 0x79, 0x0A] &&
    // dex
    is_dex(&buf[40..100].to_vec())
}

pub fn sum() -> HashMapTypeMatcher {
    let mut ret = HashMapTypeMatcher::new();

    ret.insert(TYPE_WASM, is_wasm);
    ret.insert(TYPE_DEX, is_dex);
    ret.insert(TYPE_DEY, is_dey);

    ret
}
