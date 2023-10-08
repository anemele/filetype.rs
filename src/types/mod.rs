pub mod application;
pub mod archive;
pub mod audio;
mod base;
pub mod document;
pub mod font;
pub mod image;
mod utils;
pub mod video;

use std::collections::HashMap;

pub use base::Type;
pub use base::TYPE_UNKNOWN;
pub use base::{TypeMatcher, TypeTypesMatcher};

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
