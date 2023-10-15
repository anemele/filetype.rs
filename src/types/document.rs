use super::{
    base::{new_type, HashMapTypeMatcher, Type},
    utils::document::{check_odf, msooxml, TypeCode},
};

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

// doc, xls, ppt have problems
// see: https://bz.apache.org/ooo/show_bug.cgi?id=111457
// ref: https://www.zhihu.com/tardis/zm/art/51605552
const TYPE_OFFICE_UNDER_2003: Type = new_type("application/ms-office.under-2003", "doc xls ppt");

fn is_office_under_2003(buf: &[u8]) -> bool {
    buf.len() > 3 && buf[..4] == [0xD0, 0xCF, 0x11, 0xE0]
}

fn is_doc(buf: &[u8]) -> bool {
    buf.len() > 513 && buf[..4] == [0xD0, 0xCF, 0x11, 0xE0] && buf[512] == 0xEC && buf[513] == 0xA5
}

fn is_docx(buf: &[u8]) -> bool {
    let (code, ok) = msooxml(buf);
    match code {
        TypeCode::CodeDOCX => ok,
        _ => false,
    }
}

fn is_xls(buf: &[u8]) -> bool {
    buf.len() > 513 && buf[..4] == [0xD0, 0xCF, 0x11, 0xE0] && buf[512] == 0x09 && buf[513] == 0x08
}

fn is_xlsx(buf: &[u8]) -> bool {
    let (code, ok) = msooxml(buf);
    match code {
        TypeCode::CodeXLSX => ok,
        _ => false,
    }
}

fn is_ppt(buf: &[u8]) -> bool {
    buf.len() > 513 && buf[..4] == [0xD0, 0xCF, 0x11, 0xE0] && buf[512] == 0xA0 && buf[513] == 0x46
}

fn is_pptx(buf: &[u8]) -> bool {
    let (code, ok) = msooxml(buf);
    match code {
        TypeCode::CodePPTX => ok,
        _ => false,
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

pub fn sum() -> HashMapTypeMatcher {
    let mut ret = HashMapTypeMatcher::new();

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
    ret.insert(TYPE_OFFICE_UNDER_2003, is_office_under_2003);

    ret
}
