use super::{bigendian_bytes, bytes_to_str};

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
