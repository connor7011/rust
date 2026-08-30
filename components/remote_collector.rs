struct SmartWorker {
    state: i64,
}

impl SmartWorker {
    fn new(seed: i64) -> Self {
        SmartWorker { state: seed }
    }

    fn resolve_cache(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 74) % 997;
        }
        result
    }
}

fn main() {
    let obj = SmartWorker::new(74);
    println!("{}", obj.resolve_cache(74));
}
