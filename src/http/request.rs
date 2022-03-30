use super::HttpHeaders;
use super::Method;
use super::ParseError;
use super::QueryString;

use std::convert::TryFrom;
use std::fmt::Debug;
use std::str;

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    method: Method,
    query_string: Option<QueryString<'buf>>,
    headers: Option<HttpHeaders<'buf>>,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_string(&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }

    pub fn headers(&self) -> Option<&HttpHeaders> {
        self.headers.as_ref()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> {
        let request = str::from_utf8(buf)?;

        let (method, request) = get_next_slice(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_slice(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, request) = get_next_slice(request).ok_or(ParseError::InvalidRequest)?;
        let (_, request) = get_next_slice(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method = method.parse()?;

        let mut query_string = None;
        if let Some(i) = path.find('?') {
            query_string = Some(QueryString::from(&path[i + 1..]));
            path = &path[..i];
        }

        let mut headers = None;
        if let Some(i) = request.find("\r\n\r\n") {
            headers = Some(HttpHeaders::from(&request[..i]));
        }

        Ok(Self {
            path,
            method,
            query_string,
            headers,
        })
    }
}

fn get_next_slice(request: &str) -> Option<(&str, &str)> {
    for (i, ch) in request.chars().enumerate() {
        if ch == ' ' || ch == '\r' {
            return Some((&request[..i], &request[i + 1..]));
        }
    }

    None
}
