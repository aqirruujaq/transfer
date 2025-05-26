use std::{
    io::{BufRead, BufReader, Error, Write},
    net::TcpStream,
    time::Duration,
};

use html_parse::{NOT_FOUND_BODY, join_index};
use http::{Request, Response, Version};
use httparse::EMPTY_HEADER;

use crate::server::file_tree::FileTree;

mod html_parse;

pub fn handle_connection(stream: &mut TcpStream, file_tree: &FileTree) {
    let req = Request::from_stream(stream).unwrap();
    let resp = response(&req, file_tree).unwrap();
    stream.write_all(&resp.to_byte()).unwrap();
}

pub fn response(req: &Request<()>, file_tree: &FileTree) -> http::Result<Response<Vec<u8>>> {
    match req.uri().path() {
        "/" => index(file_tree),
        "/favicon.ico" => favicon(),
        _ => not_found(),
    }
}

fn favicon() -> http::Result<Response<Vec<u8>>> {
    Response::builder()
        .status(204)
        .header("Connection", "close")
        .header("Content-Length", 0)
        .body(Vec::new())
}

fn index(file_tree: &FileTree) -> http::Result<Response<Vec<u8>>> {
    let body = join_index(file_tree.file_names());
    Response::builder()
        .status(200)
        .header("Connection", "close")
        .header("Content-Length", body.len())
        .body(body)
}

fn not_found() -> http::Result<Response<Vec<u8>>> {
    Response::builder()
        .status(204)
        .header("Connection", "close")
        .header("Content-Length", NOT_FOUND_BODY.len())
        .body(NOT_FOUND_BODY.to_vec())
}

pub trait SerializeResponse {
    fn to_byte(&self) -> Vec<u8>;
}

impl SerializeResponse for Response<Vec<u8>> {
    fn to_byte(&self) -> Vec<u8> {
        if !self.version().eq(&Version::HTTP_11) {
            panic!("Unable to serialize any response that is not an HTTP 1.1 version")
        }
        let mut res = Vec::new();
        write_resp_head(&mut res, self);
        res.extend_from_slice(self.body());
        res
    }
}

// Write the data in the http header to buf
fn write_resp_head<T>(buf: &mut Vec<u8>, resp: &Response<T>) {
    write!(buf, "HTTP/1.1 {}\r\n", resp.status()).unwrap();
    for (key, value) in resp.headers() {
        write!(
            buf,
            "{}: {}\r\n",
            key,
            value.to_str().unwrap_or_else(|e| todo!("info {e}"))
        )
        .unwrap();
    }
    write!(buf, "\r\n").unwrap();
}

pub trait DeserializeRequset {
    fn from_stream(stream: &mut TcpStream) -> http::Result<Request<()>>;
}

impl DeserializeRequset for Request<()> {
    fn from_stream(stream: &mut TcpStream) -> http::Result<Request<()>> {
        let buf = match read_http_request_head(stream) {
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

// Reading data from TcpStream, timeout after ten seconds
fn read_http_request_head(stream: &mut TcpStream) -> Result<Vec<u8>, Error> {
    stream
        .set_read_timeout(Some(Duration::from_secs(1)))
        .expect("No error will occur");
    let mut buf_reader = BufReader::new(stream);
    let mut buf_line = String::new();
    let mut buf = Vec::new();

    loop {
        let byte = buf_reader.read_line(&mut buf_line)?;
        buf.extend_from_slice(buf_line.as_bytes());

        if &buf_line == "\r\n" || buf_line.is_empty() {
            break;
        }
        buf_line.clear();
    }

    Ok(buf)
}
