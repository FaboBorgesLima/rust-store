pub enum ContentType {
    Application,
    Text,
}

impl ToString for ContentType {
    fn to_string(&self) -> String {
        match self {
            ContentType::Application => "Content-Type: application/json".to_string(),
            ContentType::Text => "Content-Type: text/html".to_string(),
        }
    }
}
