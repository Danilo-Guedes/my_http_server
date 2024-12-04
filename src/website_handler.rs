use super::http::{Request, Response, StatusCode};
use super::server::Handler;

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        let body = "<h1>IT WORKS</h1><br><h1><span style='color:purple;'>DANILO</span> YOU ARE THE GOAT!!<h1>";
        Response::new(StatusCode::Ok, Some(body.to_string()))
    }
}
