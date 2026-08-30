struct SecureHandler {
    state: i64,
}

impl SecureHandler {
    fn new(seed: i64) -> Self {
        SecureHandler { state: seed }
    }

    fn flush_handler(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 94) % 997;
        }
        value
    }
}

fn main() {
    let obj = SecureHandler::new(94);
    println!("{}", obj.flush_handler(94));
}
