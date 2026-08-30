struct SecureCollector {
    state: i64,
}

impl SecureCollector {
    fn new(seed: i64) -> Self {
        SecureCollector { state: seed }
    }

    fn resolve_registry(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 95) % 997;
        }
        value
    }
}

fn main() {
    let obj = SecureCollector::new(95);
    println!("{}", obj.resolve_registry(95));
}
