pub trait AsBytes {
    fn as_bytes(&self) -> Vec<u8>;
}

pub trait FromBytes<T> {
    fn from_bytes(src: &[u8]) -> T;
}
