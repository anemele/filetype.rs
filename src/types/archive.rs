use super::{
    base::{new_type, HashMapTypeMatcher, Type},
    utils::{compare_bytes, littleendian_bytes},
};

const TYPE_EPUB: Type = new_type("application/epub+zip", "epub");
const TYPE_ZIP: Type = new_type("application/zip", "zip");
const TYPE_TAR: Type = new_type("application/x-tar", "tar");
const TYPE_RAR: Type = new_type("application/vnd.rar", "rar");
const TYPE_GZ: Type = new_type("application/gzip", "gz");
const TYPE_BZ2: Type = new_type("application/x-bzip2", "bz2");
const TYPE_7Z: Type = new_type("application/x-7z-compressed", "7z");
const TYPE_XZ: Type = new_type("application/x-xz", "xz");
const TYPE_ZST: Type = new_type("application/zstd", "zst");
const TYPE_PDF: Type = new_type("application/pdf", "pdf");
const TYPE_EXE: Type = new_type("application/vnd.microsoft.portable-executable", "exe");
const TYPE_SWF: Type = new_type("application/x-shockwave-flash", "swf");
const TYPE_RTF: Type = new_type("application/rtf", "rtf");
const TYPE_EOT: Type = new_type("application/octet-stream", "eot");
const TYPE_PS: Type = new_type("application/postscript", "ps");
const TYPE_SQLITE: Type = new_type("application/vnd.sqlite3", "sqlite");
const TYPE_NES: Type = new_type("application/x-nintendo-nes-rom", "nes");
const TYPE_CRX: Type = new_type("application/x-google-chrome-extension", "crx");
const TYPE_CAB: Type = new_type("application/vnd.ms-cab-compressed", "cab");
const TYPE_DEB: Type = new_type("application/vnd.debian.binary-package", "deb");
const TYPE_AR: Type = new_type("application/x-unix-archive", "ar");
const TYPE_Z: Type = new_type("application/x-compress", "Z");
const TYPE_LZ: Type = new_type("application/x-lzip", "lz");
const TYPE_RPM: Type = new_type("application/x-rpm", "rpm");
const TYPE_ELF: Type = new_type("application/x-executable", "elf");
const TYPE_DCM: Type = new_type("application/dicom", "dcm");
const TYPE_ISO: Type = new_type("application/x-iso9660-image", "iso");
const TYPE_MACHO: Type = new_type("application/x-mach-binary", "macho"); // Mach-O binaries have no common extension.

fn is_epub(buf: &[u8]) -> bool {
    let subs = [
        0x50, 0x4B, 0x03, 0x04, 0x6D, 0x69, 0x6D, 0x65, 0x74, 0x79, 0x70, 0x65, 0x61, 0x70, 0x70,
        0x6C, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6F, 0x6E, 0x2F, 0x65, 0x70, 0x75, 0x62, 0x2B, 0x7A,
        0x69, 0x70,
    ];
    compare_bytes(buf, &subs, 0)
}

fn is_gz(buf: &[u8]) -> bool {
    let subs = [0x1F, 0x8B, 0x08];
    compare_bytes(buf, &subs, 0)
}

fn is_bz2(buf: &[u8]) -> bool {
    let subs = [0x42, 0x5A, 0x68];
    compare_bytes(buf, &subs, 0)
}

fn is_7z(buf: &[u8]) -> bool {
    let subs = [0x37, 0x7A, 0xBC, 0xAF, 0x27, 0x1C];
    compare_bytes(buf, &subs, 0)
}

fn is_pdf(buf: &[u8]) -> bool {
    let subs = [0x25, 0x50, 0x44, 0x46];
    compare_bytes(buf, &subs, 0)
}

fn is_exe(buf: &[u8]) -> bool {
    let subs = [0x4D, 0x5A];
    compare_bytes(buf, &subs, 0)
}

fn is_rtf(buf: &[u8]) -> bool {
    let subs = [0x7B, 0x5C, 0x72, 0x74, 0x66];
    compare_bytes(buf, &subs, 0)
}

