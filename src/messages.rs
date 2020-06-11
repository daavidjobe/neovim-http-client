pub enum Messages {
    Unknown(String),
}

impl From<String> for Messages {
    fn from(event: String) -> Self {
        match &event[..] {
            _ => Messages::Unknown(event),
        }
    }
}
