use super::base::{new_type, Type, TypeMatcher, TypeTypesMatcher};
use std::collections::HashMap;

const TYPE_WOFF: Type = new_type("application/font-woff", "woff");
const TYPE_WOFF2: Type = new_type("application/font-woff", "woff2");
const TYPE_TTF: Type = new_type("application/font-sfnt", "ttf");
const TYPE_OTF: Type = new_type("application/font-sfnt", "otf");

fn is_woff(buf: &Vec<u8>) -> bool {
    buf.len() > 7
        && buf[0] == 0x77
        && buf[1] == 0x4F
        && buf[2] == 0x46
        && buf[3] == 0x46
        && buf[4] == 0x00
        && buf[5] == 0x01
        && buf[6] == 0x00
        && buf[7] == 0x00
}

fn is_woff2(buf: &Vec<u8>) -> bool {
    buf.len() > 7
        && buf[0] == 0x77
        && buf[1] == 0x4F
        && buf[2] == 0x46
        && buf[3] == 0x32
        && buf[4] == 0x00
        && buf[5] == 0x01
        && buf[6] == 0x00
        && buf[7] == 0x00
}

fn is_ttf(buf: &Vec<u8>) -> bool {
    buf.len() > 4
        && buf[0] == 0x00
        && buf[1] == 0x01
        && buf[2] == 0x00
        && buf[3] == 0x00
        && buf[4] == 0x00
}

fn is_otf(buf: &Vec<u8>) -> bool {
    buf.len() > 4
        && buf[0] == 0x4F
        && buf[1] == 0x54
        && buf[2] == 0x54
        && buf[3] == 0x4F
        && buf[4] == 0x00
}

pub fn sum() -> TypeTypesMatcher {
    let mut ret = HashMap::<Type, TypeMatcher>::new();

    // ret.insert(XXX, is_xxx);
    ret.insert(TYPE_WOFF, is_woff);
    ret.insert(TYPE_WOFF2, is_woff2);
    ret.insert(TYPE_TTF, is_ttf);
    ret.insert(TYPE_OTF, is_otf);

    ret
}
