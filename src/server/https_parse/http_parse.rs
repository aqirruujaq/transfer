use std::{io::Read, net::TcpStream};

use http::{Request, Response};
use httparse::EMPTY_HEADER;

pub fn response(req: &Request<()>) -> http::Result<Response<String>> {
    match req.uri().path() {
        // "/" => index(),
        _ => not_found(),
    }
}

fn not_found() -> http::Result<Response<String>> {
    let res = Response::builder()
        .status(200)
        .body(include_str!("../../../404.html").to_string());
    res
}

pub trait SerializeResponse {
    fn into_byte(&self) -> Vec<u8>;
}

impl SerializeResponse for Response<String> {
    fn into_byte(&self) -> Vec<u8> {
        format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            self.body().len(),
            self.body()
        )
        .bytes()
        .collect()
    }
}

impl SerializeResponse for Response<&[u8]> {
    fn into_byte(&self) -> Vec<u8> {
        let mut res: Vec<u8> = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n",
            self.body().len()
        )
        .bytes()
        .collect();
        res.extend_from_slice(self.body());
        res
    }
}

pub trait DeserializeRequset {
    fn from_stream(stream: &mut TcpStream) -> http::Result<Request<()>>;
}

impl DeserializeRequset for Request<()> {
    fn from_stream(stream: &mut TcpStream) -> http::Result<Request<()>> {
        let buf = read_http_request(stream);

        let mut header = [EMPTY_HEADER; 16];
        let mut parse = httparse::Request::new(&mut header);

        if parse.parse(&buf).is_err() {
            todo!("add info");
        }

        Request::builder().uri(parse.path.unwrap()).body(())
    }
}

// TODO: use KMP
fn read_http_request(stream: &mut TcpStream) -> Vec<u8> {
    let mut buffer = Vec::new();
    let mut read_buf = [0u8; 1024];

    loop {
        let n = stream.read(&mut read_buf).unwrap_or_default();
        if n == 0 {
            break;
        }

        buffer.extend_from_slice(&read_buf[..n]);
    }
}
