pub fn compare_bytes(slice: &Vec<u8>, subs: &Vec<u8>, offset: usize) -> bool {
    let s1 = subs.len();
    if s1 + offset > slice.len() {
        return false;
    }

    for i in 0..s1 {
        if subs[i] != slice[i + offset] {
            return false;
        }
    }

    true
}
