use serde::{
  de::{MapAccess, Visitor},
  ser::SerializeStruct,
  Deserialize,
  Deserializer,
  Serialize,
};
use time::{macros::format_description, Date};

use crate::api::{
  get::{Parent, Synonym},
  Details,
};

pub(super) struct CsvDetails(pub Details);

impl Serialize for CsvDetails {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let Details {
      id,
      genus,
      epithet,
      synonym,
      synonym_details,
      registrant_name,
      originator_name,
      date_of_registration,
      seed_parent,
      pollen_parent,
    } = &self.0;

    let mut s = serializer.serialize_struct("details", 13)?;

    s.serialize_field("id", &id)?;
    s.serialize_field("genus", &genus)?;
    s.serialize_field("epithet", &epithet)?;
    s.serialize_field("synonym", &synonym)?;

    macro_rules! nested_field {
      ($pre:literal, $field:expr, $value:expr) => {
        s.serialize_field(concat!($pre, ".", $field), &$value)
      };
    }

    macro_rules! nested_genus_epithet {
      ($pre:literal, $value:ident) => {
        match $value {
          Some(value) => {
            nested_field!($pre, "genus", value.genus)?;
            nested_field!($pre, "epithet", value.epithet)?;
          },
          None => {
            nested_field!($pre, "genus", "")?;
            nested_field!($pre, "epithet", "")?;
          },
        }
      };
    }

    nested_genus_epithet!("synonym_details", synonym_details);

    s.serialize_field("registrant_name", &registrant_name)?;
    s.serialize_field("originator_name", &originator_name)?;
    s.serialize_field(
      "date_of_registration",
      &date_of_registration.map(|date| {
        let format = format_description!("[year]/[month]/[day]");
        date.format(format).unwrap()
      }),
    )?;

    nested_genus_epithet!("seed_parent", seed_parent);
    nested_genus_epithet!("pollen_parent", pollen_parent);

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
        let mut genus = None;
        let mut epithet = None;
        let mut synonym = None;
        let mut synonym_details_genus = None;
        let mut synonym_details_epithet = None;
        let mut registrant_name = None;
        let mut originator_name = None;
        let mut date_of_registration = None;
        let mut seed_parent_genus = None;
        let mut seed_parent_epithet = None;
        let mut pollen_parent_genus = None;
        let mut pollen_parent_epithet = None;

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
            "genus" => genus = next(&mut map)?,
            "epithet" => epithet = next(&mut map)?,
            "synonym" => synonym = Some(map.next_value()?),
            "synonym_details.genus" => synonym_details_genus = next(&mut map)?,
            "synonym_details.epithet" => synonym_details_epithet = Some(next(&mut map)?),
            "registrant_name" => registrant_name = next(&mut map)?,
            "originator_name" => originator_name = next(&mut map)?,
            "date_of_registration" => {
              if let Some(value) = next(&mut map)? {
                let format: &[time::format_description::FormatItem<'_>] =
                  format_description!("[year]/[month]/[day]");
                let value = Date::parse(&value, format).map_err(serde::de::Error::custom)?;
                date_of_registration = Some(value);
              }
            },
            "seed_parent.genus" => seed_parent_genus = next(&mut map)?,
            "seed_parent.epithet" => seed_parent_epithet = next(&mut map)?,
            "pollen_parent.genus" => pollen_parent_genus = next(&mut map)?,
            "pollen_parent.epithet" => pollen_parent_epithet = next(&mut map)?,
            _ => (),
          }
        }

        let details = || {
          macro_rules! details {
            ($ty:tt, $genus:ident, $epithet:ident) => {
              if let Some(genus) = $genus {
                let details = $ty {
                  genus,
                  epithet: $epithet.unwrap(),
                };
                Some(details)
              } else {
                None
              }
            };
          }

          let synonym_details = details!(Synonym, synonym_details_genus, synonym_details_epithet);
          let seed_parent = details!(Parent, seed_parent_genus, seed_parent_epithet);
          let pollen_parent = details!(Parent, pollen_parent_genus, pollen_parent_epithet);

          let details = Details {
            id: id?,
            genus: genus?,
            epithet: epithet?,
            synonym: synonym?,
            synonym_details,
            registrant_name,
            originator_name,
            date_of_registration,
            seed_parent,
            pollen_parent,
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
