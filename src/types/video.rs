use super::{
    base::{new_type, Type, TypeMatcher, TypeTypesMatcher},
    utils::bytes_index,
};
use std::collections::HashMap;

const TYPE_MP4: Type = new_type("video/mp4", "mp4");
const TYPE_M4V: Type = new_type("video/x-m4v", "m4v");
const TYPE_MKV: Type = new_type("video/x-matroska", "mkv");
const TYPE_WEBM: Type = new_type("video/webm", "webm");
const TYPE_MOV: Type = new_type("video/quicktime", "mov");
const TYPE_AVI: Type = new_type("video/x-msvideo", "avi");
const TYPE_WMV: Type = new_type("video/x-ms-wmv", "wmv");
const TYPE_MPG: Type = new_type("video/mpeg", "mpg");
const TYPE_FLV: Type = new_type("video/x-flv", "flv");
const TYPE_3GP: Type = new_type("video/3gpp", "3gp");

fn is_m4v(buf: &[u8]) -> bool {
    buf.len() > 10
        && buf[4] == 0x66
        && buf[5] == 0x74
        && buf[6] == 0x79
        && buf[7] == 0x70
        && buf[8] == 0x4D
        && buf[9] == 0x34
        && buf[10] == 0x56
}

fn is_mkv(buf: &[u8]) -> bool {
    buf.len() > 3
        && buf[0] == 0x1A
        && buf[1] == 0x45
        && buf[2] == 0xDF
        && buf[3] == 0xA3
        && contains_matroska_signature(buf, b"matroska")
}

fn is_webm(buf: &[u8]) -> bool {
    buf.len() > 3
        && buf[0] == 0x1A
        && buf[1] == 0x45
        && buf[2] == 0xDF
        && buf[3] == 0xA3
        && contains_matroska_signature(buf, b"webm")
}

fn is_mov(buf: &[u8]) -> bool {
    buf.len() > 15
        && ((buf[0] == 0x0 && buf[1] == 0x0 &&
    buf[2] == 0x0 && (buf[3] == 0x14 || buf[3] == 0x20) &&
    buf[4] == 0x66 && buf[5] == 0x74 && // b'f' b't'
    buf[6] == 0x79 && buf[7] == 0x70 && // b'y' b'p'
    buf[8] == 0x71 && buf[9] == 0x74) || // b'q' b't'
    (buf[4] == 0x6d && buf[5] == 0x6f && buf[6] == 0x6f && buf[7] == 0x76) ||
    (buf[4] == 0x6d && buf[5] == 0x64 && buf[6] == 0x61 && buf[7] == 0x74) ||
    (buf[12] == 0x6d && buf[13] == 0x64 && buf[14] == 0x61 && buf[15] == 0x74))
}

fn is_avi(buf: &[u8]) -> bool {
    buf.len() > 10
        && buf[0] == 0x52
        && buf[1] == 0x49
        && buf[2] == 0x46
        && buf[3] == 0x46
        && buf[8] == 0x41
        && buf[9] == 0x56
        && buf[10] == 0x49
}

fn is_wmv(buf: &[u8]) -> bool {
    buf.len() > 9
        && buf[0] == 0x30
        && buf[1] == 0x26
        && buf[2] == 0xB2
        && buf[3] == 0x75
        && buf[4] == 0x8E
        && buf[5] == 0x66
        && buf[6] == 0xCF
        && buf[7] == 0x11
        && buf[8] == 0xA6
        && buf[9] == 0xD9
}

fn is_mpeg(buf: &[u8]) -> bool {
    buf.len() > 3
        && buf[0] == 0x0
        && buf[1] == 0x0
        && buf[2] == 0x1
        && buf[3] >= 0xb0
        && buf[3] <= 0xbf
}

fn is_flv(buf: &[u8]) -> bool {
    buf.len() > 3 && buf[0] == 0x46 && buf[1] == 0x4C && buf[2] == 0x56 && buf[3] == 0x01
}

