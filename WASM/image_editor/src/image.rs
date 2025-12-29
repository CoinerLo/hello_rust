pub struct Image {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height, data: vec![0; width * height * 4] }
    }

    pub fn from(data: &[u8], height: usize, width: usize) -> Self {
        Self { width, height, data: data.to_vec() }
    }
}