fn is_nes(buf: &[u8]) -> bool {
    let subs = [0x4E, 0x45, 0x53, 0x1A];
    compare_bytes(buf, &subs, 0)
}

fn is_crx(buf: &[u8]) -> bool {
    let subs = [0x43, 0x72, 0x32, 0x34];
    compare_bytes(buf, &subs, 0)
}

fn is_ps(buf: &[u8]) -> bool {
    let subs = [0x25, 0x21];
    compare_bytes(buf, &subs, 0)
}

fn is_xz(buf: &[u8]) -> bool {
    let subs = [0xFD, 0x37, 0x7A, 0x58, 0x5A, 0x00];
    compare_bytes(buf, &subs, 0)
}

fn is_sqlite(buf: &[u8]) -> bool {
    let subs = [0x53, 0x51, 0x4C, 0x69];
    compare_bytes(buf, &subs, 0)
}

fn is_deb(buf: &[u8]) -> bool {
    let subs = [
        0x21, 0x3C, 0x61, 0x72, 0x63, 0x68, 0x3E, 0x0A, 0x64, 0x65, 0x62, 0x69, 0x61, 0x6E, 0x2D,
        0x62, 0x69, 0x6E, 0x61, 0x72, 0x79,
    ];
    compare_bytes(buf, &subs, 0)
}

fn is_ar(buf: &[u8]) -> bool {
    let subs = [0x21, 0x3C, 0x61, 0x72, 0x63, 0x68, 0x3E];
    compare_bytes(buf, &subs, 0)
}

fn is_lz(buf: &[u8]) -> bool {
    let subs = [0x4C, 0x5A, 0x49, 0x50];
    compare_bytes(buf, &subs, 0)
}

fn is_zip(buf: &[u8]) -> bool {
    buf.len() > 3
        && buf[0] == 0x50
        && buf[1] == 0x4B
        && (buf[2] == 0x3 || buf[2] == 0x5 || buf[2] == 0x7)
        && (buf[3] == 0x4 || buf[3] == 0x6 || buf[3] == 0x8)
}

fn is_tar(buf: &[u8]) -> bool {
    let subs = [0x75, 0x73, 0x74, 0x61, 0x72];
    compare_bytes(buf, &subs, 257)
}

fn is_rar(buf: &[u8]) -> bool {
    buf.len() > 6
        && buf[0] == 0x52
        && buf[1] == 0x61
        && buf[2] == 0x72
        && buf[3] == 0x21
        && buf[4] == 0x1A
        && buf[5] == 0x7
        && (buf[6] == 0x0 || buf[6] == 0x1)
}

fn is_swf(buf: &[u8]) -> bool {
    buf.len() > 2 && (buf[0] == 0x43 || buf[0] == 0x46) && buf[1] == 0x57 && buf[2] == 0x53
}

fn is_cab(buf: &[u8]) -> bool {
    buf.len() > 3
        && ((buf[0] == 0x4D && buf[1] == 0x53 && buf[2] == 0x43 && buf[3] == 0x46)
            || (buf[0] == 0x49 && buf[1] == 0x53 && buf[2] == 0x63 && buf[3] == 0x28))
}

fn is_eot(buf: &[u8]) -> bool {
    buf.len() > 35
        && buf[34] == 0x4C
        && buf[35] == 0x50
        && ((buf[8] == 0x02 && buf[9] == 0x00 && buf[10] == 0x01)
            || (buf[8] == 0x01 && buf[9] == 0x00 && buf[10] == 0x00)
            || (buf[8] == 0x02 && buf[9] == 0x00 && buf[10] == 0x02))
}

fn is_z(buf: &[u8]) -> bool {
    buf.len() > 1 && ((buf[0] == 0x1F && buf[1] == 0xA0) || (buf[0] == 0x1F && buf[1] == 0x9D))
}

fn is_rpm(buf: &[u8]) -> bool {
    buf.len() > 96 && buf[0] == 0xED && buf[1] == 0xAB && buf[2] == 0xEE && buf[3] == 0xDB
}

fn is_elf(buf: &[u8]) -> bool {
    buf.len() > 52 && buf[0] == 0x7F && buf[1] == 0x45 && buf[2] == 0x4C && buf[3] == 0x46
}

