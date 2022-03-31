use std::fs;

use crate::http::{HttpHandler, Method, Request, Response, StatusCode};

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Traversal Attack Attempted: {}", file_path);

                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl HttpHandler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        println!(
            "request.headers() Accept: {:?}",
            request.headers().unwrap().get("Accept")
        );

        match request.method() {
            Method::GET => match (request.path(), request.query_string()) {
                ("/", _) => Response::new(StatusCode::Ok, self.read_file("index.html")),
                ("/hello", None) => {
                    println!("/hello none");

                    Response::new(StatusCode::Ok, self.read_file("hello.html"))
                }
                ("/hello", Some(query_string)) => {
                    println!("search query: {:?}", &query_string.get("search"));

                    Response::new(StatusCode::Ok, self.read_file("search.html"))
                }
                (path, _) => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => {
                        println!("Not found");

                        Response::new(StatusCode::NotFound, self.read_file("404.html"))
                    }
                },
            },
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
