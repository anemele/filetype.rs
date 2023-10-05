use crate::types;
use std::path::Path;

pub fn match_all(path: &Path) {
    for (t, m) in types::sum() {
        println!("{}: {} ({})", path.display(), t.extension, t.mime);
    }
}
