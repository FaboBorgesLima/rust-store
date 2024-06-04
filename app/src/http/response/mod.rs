use content_type::ContentType;

pub mod content_type;
pub struct Response {
    pub body: String,
    pub content_type: ContentType,
    pub code: u16,
}

impl Response {
    pub fn new(content_type: ContentType, code: u16, body: String) -> Self {
        Self {
            body,
            content_type,
            code,
        }
    }

    pub fn get_content_length(&self) -> usize {
        self.body.len()
    }

    pub fn get_content_length_as_string(&self) -> String {
        let length = self.get_content_length();
        format!("Content-Length: {length}")
    }

    pub fn not_found() -> Self {
        Self {
            body: String::new(),
            code: 404,
            content_type: ContentType::ApplicationJson,
        }
    }
    pub fn server_error() -> Self {
        Self {
            body: String::new(),
            code: 500,
            content_type: ContentType::ApplicationJson,
        }
    }
    pub fn bad_request() -> Self {
        Self {
            body: String::new(),
            code: 400,
            content_type: ContentType::ApplicationJson,
        }
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        format!(
            "HTTP/1.1 {code}\r\n{content_type}\r\n{content_size}\r\nAccess-Control-Allow-Origin: *\r\n\r\n{body}",
            code = self.code,
            content_type = self.content_type.to_string(),
            content_size = self.get_content_length_as_string(),
            body = self.body
        )
    }
}
