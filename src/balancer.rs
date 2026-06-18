mod round_robin;

use round_robin::RoundRobin;

pub struct Balancer {
    backends: Vec<String>,
    strategy: RoundRobin,
}

impl Balancer {
    pub fn new(backends: Vec<String>) -> anyhow::Result<Self> {
        if backends.is_empty() {
            anyhow::bail!("at least one backend is required");
        }

        Ok(Self {
            backends,
            strategy: RoundRobin::new(),
        })
    }

    pub fn next(&mut self) -> String {
        let index =
            self.strategy.next(self.backends.len());

        self.backends[index].clone()
    }
}

#[cfg(test)]
mod tests;
