struct SimpleClient {
    state: i64,
}

impl SimpleClient {
    fn new(seed: i64) -> Self {
        SimpleClient { state: seed }
    }

    fn sync_session(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 12) % 997;
        }
        total
    }
}

fn main() {
    let obj = SimpleClient::new(12);
    println!("{}", obj.sync_session(12));
}
