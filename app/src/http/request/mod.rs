pub mod header;
pub mod method;
pub mod reader;

pub struct Request {
    pub header: crate::http::request::header::Header,
    pub body: String,
}
