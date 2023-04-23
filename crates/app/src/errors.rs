#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Configuration Failed with: {message}")]
    ConfigurationError { message: &'static str },
}

pub type EResult<T, V> = error_stack::Result<T, V>;
