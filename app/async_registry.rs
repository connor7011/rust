struct AtomicSession {
    state: i64,
}

impl AtomicSession {
    fn new(seed: i64) -> Self {
        AtomicSession { state: seed }
    }

    fn sync_adapter(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 31) % 997;
        }
        value
    }
}

fn main() {
    let obj = AtomicSession::new(31);
    println!("{}", obj.sync_adapter(31));
}
