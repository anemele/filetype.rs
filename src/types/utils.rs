use std::{io::Cursor, str};

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

pub fn compare_bytes(slice: &[u8], subs: &[u8], offset: usize) -> bool {
    let s1 = subs.len();
    if s1 + offset > slice.len() {
        return false;
    }

    for i in 0..s1 {
        if subs[i] != slice[i + offset] {
            return false;
        }
    }

    true
}

pub fn bigendian_bytes(buf: &[u8]) -> u32 {
    Cursor::new(buf).read_u32::<BigEndian>().unwrap()
}

pub fn littleendian_bytes(buf: &[u8]) -> u32 {
    Cursor::new(buf).read_u32::<LittleEndian>().unwrap()
}

fn bytes_to_str(buf: &[u8]) -> &str {
    match str::from_utf8(&buf) {
        Ok(s) => s,
        Err(_) => "",
    }
}

pub fn is_iso_bmf(buf: &[u8]) -> bool {
    if buf.len() < 16 || bytes_to_str(&buf[4..8]) != "ftyp" {
        return false;
    }

    let int = bigendian_bytes(&buf[0..4]);
    buf.len() >= int.try_into().unwrap()
}

pub fn get_ftyp(buf: &[u8]) -> (&str, &str, Vec<&str>) {
    let buf_len = buf.len();
    if buf_len < 17 {
        return ("", "", vec![""]);
    }

    let mut compatible_brands = vec![];
    let ftyp_length = bigendian_bytes(&buf[0..4]);
    let mut i = 16_usize;
    while i < ftyp_length as usize {
        i += 4;
        if buf_len >= i {
            compatible_brands.push(bytes_to_str(&buf[i - 4..i]));
        }
    }

    (
        bytes_to_str(&buf[8..12]),
        bytes_to_str(&buf[12..16]),
        compatible_brands,
    )
}

// try to implement bytes.Index in Go
pub fn bytes_index(buf: &[u8], subs: &[u8]) -> u32 {
    let (len1, len2) = (buf.len(), subs.len());

    if len2 == 0 {
        return 0;
    }
    if len2 > len1 {
        return u32::MAX;
    }

    for i in 0..=len1 - len2 {
        if *subs == buf[i..i + len2] {
            return i as u32;
        }
    }

    u32::MAX
}

#[test]
fn test_bytes_index() {
    assert_eq!(0, bytes_index(b"abc", b""));
    assert_eq!(0, bytes_index(b"abc", b"a"));
    assert_eq!(u32::MAX, bytes_index(b"a", b"subs"));
    assert_eq!(1, bytes_index(b"abc", b"b"));
    assert_eq!(2, bytes_index(b"abcab", b"cab"));
    assert_eq!(u32::MAX, bytes_index(b"abc", b"cd"));
}
