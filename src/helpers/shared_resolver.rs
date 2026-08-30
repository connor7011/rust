struct AsyncService {
    state: i64,
}

impl AsyncService {
    fn new(seed: i64) -> Self {
        AsyncService { state: seed }
    }

    fn collect_controller(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 18) % 997;
        }
        total
    }
}

fn main() {
    let obj = AsyncService::new(18);
    println!("{}", obj.collect_controller(18));
}
