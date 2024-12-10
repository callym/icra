use sqlx::PgExecutor;
use uuid::Uuid;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Genus {
  pub id: Uuid,
  pub name: String,
}

impl Genus {
  pub async fn insert(
    name: impl AsRef<str>,
    pool: impl PgExecutor<'_> + Copy,
  ) -> Result<Self, sqlx::Error> {
    let name = name.as_ref();

    let existing = Genus::get_from_name(name, pool).await;

    if let Ok(Some(genus)) = existing {
      return Ok(genus);
    }

    sqlx::query_file_as!(Genus, "sql/genera/insert.sql", name)
      .fetch_one(pool)
      .await
  }

  pub async fn get_from_id(
    id: &Uuid,
    pool: impl PgExecutor<'_>,
  ) -> Result<Option<Self>, sqlx::Error> {
    sqlx::query_as!(Genus, "SELECT * FROM genera WHERE id = $1", id)
      .fetch_optional(pool)
      .await
  }

  pub async fn get_from_name(
    name: impl AsRef<str>,
    pool: impl PgExecutor<'_>,
  ) -> Result<Option<Self>, sqlx::Error> {
    let name = name.as_ref();

    sqlx::query_as!(Genus, "SELECT * FROM genera WHERE name = $1", name)
      .fetch_optional(pool)
      .await
  }
}
