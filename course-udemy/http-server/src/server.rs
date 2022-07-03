use crate::http::Request;
use std::convert::TryFrom;
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

          let mut buffer: [u8; 1024] = [0; 1024];
          match tcp_stream.read(&mut buffer) {
            Ok(_) => {
              println!("Received a request: {}", String::from_utf8_lossy(&buffer));
              match Request::try_from(&buffer[..]) {
                Ok(request) => {
                  dbg!(request);
                }
                Err(e) => println!("Failed to convert the buffer into a request {}", e),
              }
            }
            Err(e) => {
              println!("Failed to establish a connection: {}", e);
            }
          }
        }
        Err(error) => {
          println!("Failed to establish a connection: {} ", error);
        } // _ => {}
      }
    }
  }
}
