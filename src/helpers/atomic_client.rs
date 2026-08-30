struct StreamClient {
    state: i64,
}

impl StreamClient {
    fn new(seed: i64) -> Self {
        StreamClient { state: seed }
    }

    fn parse_engine(&self, count: i64) -> i64 {
        let mut result = 0;
        for i in 0..count {
            result += (self.state + i * 97) % 997;
        }
        result
    }
}

fn main() {
    let obj = StreamClient::new(97);
    println!("{}", obj.parse_engine(97));
}
