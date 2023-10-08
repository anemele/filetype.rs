use super::{
    base::{new_type, HashMapTypeMatcher, Type},
    utils::{bytes_index, compare_bytes},
};

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
    let subs = [0x30, 0x26, 0xB2, 0x75, 0x8E, 0x66, 0xCF, 0x11, 0xA6, 0xD9];
    compare_bytes(buf, &subs, 0)
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
    compare_bytes(buf, &[0x46, 0x4C, 0x56, 0x01], 0)
}

fn is_mp4(buf: &[u8]) -> bool {
    buf.len() > 11
        && (buf[4..8] == *b"ftyp")
        && ((buf[8..12] == *b"avc1")
            || (buf[8..12] == *b"dash")
            || (buf[8..12] == *b"iso2")
            || (buf[8..12] == *b"iso3")
            || (buf[8..12] == *b"iso4")
            || (buf[8..12] == *b"iso5")
            || (buf[8..12] == *b"iso6")
            || (buf[8..12] == *b"isom")
            || (buf[8..12] == *b"mmp4")
            || (buf[8..12] == *b"mp41")
            || (buf[8..12] == *b"mp42")
            || (buf[8..12] == *b"mp4v")
            || (buf[8..12] == *b"mp71")
            || (buf[8..12] == *b"MSNV")
            || (buf[8..12] == *b"NDAS")
            || (buf[8..12] == *b"NDSC")
            || (buf[8..12] == *b"NSDC")
            || (buf[8..12] == *b"NDSH")
            || (buf[8..12] == *b"NDSM")
            || (buf[8..12] == *b"NDSP")
            || (buf[8..12] == *b"NDSS")
            || (buf[8..12] == *b"NDXC")
            || (buf[8..12] == *b"NDXH")
            || (buf[8..12] == *b"NDXM")
            || (buf[8..12] == *b"NDXP")
            || (buf[8..12] == *b"NDXS")
            || (buf[8..12] == *b"F4V ")
            || (buf[8..12] == *b"F4P "))
}

fn is_3gp(buf: &[u8]) -> bool {
    let subs = [0x66, 0x74, 0x79, 0x70, 0x33, 0x67, 0x70];
    compare_bytes(buf, &subs, 4)
}

fn contains_matroska_signature(buf: &[u8], subt: &[u8]) -> bool {
    let limit = buf.len().min(4096);
    let index = bytes_index(&buf[..limit], subt);

    index >= 3 && buf[index as usize - 3] == 0x42 && buf[index as usize - 2] == 0x82
}

pub fn sum() -> HashMapTypeMatcher {
    let mut ret = HashMapTypeMatcher::new();

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
