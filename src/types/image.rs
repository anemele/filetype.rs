use std::collections::HashMap;

use super::base::{new_type, Type};

const JPEG: Type = new_type("image/jpeg", "jpg");
const JPEG2000: Type = new_type("image/jp2", "jp2");
const PNG: Type = new_type("image/png", "png");
const GIF: Type = new_type("image/gif", "gif");
const WEBP: Type = new_type("image/webp", "webp");
const CR2: Type = new_type("image/x-canon-cr2", "cr2");
const TIFF: Type = new_type("image/tiff", "tif");
const BMP: Type = new_type("image/bmp", "bmp");
const JXR: Type = new_type("image/vnd.ms-photo", "jxr");
const PSD: Type = new_type("image/vnd.adobe.photosh", "psd");
const ICO: Type = new_type("image/vnd.microsoft.ico", "ico");
const HEIF: Type = new_type("image/heif", "heif");
const DWG: Type = new_type("image/vnd.dwg", "dwg");
const EXR: Type = new_type("image/x-exr", "exr");
const AVIF: Type = new_type("image/avif", "avif");

fn is_jpeg(buf: Vec<u8>) -> bool {
    buf.len() > 2 && buf[0] == 0xFF && buf[1] == 0xD8 && buf[2] == 0xFF
}

fn is_jpeg2000(buf: Vec<u8>) -> bool {
    buf.len() > 12
        && buf[0] == 0x0
        && buf[1] == 0x0
        && buf[2] == 0x0
        && buf[3] == 0xC
        && buf[4] == 0x6A
        && buf[5] == 0x50
        && buf[6] == 0x20
        && buf[7] == 0x20
        && buf[8] == 0xD
        && buf[9] == 0xA
        && buf[10] == 0x87
        && buf[11] == 0xA
        && buf[12] == 0x0
}

fn is_png(buf: Vec<u8>) -> bool {
    buf.len() > 3 && buf[0] == 0x89 && buf[1] == 0x50 && buf[2] == 0x4E && buf[3] == 0x47
}

fn is_gif(buf: Vec<u8>) -> bool {
    buf.len() > 2 && buf[0] == 0x47 && buf[1] == 0x49 && buf[2] == 0x46
}

fn is_webp(buf: Vec<u8>) -> bool {
    buf.len() > 11 && buf[8] == 0x57 && buf[9] == 0x45 && buf[10] == 0x42 && buf[11] == 0x50
}

fn is_cr2(buf: Vec<u8>) -> bool {
    buf.len()> 10 &&
    ((buf[0] == 0x49 && buf[1] == 0x49 && buf[2] == 0x2A && buf[3] == 0x0) || // Little Endian
        (buf[0] == 0x4D && buf[1] == 0x4D && buf[2] == 0x0 && buf[3] == 0x2A)) && // Big Endian
    buf[8] == 0x43 && buf[9] == 0x52 && // CR2 magic word
    buf[10] == 0x02 // CR2 major version
}

fn is_tiff(buf: Vec<u8>) -> bool {
    buf.len() > 10 &&
    ((buf[0] == 0x49 && buf[1] == 0x49 && buf[2] == 0x2A && buf[3] == 0x0) || // Little Endian
        (buf[0] == 0x4D && buf[1] == 0x4D && buf[2] == 0x0 && buf[3] == 0x2A)) && // Big Endian
    !is_cr2(buf) // To avoid conflicts differentiate Tiff from CR2
}

fn is_bmp(buf: Vec<u8>) -> bool {
    buf.len() > 1 && buf[0] == 0x42 && buf[1] == 0x4D
}

fn is_jxr(buf: Vec<u8>) -> bool {
    buf.len() > 2 && buf[0] == 0x49 && buf[1] == 0x49 && buf[2] == 0xBC
}

fn is_psd(buf: Vec<u8>) -> bool {
    buf.len() > 3 && buf[0] == 0x38 && buf[1] == 0x42 && buf[2] == 0x50 && buf[3] == 0x53
}

fn is_ico(buf: Vec<u8>) -> bool {
    buf.len() > 3 && buf[0] == 0x00 && buf[1] == 0x00 && buf[2] == 0x01 && buf[3] == 0x00
}

fn is_heif(buf: Vec<u8>) -> bool {
    todo!();
}

fn is_dwg(buf: Vec<u8>) -> bool {
    buf.len() > 3 && buf[0] == 0x41 && buf[1] == 0x43 && buf[2] == 0x31 && buf[3] == 0x30
}

fn is_exr(buf: Vec<u8>) -> bool {
    buf.len() > 3 && buf[0] == 0x76 && buf[1] == 0x2f && buf[2] == 0x31 && buf[3] == 0x01
}

fn is_avif(buf: Vec<u8>) -> bool {
    todo!();
}

pub fn sum() -> HashMap<Type<'static>, fn(Vec<u8>) -> bool> {
    let mut ret = HashMap::<Type, fn(Vec<u8>) -> bool>::new();

    ret.insert(JPEG, is_jpeg);
    ret.insert(JPEG2000, is_jpeg2000);
    ret.insert(PNG, is_png);
    ret.insert(GIF, is_gif);
    ret.insert(WEBP, is_webp);
    ret.insert(CR2, is_cr2);
    ret.insert(TIFF, is_tiff);
    ret.insert(BMP, is_bmp);
    ret.insert(JXR, is_jxr);
    ret.insert(PSD, is_psd);
    ret.insert(ICO, is_ico);
    ret.insert(HEIF, is_heif);
    ret.insert(DWG, is_dwg);
    ret.insert(EXR, is_exr);
    ret.insert(AVIF, is_avif);

    ret
}