fn is_dcm(buf: &[u8]) -> bool {
    buf.len() > 131 && buf[128] == 0x44 && buf[129] == 0x49 && buf[130] == 0x43 && buf[131] == 0x4D
}

fn is_iso(buf: &[u8]) -> bool {
    let subs = [0x43, 0x44, 0x30, 0x30, 0x31];
    compare_bytes(buf, &subs, 32769)
}

fn is_macho(buf: &[u8]) -> bool {
    buf.len() > 3
        && ((buf[0] == 0xFE && buf[1] == 0xED && buf[2] == 0xFA && buf[3] == 0xCF) ||
(buf[0] == 0xFE && buf[1] == 0xED && buf[2] == 0xFA && buf[3] == 0xCE) ||
(buf[0] == 0xBE && buf[1] == 0xBA && buf[2] == 0xFE && buf[3] == 0xCA) ||
// Big endian versions below here...
(buf[0] == 0xCF && buf[1] == 0xFA && buf[2] == 0xED && buf[3] == 0xFE) ||
(buf[0] == 0xCE && buf[1] == 0xFA && buf[2] == 0xED && buf[3] == 0xFE) ||
(buf[0] == 0xCA && buf[1] == 0xFE && buf[2] == 0xBA && buf[3] == 0xBE))
}

const ZSTD_MAGIC_SKIPPABLE_START: u32 = 0x184D2A50;
const ZSTD_MAGIC_SKIPPABLE_MASK: u32 = 0xFFFFFFF0;

fn is_zst(buf: &[u8]) -> bool {
    // buf.len()
    let subs = [0x28, 0xB5, 0x2F, 0xFD];
    if compare_bytes(buf, &subs, 0) {
        return true;
    }
    if buf.len() < 8 {
        return false;
    }
    if littleendian_bytes(&buf[..4]) & ZSTD_MAGIC_SKIPPABLE_MASK == ZSTD_MAGIC_SKIPPABLE_START {
        let user_data_length = littleendian_bytes(&buf[4..8]) as usize;
        if buf.len() < 8 + user_data_length {
            return false;
        }
        let next_frame = &buf[8 + user_data_length..];
        return is_zst(next_frame);
    }
    false
}

pub fn sum() -> HashMapTypeMatcher {
    let mut ret = HashMapTypeMatcher::new();

    // ret.insert(XXX, is_xxx);
    ret.insert(TYPE_ZIP, is_zip);
    ret.insert(TYPE_TAR, is_tar);
    ret.insert(TYPE_RAR, is_rar);
    ret.insert(TYPE_SWF, is_swf);
    ret.insert(TYPE_CAB, is_cab);
    ret.insert(TYPE_EOT, is_eot);
    ret.insert(TYPE_Z, is_z);
    ret.insert(TYPE_RPM, is_rpm);
    ret.insert(TYPE_ELF, is_elf);
    ret.insert(TYPE_DCM, is_dcm);
    ret.insert(TYPE_ISO, is_iso);
    ret.insert(TYPE_MACHO, is_macho);
    ret.insert(TYPE_ZST, is_zst);

    ret.insert(TYPE_EPUB, is_epub);
    ret.insert(TYPE_GZ, is_gz);
    ret.insert(TYPE_BZ2, is_bz2);
    ret.insert(TYPE_7Z, is_7z);
    ret.insert(TYPE_PDF, is_pdf);
    ret.insert(TYPE_EXE, is_exe);
    ret.insert(TYPE_RTF, is_rtf);
    ret.insert(TYPE_NES, is_nes);
    ret.insert(TYPE_CRX, is_crx);
    ret.insert(TYPE_PS, is_ps);
    ret.insert(TYPE_XZ, is_xz);
    ret.insert(TYPE_SQLITE, is_sqlite);
    ret.insert(TYPE_DEB, is_deb);
    ret.insert(TYPE_AR, is_ar);
    ret.insert(TYPE_ZST, is_zst);
    ret.insert(TYPE_LZ, is_lz);

    ret
}
