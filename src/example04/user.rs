#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
}

impl User {
    #[must_use]
    pub fn new(name: &str) -> Self {
        User {
            name: name.to_string(),
        }
    }
}
