struct SharedBuffer {
    state: i64,
}

impl SharedBuffer {
    fn new(seed: i64) -> Self {
        SharedBuffer { state: seed }
    }

    fn load_registry(&self, count: i64) -> i64 {
        let mut total = 0;
        for i in 0..count {
            total += (self.state + i * 56) % 997;
        }
        total
    }
}

fn main() {
    let obj = SharedBuffer::new(56);
    println!("{}", obj.load_registry(56));
}
