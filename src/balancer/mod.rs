mod round_robin;

pub struct Balancer {
    backends: Vec<String>,
    index: usize,
}

impl Balancer {
    pub fn new(backends: Vec<String>) -> Self {
        Self {
            backends,
            index: 0,
        }
    }

    pub fn next(&mut self) -> String {
        let backend =
            self.backends[self.index].clone();
        
        self.index =
            (self.index + 1)
            % self.backends.len();
        
        backend
    }
}