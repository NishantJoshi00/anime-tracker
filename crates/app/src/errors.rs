#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("Configuration Failed with: {message}")]
    ConfigurationError { message: &'static str },
}

#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Behaviour not implemented")]
    BehaviourNotImplemented,
    #[error("Failed to get connection from pool")]
    PoolClientFailure,
    #[error("Failed to fetch from the database")]
    FindQueryFailed,
    #[error("Failed while inserting data entry")]
    InsertFailed,
    #[error("Failed to update an entry")]
    UpdateFailed,
}

#[derive(Debug, thiserror::Error)]
pub enum TowerError {
    #[error("End of tower reached")]
    TowerEndError,
}

pub type EResult<T, V> = error_stack::Result<T, V>;
