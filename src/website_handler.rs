use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, Some("<h1>IT WORKS</h1><br><h1><span style='color:purple;'>DANILO</span> YOU ARE THE GOAT!!<h1>".to_string())),
                "/hello" => Response::new(StatusCode::Ok, Some("<h1>HELLO</h1>".to_string())),
                _ => Response::new(StatusCode::NotFound, None),
            },
            _ => Response::new(StatusCode::NotFound, None),
        }

        // let body = "<h1>IT WORKS</h1><br><h1><span style='color:purple;'>DANILO</span> YOU ARE THE GOAT!!<h1>";
        // Response::new(StatusCode::Ok, Some(body.to_string()))
    }
}
