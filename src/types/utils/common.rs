use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::{io::Cursor, str};
pub fn compare_bytes(slice: &[u8], subs: &[u8], offset: usize) -> bool {
    let s1 = subs.len();
    s1 + offset <= slice.len() && *subs == slice[offset..s1 + offset]
}

pub fn bigendian_bytes(buf: &[u8]) -> u32 {
    Cursor::new(buf).read_u32::<BigEndian>().unwrap()
}

pub fn littleendian_bytes(buf: &[u8]) -> u32 {
    Cursor::new(buf).read_u32::<LittleEndian>().unwrap()
}

pub fn bytes_to_str(buf: &[u8]) -> &str {
    match str::from_utf8(&buf) {
        Ok(s) => s,
        Err(_) => "",
    }
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
