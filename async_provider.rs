struct LocalCollector {
    state: i64,
}

impl LocalCollector {
    fn new(seed: i64) -> Self {
        LocalCollector { state: seed }
    }

    fn fetch_worker(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 82) % 997;
        }
        total
    }
}

fn main() {
    let obj = LocalCollector::new(82);
    println!("{}", obj.fetch_worker(82));
}
