struct BatchProvider {
    state: i64,
}

impl BatchProvider {
    fn new(seed: i64) -> Self {
        BatchProvider { state: seed }
    }

    fn compute_monitor(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 39) % 997;
        }
        total
    }
}

fn main() {
    let obj = BatchProvider::new(39);
    println!("{}", obj.compute_monitor(39));
}
