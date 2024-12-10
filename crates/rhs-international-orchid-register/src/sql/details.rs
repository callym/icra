use sqlx::PgExecutor;
use time::Date;
use uuid::Uuid;

use crate::sql::{genera::Genus, Error};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Details {
  pub id: Uuid,
  pub registar_id: i32,
  pub genus: Uuid,
  pub epithet: String,
  pub synonym: bool,
  pub synonym_genus: Option<Uuid>,
  pub synonym_epithet: Option<Uuid>,
  pub registrant_name: Option<String>,
  pub originator_name: Option<String>,
  pub date_of_registration: Option<Date>,
  pub seed_parent: Option<Uuid>,
  pub pollen_parent: Option<Uuid>,
}

impl Details {
  pub async fn insert(
    details: &crate::api::Details,
    pool: impl PgExecutor<'_> + Copy,
  ) -> Result<Self, Error> {
    let res = Details::get_from_name(&details.genus, &details.epithet, pool).await?;
    if let Some(res) = res {
      return Ok(res);
    }

    let genus = match Genus::get_from_name(&details.genus, pool).await? {
      Some(genus) => genus,
      None => Genus::insert(&details.genus, pool).await?,
    };

    let (synonym_genus, synonym_epithet) = if details.synonym {
      match &details.synonym_details {
        Some(details) => {
          let genus = match Genus::get_from_name(&details.genus, pool).await? {
            Some(genus) => genus,
            None => Genus::insert(&details.genus, pool).await?,
          };

          let epithet = if let Some(epithet) = &details.epithet {
            let synonym = Details::get_from_name(genus.name, epithet, pool)
              .await?
              .ok_or(Error::NotFound)?;

            Some(synonym.id)
          } else {
            None
          };

          (Some(genus.id), epithet)
        },
        None => (None, None),
      }
    } else {
      (None, None)
    };

    let seed_parent: Option<Uuid> = if let Some(parent) = &details.seed_parent {
      let parent = Details::get_from_name(&parent.genus, &parent.epithet, pool)
        .await?
        .ok_or(Error::NotFound)?;

      Some(parent.id)
    } else {
      None
    };
    let pollen_parent: Option<Uuid> = if let Some(parent) = &details.pollen_parent {
      let parent = Details::get_from_name(&parent.genus, &parent.epithet, pool)
        .await?
        .ok_or(Error::NotFound)?;

      Some(parent.id)
    } else {
      None
    };

    let id = details.id as i32;

    let res = sqlx::query_file_as!(
      Self,
      "sql/details/insert.sql",
      id,
      genus.id,
      details.epithet,
      details.synonym,
      synonym_genus,
      synonym_epithet,
      details.registrant_name,
      details.originator_name,
      details.date_of_registration,
      seed_parent,
      pollen_parent
    )
    .fetch_one(pool)
    .await?;

    Ok(res)
  }

  pub async fn get_from_id(
    id: &Uuid,
    pool: impl PgExecutor<'_>,
  ) -> Result<Option<Self>, sqlx::Error> {
    sqlx::query_as!(Details, "SELECT * FROM details WHERE id = $1", id)
      .fetch_optional(pool)
      .await
  }

  pub async fn get_from_name(
    genus: impl AsRef<str>,
    epithet: impl AsRef<str>,
    pool: impl PgExecutor<'_>,
  ) -> Result<Option<Self>, sqlx::Error> {
    let genus = genus.as_ref();
    let epithet = epithet.as_ref();

    if epithet.contains('�') {
      let epithet = epithet.replace('�', "%");
      sqlx::query_file_as!(
        Self,
        "sql/details/get_from_name_unicode.sql",
        genus,
        epithet
      )
      .fetch_optional(pool)
      .await
    } else {
      sqlx::query_file_as!(Self, "sql/details/get_from_name.sql", genus, epithet)
        .fetch_optional(pool)
        .await
    }
  }
}

// impl From<crate::api::Details> for InsertDetails {
//   fn from(value: crate::api::Details) -> Self {
//     let crate::api::Details {
//       id,
//       genus,
//       epithet,
//       synonym,
//       synonym_details,
//       registrant_name,
//       originator_name,
//       date_of_registration,
//       seed_parent,
//       pollen_parent,
//     } = value;

//     Self {
//       id: None,
//       registar_id: id,
//       genus,
//       epithet,
//       synonym,
//       synonym_genus: None,
//       synonym_epithet: None,
//       registrant_name,
//       originator_name,
//       date_of_registration,
//       seed_parent: None,
//       pollen_parent: None,
//     }
//   }
// }
