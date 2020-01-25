use mobc_postgres::mobc::Error as MobcError;
use mobc_postgres::tokio_postgres::Error as PostgresError;
use serde_tokio_postgres::Error as SerdeError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Pool error: {0}")]
    Mobc(#[from] MobcError<PostgresError>),
    #[error("Postgress error: {0}")]
    Postgres(#[from] PostgresError),
    #[error("Deserialization error: {0}")]
    Serde(#[from] SerdeError),
}
