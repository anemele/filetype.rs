use crate::{
    constants::NUM_SIGNATURE_BYTES,
    types::{self, Type, TypeTypesMatcher, TYPE_UNKNOWN},
    utils::get_signature_bytes,
};
use std::path::Path;

fn match_wrapper(path: &Path, tm: TypeTypesMatcher) -> Type {
    let mut sig = [0_u8; NUM_SIGNATURE_BYTES];
    get_signature_bytes(path, &mut sig);
    for (t, m) in tm {
        if m(&sig) {
            return t;
        }
    }

    TYPE_UNKNOWN
}

pub type Matcher = fn(&Path) -> Type;

pub fn match_all(path: &Path) -> Type {
    match_wrapper(path, types::sum())
}

pub fn match_application(path: &Path) -> Type {
    match_wrapper(path, types::application::sum())
}

pub fn match_archive(path: &Path) -> Type {
    match_wrapper(path, types::archive::sum())
}

pub fn match_audio(path: &Path) -> Type {
    match_wrapper(path, types::audio::sum())
}

pub fn match_document(path: &Path) -> Type {
    match_wrapper(path, types::document::sum())
}

pub fn match_font(path: &Path) -> Type {
    match_wrapper(path, types::font::sum())
}

pub fn match_image(path: &Path) -> Type {
    match_wrapper(path, types::image::sum())
}

pub fn match_video(path: &Path) -> Type {
    match_wrapper(path, types::video::sum())
}
