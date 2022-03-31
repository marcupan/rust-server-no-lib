use std::io::Read;
use std::net::TcpListener;

use super::handler::WebsiteHandler;
use super::http::{HttpHandler, Request};
use super::thread::ThreadPool;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Server { address }
    }

    pub fn run(self, public_path: String) {
        println!("Listening to: {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();
        let pool = ThreadPool::new(4);
        let clone_path = &public_path;

        for stream in listener.incoming().take(2) {
            let mut stream = stream.unwrap();
            let mut handler = WebsiteHandler::new(clone_path.to_string());

            pool.execute(move || {
                let mut buffer = [0; 2048];

                match stream.read(&mut buffer) {
                    Ok(_) => {
                        // println!("Received a request: {}", String::from_utf8_lossy(&buffer));

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
            });
        }
    }
}
