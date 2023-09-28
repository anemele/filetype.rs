pub enum InputType<'a> {
    File(&'a str),
    Bytes(Vec<u8>),
}
