pub mod details;
pub mod genera;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Not found")]
  NotFound,
  #[error(transparent)]
  Sqlx(#[from] sqlx::Error),
}
