use std::{collections::HashMap, path::Path};

use crate::{api::Details, csv::serde::CsvDetails};

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("This entry does not exist")]
  None,
  #[error(transparent)]
  Get(#[from] crate::api::get::Error),
  #[error(transparent)]
  Io(#[from] std::io::Error),
  #[error(transparent)]
  Csv(#[from] csv::Error),
}

pub struct Patches {
  data: HashMap<u32, Details>,
}

impl Patches {
  pub async fn from_data_file(path: impl AsRef<Path>) -> Result<Self, Error> {
    let path = path.as_ref();
    let file = tokio::fs::read(path).await?;
    let data: Result<Vec<CsvDetails>, _> = csv::ReaderBuilder::new()
      .comment(Some(b'#'))
      .from_reader(&file[..])
      .deserialize()
      .collect();
    let data = data?;

    let data = HashMap::from_iter(data.into_iter().map(|i| (i.0.id, i.0)));

    Ok(Self { data })
  }

  pub fn apply_all(&self, details: &mut HashMap<u32, Details>) {
    for patch in self.data.values() {
      details.insert(patch.id, patch.clone());
    }
  }
}
