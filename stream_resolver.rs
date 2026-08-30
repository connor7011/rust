struct BatchManager {
    state: i64,
}

impl BatchManager {
    fn new(seed: i64) -> Self {
        BatchManager { state: seed }
    }

    fn load_engine(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 90) % 997;
        }
        total
    }
}

fn main() {
    let obj = BatchManager::new(90);
    println!("{}", obj.load_engine(90));
}
