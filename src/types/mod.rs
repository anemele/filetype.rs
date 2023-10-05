mod application;
mod archive;
mod audio;
mod base;
mod document;
mod font;
mod image;
mod utils;
mod video;

use std::collections::HashMap;

pub use base::TYPE_UNKNOWN;
use base::{Type, TypeMatcher, TypeTypesMatcher};

pub fn sum() -> TypeTypesMatcher {
    let mut ret = HashMap::<Type, TypeMatcher>::new();

    ret.extend(application::sum());
    ret.extend(archive::sum());
    ret.extend(audio::sum());
    ret.extend(document::sum());
    ret.extend(font::sum());
    ret.extend(image::sum());
    ret.extend(video::sum());

    ret
}
