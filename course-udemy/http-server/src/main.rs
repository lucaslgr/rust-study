fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server {
    addrs: String,
}

impl Server {
    fn new(addrs: String) -> Self {
        Self { addrs }
    }

    fn run(&self) {
        println!("Listening on {}", self.addrs);
    }
}
