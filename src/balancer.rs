mod round_robin;

use round_robin::RoundRobin;

pub struct Balancer {
    backends: Vec<String>,
    strategy: RoundRobin,
}

impl Balancer {
    pub fn new(backends: Vec<String>) -> Self {
        Self {
            backends,
            strategy: RoundRobin::new(),
        }
    }

    pub fn next(&mut self) -> String {
        let index =
            self.strategy.next(self.backends.len());

        self.backends[index].clone()
    }
}
