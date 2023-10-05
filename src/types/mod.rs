mod application;
mod archive;
mod audio;
mod base;
mod document;
mod font;
mod image;
mod video;

use std::collections::HashMap;

use base::Type;

pub fn sum() -> HashMap<Type<'static>, fn(Vec<u8>) -> bool> {
    let mut ret = HashMap::<Type, fn(Vec<u8>) -> bool>::new();

    ret.extend(application::sum());
    ret.extend(image::sum());

    ret
}
