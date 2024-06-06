use content_type::ContentType;

pub mod content_type;
pub struct Response {
    pub body: String,
    pub content_type: ContentType,
    pub code: u16,
    access_control_allow_cross_origin: String,
    access_control_allow_methods: String,
}

impl Response {
    pub fn new(content_type: ContentType, code: u16, body: String) -> Self {
        Self {
            body,
            content_type,
            code,
            access_control_allow_cross_origin: String::from("*"),
            access_control_allow_methods: String::from("POST, GET, PUT, PATH, DELETE"),
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
        Self::new(ContentType::ApplicationJson, 404, String::new())
    }
    pub fn server_error() -> Self {
        Self::new(ContentType::ApplicationJson, 500, String::new())
    }
    pub fn bad_request() -> Self {
        Self::new(ContentType::ApplicationJson, 400, String::new())
    }

    pub fn ok(body: Option<String>) -> Self {
        Self::new(ContentType::ApplicationJson, 200, body.unwrap_or_default())
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        format!(
            "HTTP/1.1 {code}\r\n{content_type}\r\n{content_size}\r\nAccess-Control-Allow-Origin: {access_control_allow_cross_origin}\r\nAccess-Control-Allow-Methods: {access_control_allow_methods} \r\n\r\n{body}",
            code = self.code,
            content_type = self.content_type.to_string(),
            content_size = self.get_content_length_as_string(),
            access_control_allow_cross_origin= self.access_control_allow_cross_origin,
            access_control_allow_methods = self.access_control_allow_methods,
            body = self.body
        )
    }
}
