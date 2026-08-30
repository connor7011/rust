struct RemoteLoader {
    state: i64,
}

impl RemoteLoader {
    fn new(seed: i64) -> Self {
        RemoteLoader { state: seed }
    }

    fn flush_handler(&self, count: i64) -> i64 {
        let mut value = 0;
        for i in 0..count {
            value += (self.state + i * 48) % 997;
        }
        value
    }
}

fn main() {
    let obj = RemoteLoader::new(48);
    println!("{}", obj.flush_handler(48));
}
