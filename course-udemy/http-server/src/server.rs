use std::io::Read;
use std::net::TcpListener;

pub struct Server {
  addrs: String,
}

impl Server {
  pub fn new(addrs: String) -> Self {
    Self { addrs }
  }

  pub fn run(&self) {
    let listener = TcpListener::bind(&self.addrs).unwrap();
    println!("Listening on {}", self.addrs);

    loop {
      let result = listener.accept();
      match result {
        Ok((mut tcp_stream, socket_addr)) => {
          println!("Receive a package from client_addres: {}", socket_addr);

          let mut buffer: [u8] = [0; 1024];
          tcp_stream.read(&mut buffer);
        }
        Err(error) => {
          println!("Failed to establish a connection: {} ", error);
        } // _ => {}
      }
    }
  }
}
