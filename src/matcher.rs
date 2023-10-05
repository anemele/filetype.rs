use crate::{types, utils::get_signature_bytes};
use std::path::Path;

pub fn match_all(path: &Path) {
    let sig = get_signature_bytes(path);

    for (t, m) in types::sum() {
        if m(&sig) {
            println!("{}: {} ({})", path.display(), t.extension, t.mime);
            return;
        }
    }

    println!("{}: {}", path.display(), types::TYPE_UNKNOWN);
}
