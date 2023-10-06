use std::{io::Cursor, str};

use byteorder::{BigEndian, ReadBytesExt};

pub fn compare_bytes(slice: &Vec<u8>, subs: &Vec<u8>, offset: usize) -> bool {
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

fn bigendian_bytes(buf: &[u8]) -> u32 {
    Cursor::new(buf).read_u32::<BigEndian>().unwrap()
}

fn bytes_to_str(buf: &[u8]) -> &str {
    str::from_utf8(&buf).unwrap()
}

pub fn is_iso_bmf(buf: &Vec<u8>) -> bool {
    if buf.len() < 16 || bytes_to_str(&buf[4..8]) != "ftyp" {
        return false;
    }

    let int = bigendian_bytes(&buf[0..4]);
    buf.len() >= int.try_into().unwrap()
}

pub fn get_ftyp(buf: &Vec<u8>) -> (&str, &str, Vec<&str>) {
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
