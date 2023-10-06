use crate::{constants::NUM_SIGNATURE_BYTES, types, utils::get_signature_bytes};
use std::path::Path;

pub fn match_all(path: &Path) {
    let mut sig = [0_u8; NUM_SIGNATURE_BYTES];
    get_signature_bytes(path, &mut sig);

    for (t, m) in types::sum() {
        if m(&sig) {
            println!("{}: {} ({})", path.display(), t.extension, t.mime);
            return;
        }
    }

    println!("{}: {}", path.display(), types::TYPE_UNKNOWN);
}
