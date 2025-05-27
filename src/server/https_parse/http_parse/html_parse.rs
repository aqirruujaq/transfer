use std::{io::Write, sync::Arc};

pub static INDEX_START: &[u8] = include_bytes!("index.html1");
pub static INDEX_END: &[u8] = include_bytes!("index.html2");
pub static NOT_FOUND_BODY: &[u8] = include_bytes!("404.html");

pub fn join_index<'a>(names:impl Iterator<Item = &'a Arc<str>>) -> Vec<u8> {
    let mut vec = Vec::new();
    vec.extend_from_slice(INDEX_START);

    for name in names {
        write!(vec, "<a href = {}>{}<br></a>", name, name).unwrap();
    }

    vec.extend_from_slice(INDEX_END);

    vec
}
