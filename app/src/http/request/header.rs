use super::method::Method;

pub struct Header {
    pub length: usize,
    pub path: String,
    pub method: Method,
    pub url_params: String,
}
