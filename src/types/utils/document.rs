use super::{bytes_index, compare_bytes, littleendian_bytes};

pub enum TypeCode {
    CodeNone = -1,
    _CodeDOC = 0,
    CodeDOCX,
    _CodeXLS,
    CodeXLSX,
    _CodePPT,
    CodePPTX,
    CodeOOXML,
    _CodeODP,
    _CodeODS,
    _CodeODT,
}

pub fn msooxml(buf: &[u8]) -> (TypeCode, bool) {
    let ret = (TypeCode::CodeNone, false);
    let signature = [b'P', b'K', 0x03, 0x04];
    if !compare_bytes(buf, &signature, 0) {
        return ret;
    }

    let (code, ok) = check_msooml(buf, 0x1E);
    if ok {
        return (code, ok);
    }

    if !compare_bytes(buf, b"[Content_Types].xml", 0x1E)
        && !compare_bytes(buf, b"_rels/.rels", 0x1E)
        && !compare_bytes(buf, b"docProps", 0x1E)
        && !compare_bytes(buf, b"_rels", 0x1E)
    {
        return ret;
    }

    let mut start_offset = littleendian_bytes(&buf[18..22]) + 49;
    let index = search(buf, start_offset, 6000);
    if index == u32::MAX {
        return ret;
    }

    start_offset += index + 4 + 26;
    let index = search(buf, start_offset, 6000);
    if index == u32::MAX {
        return ret;
    }

    start_offset += index + 4 + 26;
    let (code, ok) = check_msooml(buf, start_offset as usize);
    if ok {
        return (code, ok);
    }

    start_offset += 26;
    let index = search(buf, start_offset, 6000);
    if index == u32::MAX {
        return (TypeCode::CodeOOXML, true);
    }

    start_offset += index + 4 + 26;
    let (code, ok) = check_msooml(buf, start_offset as usize);
    if ok {
        return (code, ok);
    }

    (TypeCode::CodeOOXML, true)
}

fn check_msooml(buf: &[u8], offset: usize) -> (TypeCode, bool) {
    if compare_bytes(buf, b"word/", offset) {
        (TypeCode::CodeDOCX, true)
    } else if compare_bytes(buf, b"ppt/", offset) {
        (TypeCode::CodePPTX, true)
    } else if compare_bytes(buf, b"xl/", offset) {
        (TypeCode::CodeXLSX, true)
    } else {
        (TypeCode::CodeNone, false)
    }
}

fn search(buf: &[u8], start: u32, range_num: u32) -> u32 {
    let end = (start + range_num).min(buf.len() as u32);
    if start >= end {
        return u32::MAX;
    }

    let signature = [b'P', b'K', 0x03, 0x04];
    bytes_index(&buf[start as usize..end as usize], &signature)
}

// https://en.wikipedia.org/wiki/OpenDocument_technical_specification
// https://en.wikipedia.org/wiki/ZIP_(file_format)
pub fn check_odf(buf: &[u8], mimetype: &str) -> bool {
    if 38 + mimetype.len() > buf.len() {
        return false;
    }
    // Perform all byte checks first for better performance
    // Check ZIP start
    if buf[0] != b'P' || buf[1] != b'K' || buf[2] != 3 || buf[3] != 4 {
        return false;
    }
    // Now check the first file data
    // Compression method: not compressed
    if buf[8] != 0 || buf[9] != 0 {
        return false;
    }
    // Filename length must be 8 for "mimetype"
    if buf[26] != 8 || buf[27] != 0 {
        return false;
    }
    // Check the file contents sizes
    if buf[18] as usize != mimetype.len()
        || buf[19] != 0
        || buf[20] != 0
        || buf[21] != 0
        || buf[22] as usize != mimetype.len()
        || buf[23] != 0
        || buf[24] != 0
        || buf[25] != 0
    {
        return false;
    }
    // No extra field (for data offset below)
    if buf[28] != 0 || buf[29] != 0 {
        return false;
    }
    // Finally check the file name and contents
    return buf[30..38] == *b"mimetype" && buf[38..38 + mimetype.len()] == *mimetype.as_bytes();
}