fn is_mp4(buf: &[u8]) -> bool {
    buf.len() > 11
        && (buf[4] == b'f' && buf[5] == b't' && buf[6] == b'y' && buf[7] == b'p')
        && ((buf[8] == b'a' && buf[9] == b'v' && buf[10] == b'c' && buf[11] == b'1')
            || (buf[8] == b'd' && buf[9] == b'a' && buf[10] == b's' && buf[11] == b'h')
            || (buf[8] == b'i' && buf[9] == b's' && buf[10] == b'o' && buf[11] == b'2')
            || (buf[8] == b'i' && buf[9] == b's' && buf[10] == b'o' && buf[11] == b'3')
            || (buf[8] == b'i' && buf[9] == b's' && buf[10] == b'o' && buf[11] == b'4')
            || (buf[8] == b'i' && buf[9] == b's' && buf[10] == b'o' && buf[11] == b'5')
            || (buf[8] == b'i' && buf[9] == b's' && buf[10] == b'o' && buf[11] == b'6')
            || (buf[8] == b'i' && buf[9] == b's' && buf[10] == b'o' && buf[11] == b'm')
            || (buf[8] == b'm' && buf[9] == b'm' && buf[10] == b'p' && buf[11] == b'4')
            || (buf[8] == b'm' && buf[9] == b'p' && buf[10] == b'4' && buf[11] == b'1')
            || (buf[8] == b'm' && buf[9] == b'p' && buf[10] == b'4' && buf[11] == b'2')
            || (buf[8] == b'm' && buf[9] == b'p' && buf[10] == b'4' && buf[11] == b'v')
            || (buf[8] == b'm' && buf[9] == b'p' && buf[10] == b'7' && buf[11] == b'1')
            || (buf[8] == b'M' && buf[9] == b'S' && buf[10] == b'N' && buf[11] == b'V')
            || (buf[8] == b'N' && buf[9] == b'D' && buf[10] == b'A' && buf[11] == b'S')
            || (buf[8] == b'N' && buf[9] == b'D' && buf[10] == b'S' && buf[11] == b'C')
            || (buf[8] == b'N' && buf[9] == b'S' && buf[10] == b'D' && buf[11] == b'C')
            || (buf[8] == b'N' && buf[9] == b'D' && buf[10] == b'S' && buf[11] == b'H')
            || (buf[8] == b'N' && buf[9] == b'D' && buf[10] == b'S' && buf[11] == b'M')
            || (buf[8] == b'N' && buf[9] == b'D' && buf[10] == b'S' && buf[11] == b'P')
            || (buf[8] == b'N' && buf[9] == b'D' && buf[10] == b'S' && buf[11] == b'S')
            || (buf[8] == b'N' && buf[9] == b'D' && buf[10] == b'X' && buf[11] == b'C')
            || (buf[8] == b'N' && buf[9] == b'D' && buf[10] == b'X' && buf[11] == b'H')
            || (buf[8] == b'N' && buf[9] == b'D' && buf[10] == b'X' && buf[11] == b'M')
            || (buf[8] == b'N' && buf[9] == b'D' && buf[10] == b'X' && buf[11] == b'P')
            || (buf[8] == b'N' && buf[9] == b'D' && buf[10] == b'X' && buf[11] == b'S')
            || (buf[8] == b'F' && buf[9] == b'4' && buf[10] == b'V' && buf[11] == b' ')
            || (buf[8] == b'F' && buf[9] == b'4' && buf[10] == b'P' && buf[11] == b' '))
}

fn is_3gp(buf: &[u8]) -> bool {
    buf.len() > 10
        && buf[4] == 0x66
        && buf[5] == 0x74
        && buf[6] == 0x79
        && buf[7] == 0x70
        && buf[8] == 0x33
        && buf[9] == 0x67
        && buf[10] == 0x70
}

fn contains_matroska_signature(buf: &[u8], subt: &[u8]) -> bool {
    let limit = buf.len().min(4096);
    let index = bytes_index(&buf[..limit], subt);

    index >= 3 && buf[index as usize - 3] == 0x42 && buf[index as usize - 2] == 0x82
}

pub fn sum() -> TypeTypesMatcher {
    let mut ret = HashMap::<Type, TypeMatcher>::new();

    // ret.insert(XXX, is_xxx);
    ret.insert(TYPE_MP4, is_mp4);
    ret.insert(TYPE_M4V, is_m4v);
    ret.insert(TYPE_MKV, is_mkv);
    ret.insert(TYPE_WEBM, is_webm);
    ret.insert(TYPE_MOV, is_mov);
    ret.insert(TYPE_AVI, is_avi);
    ret.insert(TYPE_WMV, is_wmv);
    ret.insert(TYPE_MPG, is_mpeg);
    ret.insert(TYPE_FLV, is_flv);
    ret.insert(TYPE_3GP, is_3gp);

    ret
}
