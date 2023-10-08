use filetypes::matcher::Matcher;
use std::path::Path;

const SAMPLE_DIR: &str = "tests/fixtures";

fn test(matcher: Matcher, file: &str, mime: &str, extension: &str) {
    let sample = Path::new(SAMPLE_DIR).join(file);
    let ret = matcher(sample.as_path());
    assert_eq!(mime, ret.mime);
    assert_eq!(extension, ret.extension);
}

#[cfg(test)]
mod tests {

    use filetypes::matcher::*;

    use super::test;

    #[test]
    fn test_image() {
        test(match_image, "sample.jpg", "image/jpeg", "jpg");
        test(match_image, "sample.png", "image/png", "png");
        test(match_image, "sample.gif", "image/gif", "gif");
    }

    #[test]
    fn test_video() {
        test(match_video, "sample.mov", "video/quicktime", "mov");
        test(match_video, "sample.mp4", "video/mp4", "mp4");
    }

    #[test]
    fn test_audio() {
        test(match_audio, "sample.m4a", "audio/mp4", "m4a");
    }

    #[test]
    fn test_archive() {
        test(match_archive, "sample.zip", "application/zip", "zip");
        test(match_archive, "sample.tar", "application/x-tar", "tar");
    }

    #[ignore = "MS Office 95"]
    #[test]
    fn test_document_2003() {
        test(match_document, "sample.doc", "application/msword", "doc");
        test(
            match_document,
            "sample.ppt",
            "application/vnd.ms-powerpoint",
            "ppt",
        );
        test(
            match_document,
            "sample.xls",
            "application/vnd.ms-excel",
            "xls",
        );
    }

    #[test]
    fn test_document() {
        test(
            match_document,
            "sample.docx",
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            "docx",
        );
        test(
            match_document,
            "sample.pptx",
            "application/vnd.openxmlformats-officedocument.presentationml.presentation",
            "pptx",
        );
        test(
            match_document,
            "sample.xlsx",
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            "xlsx",
        );
    }
}
