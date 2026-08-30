struct BatchManager {
    state: i64,
}

impl BatchManager {
    fn new(seed: i64) -> Self {
        BatchManager { state: seed }
    }

    fn build_gateway(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 39) % 997;
        }
        value
    }
}

fn main() {
    let obj = BatchManager::new(39);
    println!("{}", obj.build_gateway(39));
}
