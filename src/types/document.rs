use super::{
    base::{new_type, Type, TypeMatcher, TypeTypesMatcher},
    utils::compare_bytes,
};
use std::collections::HashMap;

enum TypeCode {
    CodeNone = -1,
    _CodeDOC = 0,
    CodeDOCX,
    _CodeXLS,
    CodeXLSX,
    _CodePPT,
    CodePPTX,
    _CodeOOXML,
    _CodeODP,
    _CodeODS,
    _CodeODT,
}

const TYPE_DOC: Type = new_type("application/msword", "doc");
const TYPE_DOCX: Type = new_type(
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    "docx",
);
const TYPE_XLS: Type = new_type("application/vnd.ms-excel", "xls");
const TYPE_XLSX: Type = new_type(
    "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    "xlsx",
);
const TYPE_PPT: Type = new_type("application/vnd.ms-powerpoint", "ppt");
const TYPE_PPTX: Type = new_type(
    "application/vnd.openxmlformats-officedocument.presentationml.presentation",
    "pptx",
);
const TYPE_ODP: Type = new_type("application/vnd.oasis.opendocument.presentation", "odp");
const TYPE_ODS: Type = new_type("application/vnd.oasis.opendocument.spreadsheet", "ods");
const TYPE_ODT: Type = new_type("application/vnd.oasis.opendocument.text", "odt");

fn is_doc(buf: &[u8]) -> bool {
    (buf.len() > 513
        && buf[0] == 0xD0
        && buf[1] == 0xCF
        && buf[2] == 0x11
        && buf[3] == 0xE0
        && buf[512] == 0xEC
        && buf[513] == 0xA5)
        || (buf.len() > 3 && buf[0] == 0xD0 && buf[1] == 0xCF && buf[2] == 0x11 && buf[3] == 0xE0)
}

fn is_docx(buf: &[u8]) -> bool {
    let (code, ok) = msooxml(buf);
    match code {
        TypeCode::CodeDOCX => ok,
        _ => false,
    }
}

fn is_xls(buf: &[u8]) -> bool {
    (buf.len() > 513
        && buf[0] == 0xD0
        && buf[1] == 0xCF
        && buf[2] == 0x11
        && buf[3] == 0xE0
        && buf[512] == 0x09
        && buf[513] == 0x08)
        || (buf.len() > 3 && buf[0] == 0xD0 && buf[1] == 0xCF && buf[2] == 0x11 && buf[3] == 0xE0)
}

fn is_xlsx(buf: &[u8]) -> bool {
    let (code, ok) = msooxml(buf);
    match code {
        TypeCode::CodeXLSX => ok,
        _ => false,
    }
}

fn is_ppt(buf: &[u8]) -> bool {
    (buf.len() > 513
        && buf[0] == 0xD0
        && buf[1] == 0xCF
        && buf[2] == 0x11
        && buf[3] == 0xE0
        && buf[512] == 0xA0
        && buf[513] == 0x46)
        || (buf.len() > 3 && buf[0] == 0xD0 && buf[1] == 0xCF && buf[2] == 0x11 && buf[3] == 0xE0)
}

fn is_pptx(buf: &[u8]) -> bool {
    let (code, ok) = msooxml(buf);
    match code {
        TypeCode::CodePPTX => ok,
        _ => false,
    }
}

fn msooxml(buf: &[u8]) -> (TypeCode, bool) {
    let signature = vec![b'P', b'K', 0x03, 0x04];
    if !compare_bytes(buf, &signature, 0) {
        return (TypeCode::CodeNone, false);
    }

    let (code, ok) = check_msooml(buf, 0x1E);
    if ok {
        return (code, ok);
    }

    if !compare_bytes(buf, &b"[Content_Types].xml".to_vec(), 0x1E)
        && !compare_bytes(buf, &b"_rels/.rels".to_vec(), 0x1E)
        && !compare_bytes(buf, &b"docProps".to_vec(), 0x1E)
        && !compare_bytes(buf, &b"_rels".to_vec(), 0x1E)
    {
        return (TypeCode::CodeNone, false);
    }

    (TypeCode::CodeNone, false)
}

fn check_msooml(buf: &[u8], offset: usize) -> (TypeCode, bool) {
    if compare_bytes(buf, &b"word/".to_vec(), offset) {
        (TypeCode::CodeDOCX, true)
    } else if compare_bytes(buf, &b"ppt/".to_vec(), offset) {
        (TypeCode::CodePPTX, true)
    } else if compare_bytes(buf, &b"xl/".to_vec(), offset) {
        (TypeCode::CodeXLSX, true)
    } else {
        (TypeCode::CodeNone, false)
    }
}

fn is_odp(buf: &[u8]) -> bool {
    check_odf(buf, TYPE_ODP.mime)
}

fn is_ods(buf: &[u8]) -> bool {
    check_odf(buf, TYPE_ODS.mime)
}

fn is_odt(buf: &[u8]) -> bool {
    check_odf(buf, TYPE_ODT.mime)
}

// https://en.wikipedia.org/wiki/OpenDocument_technical_specification
// https://en.wikipedia.org/wiki/ZIP_(file_format)
fn check_odf(buf: &[u8], mimetype: &str) -> bool {
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
    return buf[30..38] == b"mimetype".to_vec()
        && buf[38..38 + mimetype.len()] == *mimetype.as_bytes();
}

pub fn sum() -> TypeTypesMatcher {
    let mut ret = HashMap::<Type, TypeMatcher>::new();

    // ret.insert(XXX, is_xxx);
    ret.insert(TYPE_DOC, is_doc);
    ret.insert(TYPE_DOCX, is_docx);
    ret.insert(TYPE_XLS, is_xls);
    ret.insert(TYPE_XLSX, is_xlsx);
    ret.insert(TYPE_PPT, is_ppt);
    ret.insert(TYPE_PPTX, is_pptx);
    ret.insert(TYPE_ODP, is_odp);
    ret.insert(TYPE_ODS, is_ods);
    ret.insert(TYPE_ODT, is_odt);

    ret
}
