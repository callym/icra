use serde::{
  de::{MapAccess, Visitor},
  ser::SerializeStruct,
  Deserialize,
  Deserializer,
  Serialize,
};
use time::{macros::format_description, Date};

use crate::{BegoniaType, Details, Get};

pub(super) struct CsvDetails(pub Details);

impl Serialize for CsvDetails {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let Details {
      id,
      name,
      ty,
      parents,
      hybridizer,
      location,
      origin,
      pub_date,
    } = &self.0;

    let mut s = serializer.serialize_struct("details", 13)?;

    s.serialize_field("id", &id)?;
    s.serialize_field("name", &name)?;
    s.serialize_field(
      "ty",
      &ty
        .iter()
        .map(|ty| ty.to_string())
        .collect::<Vec<_>>()
        .join(","),
    )?;
    s.serialize_field("parents", &parents)?;
    s.serialize_field("hybridizer", &hybridizer)?;
    s.serialize_field("location", &location)?;
    s.serialize_field("origin", &origin)?;
    s.serialize_field("pub_date", &pub_date)?;

    s.end()
  }
}

impl<'de> Deserialize<'de> for CsvDetails {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct CsvDetailsVisitor;

    impl<'de> Visitor<'de> for CsvDetailsVisitor {
      type Value = CsvDetails;

      fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("struct CsvDetails")
      }

      fn visit_map<V>(self, mut map: V) -> Result<CsvDetails, V::Error>
      where
        V: MapAccess<'de>,
      {
        let mut id = None;
        let mut name = None;
        let mut ty = None;
        let mut parents = None;
        let mut hybridizer = None;
        let mut location = None;
        let mut origin = None;
        let mut pub_date = None;

        let next = |map: &mut V| {
          let value: String = map.next_value()?;

          if value.trim().is_empty() {
            Ok(None)
          } else {
            Ok(Some(value))
          }
        };

        while let Some(key) = map.next_key()? {
          match key {
            "id" => id = Some(map.next_value()?),
            "name" => name = next(&mut map)?,
            "ty" => {
              ty = {
                let s: String = map.next_value()?;
                Some(
                  s.split(",")
                    .map(|s| s.trim().parse())
                    .collect::<Result<Vec<BegoniaType>, _>>()
                    .map_err(|e| serde::de::Error::custom(format!("{:?}", e)))?,
                )
              }
            },
            "parents" => parents = next(&mut map)?,
            "hybridizer" => hybridizer = next(&mut map)?,
            "location" => location = next(&mut map)?,
            "origin" => origin = next(&mut map)?,
            "pub_date" => pub_date = next(&mut map)?,
            _ => (),
          }
        }

        let details = || {
          let details = Details {
            id: id?,
            name: name?,
            ty: ty?,
            parents: parents?,
            hybridizer: hybridizer?,
            location: location?,
            origin: origin?,
            pub_date: pub_date?,
          };

          Some(details)
        };

        let details = details().unwrap();

        Ok(CsvDetails(details))
      }
    }

    const FIELDS: &[&str] = &[];
    deserializer.deserialize_struct("CsvDetails", FIELDS, CsvDetailsVisitor)
  }
}

mod tests {
  use super::*;

  const CSV: &str = "id,name,ty,parents,hybridizer,location,origin,pub_date\n94,Mottled Sheen,rhizome,Joe Hayden x bowerae,Don Horton,USA - CA,1952,1954 Sep p. 209\n";

  #[tokio::test]
  async fn to_csv() {
    let details = Get { id: 94 }.lookup().await.unwrap();
    let details = CsvDetails(details);

    let mut writer = csv::Writer::from_writer(Vec::new());
    writer.serialize(details).unwrap();

    let s = String::from_utf8(writer.into_inner().unwrap()).unwrap();

    assert_eq!(CSV, s);
  }

  #[tokio::test]
  async fn from_csv() {
    let mut reader = csv::Reader::from_reader(CSV.as_bytes());
    let data = reader
      .deserialize::<CsvDetails>()
      .map(|d| d.map(|d| d.0))
      .collect::<Result<Vec<Details>, _>>()
      .unwrap();

    dbg!(data);
  }
}
