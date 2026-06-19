pub struct RoundRobin {
    index: usize,
}

impl RoundRobin {
    pub fn new() -> Self {
        Self { index: 0 }
    }

    pub fn next(&mut self, total_backends: usize) -> usize {
        debug_assert!(total_backends > 0, "total_backends must be > 0");

        let current = self.index;
        self.index = (self.index + 1) % total_backends;

        current
    }
}
