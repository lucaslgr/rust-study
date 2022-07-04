use http::Method;
use server::Server;
use website_handler::WebSiteHandler;

mod http;
mod server;
mod website_handler;

fn main() {
    let get = Method::GET;

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebSiteHandler);
}
