use std::{
  collections::{HashMap, HashSet},
  path::Path,
};

use crate::{
  api::{Details, Get},
  csv::{serde::CsvDetails, KnownBad},
};

#[derive(Clone, Debug)]
pub struct Dump {
  data: HashMap<u32, Details>,
  known_bad: HashSet<crate::csv::KnownBad>,
}

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

impl Dump {
  pub fn empty() -> Self {
    Self {
      data: HashMap::new(),
      known_bad: HashSet::new(),
    }
  }

  pub async fn from_data_file(path: impl AsRef<Path>) -> Result<Self, Error> {
    let path = path.as_ref();
    let file = tokio::fs::read(path).await?;
    let data: Result<Vec<CsvDetails>, _> =
      csv::Reader::from_reader(&file[..]).deserialize().collect();
    let data = data?;

    let data = HashMap::from_iter(data.into_iter().map(|i| (i.0.id, i.0)));

    Ok(Self {
      data,
      known_bad: HashSet::new(),
    })
  }

  pub async fn from_data_and_known_bad_files(
    data: impl AsRef<Path>,
    known_bad: impl AsRef<Path>,
  ) -> Result<Self, Error> {
    let mut s = Self::from_data_file(data).await?;

    let known_bad = known_bad.as_ref();
    let file = tokio::fs::read(known_bad).await?;
    let data: Result<Vec<KnownBad>, _> = {
      let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(&file[..]);

      reader.deserialize().collect()
    };
    let data = data?;

    let data = HashSet::from_iter(data.into_iter());

    s.known_bad = data;

    Ok(s)
  }

  pub async fn write(
    &self,
    data: impl AsRef<Path>,
    known_bad: impl AsRef<Path>,
  ) -> Result<(), Error> {
    let path = data.as_ref();

    let mut data = self.data.clone().into_values().collect::<Vec<_>>();
    data.sort_by_key(|e| e.id);

    let mut writer = csv::WriterBuilder::new().from_writer(Vec::new());

    for entry in data {
      writer.serialize(CsvDetails(entry)).unwrap();
    }

    tokio::fs::write(path, writer.into_inner().unwrap()).await?;

    let path = known_bad.as_ref();

    let mut data = self.known_bad.clone().into_iter().collect::<Vec<_>>();
    data.sort();

    let mut writer = csv::WriterBuilder::new()
      .has_headers(false)
      .from_writer(Vec::new());

    for entry in data {
      writer.serialize(entry).unwrap();
    }

    tokio::fs::write(path, writer.into_inner().unwrap()).await?;

    Ok(())
  }

  pub fn has(&self, get: Get) -> bool {
    self.data.contains_key(&get.id)
  }

  pub fn last(&self) -> u32 {
    self.data.keys().max().cloned().unwrap_or_default()
  }

  pub fn get(&self, get: Get) -> Option<&Details> {
    self.data.get(&get.id)
  }

  pub async fn get_or_insert(&mut self, get: Get) -> Result<&Details, Error> {
    if self.is_known_bad(get) {
      Err(Error::None)
    } else if self.has(get) {
      self.get(get).ok_or(Error::None)
    } else {
      let entry = get.lookup().await?;
      let id = entry.id;
      self.data.insert(id, entry);
      Ok(self.get(get).unwrap())
    }
  }

  pub fn insert(&mut self, details: Details) {
    self.data.insert(details.id, details);
  }

  pub fn iter(&self) -> impl Iterator<Item = &Details> {
    self.data.values()
  }

  pub fn data(&self) -> &HashMap<u32, Details> {
    &self.data
  }

  pub fn data_mut(&mut self) -> &mut HashMap<u32, Details> {
    &mut self.data
  }

  fn is_known_bad(&self, get: Get) -> bool {
    self.known_bad.iter().any(|i| i.contains(get.id))
  }
}

#[tokio::test]
async fn save() -> Result<(), Error> {
  let mut dump = Dump::empty();

  dump.get_or_insert(Get { id: 1437 }).await?;
  dump.get_or_insert(Get { id: 1475 }).await?;
  dump.get_or_insert(Get { id: 1064207 }).await?;
  dump.get_or_insert(Get { id: 134249 }).await?;

  dump.write("dump2.csv", "known_bad2.csv").await?;

  Ok(())
}

#[tokio::test]
async fn load_up_to() -> Result<(), Error> {
  let mut dump = Dump::from_data_and_known_bad_files("data/dump.csv", "data/known_bad.csv").await?;

  for i in 0..100 {
    if let Ok(details) = dump.get_or_insert(Get { id: i }).await {
      println!("{i}: {} {}", details.genus, details.epithet);
    } else {
      println!("{i}: not found");
    }
  }

  Ok(())
}

#[tokio::test]
async fn load() -> Result<(), Error> {
  Dump::from_data_and_known_bad_files("dump.csv", "known_bad.csv").await?;

  Ok(())
}

#[tokio::test]
async fn round_trip() -> Result<(), Error> {
  let dump = Dump::from_data_and_known_bad_files("data/dump.csv", "data/known_bad.csv").await?;

  dump.write("data/data2.csv", "data/known_bad2.csv").await?;

  Ok(())
}
