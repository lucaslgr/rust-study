use super::server::Handler;
use crate::http::{Request, Response, StatusCode, Method};

pub struct  WebSiteHandler;

impl Handler for WebSiteHandler {
  fn handle_request(&mut self, request: &Request) -> Response {
    match request.method() {
      _ => Response::new(StatusCode::NotFound, None),
      Method::GET => match request.path() {
        "/" => Response::new(StatusCode::Ok, Some("<h1>Welcome</h1>".to_string())),
        "/hello" => Response::new(StatusCode::Ok, Some("<h1>Hello</h1>".to_string())),
        _ => Response::new(StatusCode::NotFound, None),
      }
    }
  }
}