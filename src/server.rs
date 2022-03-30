use std::io::Read;
use std::net::TcpListener;

use super::http::{HttpHandler, Request};

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Server { address }
    }

    pub fn run(self, mut handler: impl HttpHandler) {
        println!("Listening to: {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 2048];

                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(response) => handler.handle_request(&response),
                                Err(err) => handler.handle_bad_request(&err),
                            };

                            if let Err(err) = response.send(&mut stream) {
                                println!("Failed to send response: {}", err);
                            }
                        }
                        Err(err) => println!("Error: {}", err),
                    }
                }
                Err(err) => println!("Error: {}", err),
            }
        }
    }
}
