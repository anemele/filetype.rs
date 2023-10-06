use super::base::{new_type, Type, TypeMatcher, TypeTypesMatcher};
use std::collections::HashMap;

const TYPE_MIDI: Type = new_type("audio/midi", "mid");
const TYPE_MP3: Type = new_type("audio/mpeg", "mp3");
const TYPE_M4A: Type = new_type("audio/mp4", "m4a");
const TYPE_OGG: Type = new_type("audio/ogg", "ogg");
const TYPE_FLAC: Type = new_type("audio/x-flac", "flac");
const TYPE_WAV: Type = new_type("audio/x-wav", "wav");
const TYPE_AMR: Type = new_type("audio/amr", "amr");
const TYPE_AAC: Type = new_type("audio/aac", "aac");
const TYPE_AIFF: Type = new_type("audio/x-aiff", "aiff");

fn is_midi(buf: &[u8]) -> bool {
    buf.len() > 3 && buf[0] == 0x4D && buf[1] == 0x54 && buf[2] == 0x68 && buf[3] == 0x64
}

fn is_mp3(buf: &[u8]) -> bool {
    buf.len() > 2
        && ((buf[0] == 0x49 && buf[1] == 0x44 && buf[2] == 0x33)
            || (buf[0] == 0xFF && buf[1] == 0xfb))
}

fn is_m4a(buf: &[u8]) -> bool {
    buf.len() > 10
        && ((buf[4] == 0x66
            && buf[5] == 0x74
            && buf[6] == 0x79
            && buf[7] == 0x70
            && buf[8] == 0x4D
            && buf[9] == 0x34
            && buf[10] == 0x41)
            || (buf[0] == 0x4D && buf[1] == 0x34 && buf[2] == 0x41 && buf[3] == 0x20))
}

fn is_ogg(buf: &[u8]) -> bool {
    buf.len() > 3 && buf[0] == 0x4F && buf[1] == 0x67 && buf[2] == 0x67 && buf[3] == 0x53
}

fn is_flac(buf: &[u8]) -> bool {
    buf.len() > 3 && buf[0] == 0x66 && buf[1] == 0x4C && buf[2] == 0x61 && buf[3] == 0x43
}

fn is_wav(buf: &[u8]) -> bool {
    buf.len() > 11
        && buf[0] == 0x52
        && buf[1] == 0x49
        && buf[2] == 0x46
        && buf[3] == 0x46
        && buf[8] == 0x57
        && buf[9] == 0x41
        && buf[10] == 0x56
        && buf[11] == 0x45
}

fn is_amr(buf: &[u8]) -> bool {
    buf.len() > 11
        && buf[0] == 0x23
        && buf[1] == 0x21
        && buf[2] == 0x41
        && buf[3] == 0x4D
        && buf[4] == 0x52
        && buf[5] == 0x0A
}

fn is_aac(buf: &[u8]) -> bool {
    buf.len() > 1 && ((buf[0] == 0xFF && buf[1] == 0xF1) || (buf[0] == 0xFF && buf[1] == 0xF9))
}

fn is_aiff(buf: &[u8]) -> bool {
    buf.len() > 11
        && buf[0] == 0x46
        && buf[1] == 0x4F
        && buf[2] == 0x52
        && buf[3] == 0x4D
        && buf[8] == 0x41
        && buf[9] == 0x49
        && buf[10] == 0x46
        && buf[11] == 0x46
}

pub fn sum() -> TypeTypesMatcher {
    let mut ret = HashMap::<Type, TypeMatcher>::new();

    // ret.insert(XXX, is_xxx);
    ret.insert(TYPE_MIDI, is_midi);
    ret.insert(TYPE_MP3, is_mp3);
    ret.insert(TYPE_M4A, is_m4a);
    ret.insert(TYPE_OGG, is_ogg);
    ret.insert(TYPE_FLAC, is_flac);
    ret.insert(TYPE_WAV, is_wav);
    ret.insert(TYPE_AMR, is_amr);
    ret.insert(TYPE_AAC, is_aac);
    ret.insert(TYPE_AIFF, is_aiff);

    ret
}
