use crate::http::{ParseError, Request, Response, StatusCode};
use std::io::Read;
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        let listener = TcpListener::bind(&self.addr).unwrap();

        println!("Listening on {}", &self.addr);

        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    println!("Connection established: {}", addr);
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            print!("Received a request: {}", String::from_utf8_lossy(&buffer)); // lossy replaces invalid utf-8 sequences with �

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(&request);
                                    handler.handle_request(&request)
                                }
                                Err(e) => {
                                   handler.handle_bad_request(&e)
                                }
                            };
                            if let Err(e) = response.send(&mut stream) {
                                eprintln!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => eprintln!("Failed to read from connection: {}", e),
                    }
                }

                Err(e) => {
                    eprintln!("Failed to establish a connection: {}", e);
                }
            }
        }
    }
}


pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        eprintln!("Failed to parse a request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}