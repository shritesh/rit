use crate::Object;

pub struct Blob {
    data: Vec<u8>,
}

impl Blob {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}

impl Object for Blob {
    fn kind(&self) -> &str {
        "blob"
    }

    fn data(&self) -> Vec<u8> {
        self.data.clone()
    }
}
