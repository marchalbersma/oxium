pub struct Cursor<'a> {
    bytes: &'a [u8],
    pub pos: usize,
}

impl<'a> Cursor<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            bytes: src.as_bytes(),
            pos: 0,
        }
    }

    pub fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }
}

impl Iterator for Cursor<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let byte = self.peek();

        if byte.is_some() {
            self.pos += 1;
        }

        byte
    }
}
