use std::io::Read;
use crate::http::{Request, Response, StatusCode, ParseError};
use std::convert::TryFrom;
use std::net::TcpListener;

pub trait Handler {
  fn handle_request(&mut self, request: &Request) -> Response;
  fn handle_bad_request(&mut self, e: &ParseError) -> Response {
    println!("Failed to parse request: {}", e);
    Response::new(StatusCode::BadRequest, None)
  }
}

pub struct Server {
  addrs: String,
}

impl Server {
  pub fn new(addrs: String) -> Self {
    Self { addrs }
  }

  pub fn run(&self, mut handler: impl Handler) {
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
              
              let response = match Request::try_from(&buffer[..]) {
                Ok(request) =>
                  handler.handle_request(&request),
                Err(e) =>
                  handler.handle_bad_request(&e),
              };

              if let Err(e) = response.send(&mut tcp_stream) {
                println!("Failed to send the response {}", e);
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
