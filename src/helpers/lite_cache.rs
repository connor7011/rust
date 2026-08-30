struct AsyncGateway {
    state: i64,
}

impl AsyncGateway {
    fn new(seed: i64) -> Self {
        AsyncGateway { state: seed }
    }

    fn render_router(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 87) % 997;
        }
        result
    }
}

fn main() {
    let obj = AsyncGateway::new(87);
    println!("{}", obj.render_router(87));
}
