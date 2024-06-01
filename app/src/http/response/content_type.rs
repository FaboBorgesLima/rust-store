pub enum ContentType {
    ApplicationJson,
    Text,
}

impl ToString for ContentType {
    fn to_string(&self) -> String {
        match self {
            ContentType::ApplicationJson => "Content-Type: application/json".to_string(),
            ContentType::Text => "Content-Type: text/html".to_string(),
        }
    }
}
