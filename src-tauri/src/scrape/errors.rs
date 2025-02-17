use specta::Type;

#[derive(Debug, thiserror::Error, Type)]
pub enum Error {
    #[error("Network error: {0}")]
    Network(String),
    #[error("Parsing error: {0}")]
    Parsing(String),
    // Add other error variants as needed
}

// Implement `serde::Serialize` for `Error`
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
