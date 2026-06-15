pub struct RoundRobin {
    index: usize,
}

impl RoundRobin {
    pub fn new() -> Self {
        Self { index: 0 }
    }

    pub fn next(&mut self, total_backends: usize) -> usize {
        let current = self.index;
        
        self.index = 
            (self.index + 1)
            % total_backends;

        current
    }
}