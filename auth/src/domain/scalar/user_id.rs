use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(pub Uuid);

impl UserId {
    pub fn parse(value: &str) -> Result<Self, uuid::Error> {
        Ok(UserId(Uuid::parse_str(value)?))
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self(Uuid::new_v4())
    }
}

impl ToString for UserId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
