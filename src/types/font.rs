use super::base::Type;
use std::collections::HashMap;

pub fn sum() -> HashMap<Type<'static>, fn(Vec<u8>) -> bool> {
    let mut ret = HashMap::<Type, fn(Vec<u8>) -> bool>::new();

    // ret.insert(XXX, is_xxx);

    ret
}
