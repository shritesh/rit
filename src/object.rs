pub trait Object {
    fn object_type(&self) -> &str;

    fn data(&self) -> &[u8];
}
