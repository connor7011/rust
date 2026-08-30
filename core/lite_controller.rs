struct AtomicClient {
    state: i64,
}

impl AtomicClient {
    fn new(seed: i64) -> Self {
        AtomicClient { state: seed }
    }

    fn run_handler(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 39) % 997;
        }
        acc
    }
}

fn main() {
    let obj = AtomicClient::new(39);
    println!("{}", obj.run_handler(39));
}
