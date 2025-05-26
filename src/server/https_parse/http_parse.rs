use std::{
    io::{BufRead, BufReader, Error},
    net::TcpStream,
};

use http::{Request, Response};
use httparse::EMPTY_HEADER;

pub fn response(req: &Request<()>) -> http::Result<Response<String>> {
    match req.uri().path() {
        "/" => index(),
        _ => not_found(),
    }
}

fn index() -> http::Result<Response<String>> {
    Response::builder()
        .status(200)
        .body(include_str!("../../../index.html").to_string())
}

fn not_found() -> http::Result<Response<String>> {
    Response::builder()
        .status(200)
        .body(include_str!("../../../404.html").to_string())
}

pub trait SerializeResponse {
    fn to_byte(&self) -> Vec<u8>;
}

impl SerializeResponse for Response<String> {
    fn to_byte(&self) -> Vec<u8> {
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
    fn to_byte(&self) -> Vec<u8> {
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
        let buf = match read_http_request(stream){
            Ok(buf) => buf,
            Err(e) => todo!("resolve or info {}", e),
        };

        let mut header = [EMPTY_HEADER; 16];
        let mut parse = httparse::Request::new(&mut header);

        if parse.parse(&buf).is_err() {
            todo!("add info");
        }

        Request::builder().uri(parse.path.unwrap()).body(())
    }
}

fn read_http_request(stream: &mut TcpStream) -> Result<Vec<u8>, Error> {
    let mut buf_reader = BufReader::new(stream);
    let mut buf_line = String::new();
    let mut buf = Vec::new();

    loop {
        buf_reader.read_line(&mut buf_line)?;
        buf.extend_from_slice(buf_line.as_bytes());
        if &buf_line == "\r\n" {
           break;
        }
        buf_line.clear();
    }

    Ok(buf)
}
