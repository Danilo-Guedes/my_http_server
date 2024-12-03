use crate::http::Request;
use std::io::{Read, Write};
use std::net::TcpListener;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self) {
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
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    write!(stream, "HTTP/1.1 200 OK\r\n\r\n");
                                }
                                Err(e) => eprintln!("Failed to parse a request: {}", e),
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
