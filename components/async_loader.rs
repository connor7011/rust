struct CoreResolver {
    state: i64,
}

impl CoreResolver {
    fn new(seed: i64) -> Self {
        CoreResolver { state: seed }
    }

    fn decode_dispatcher(&self, count: i64) -> i64 {
        let mut acc = 0;
        for i in 0..count {
            acc += (self.state + i * 77) % 997;
        }
        acc
    }
}

fn main() {
    let obj = CoreResolver::new(77);
    println!("{}", obj.decode_dispatcher(77));
}
