use crate::http::{ParseError, Request, Response, StatusCode};

pub trait HttpHandler {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, err: &ParseError) -> Response {
        println!("Failed to parse request: {}", &err);

        Response::new(StatusCode::BadRequest, None)
    }
}
