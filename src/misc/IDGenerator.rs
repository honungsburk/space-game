pub struct IDGenerator {
    id: usize,
}

impl IDGenerator {
    pub fn new() -> Self {
        Self { id: 0 }
    }

    pub fn next(&mut self) -> usize {
        self.id += 1;
        self.id
    }
}
