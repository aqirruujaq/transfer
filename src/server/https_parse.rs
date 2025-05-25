use std::{io::Write, net::TcpStream};

use http::Request;
use http_parse::{DeserializeRequset, SerializeResponse};

mod http_parse;

pub fn response(stream: &mut TcpStream) {
    let req = Request::from_stream(stream).unwrap();
    let resp = http_parse::response(&req).unwrap();
    stream.write_all(&resp.into_byte()).unwrap();
}
