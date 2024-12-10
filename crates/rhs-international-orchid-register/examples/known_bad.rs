use std::fmt::Display;

use serde::de::Visitor;

#[derive(Debug, serde::Serialize)]
enum KnownBad {
  Single(u32),
  Range(std::ops::RangeInclusive<u32>),
}

impl<'de> serde::Deserialize<'de> for KnownBad {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    struct KnownBadVisitor;

    impl<'de> Visitor<'de> for KnownBadVisitor {
      type Value = KnownBad;

      fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("enum KnownBad")
      }

      fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        self.visit_str(&v)
      }

      fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
      where
        E: serde::de::Error,
      {
        let split = v.split("..=").collect::<Vec<_>>();

        let res = match split.len() {
          1 => KnownBad::Single(split[0].parse().unwrap()),
          2 => KnownBad::Range(split[0].parse().unwrap()..=split[1].parse().unwrap()),
          i => Err(E::invalid_length(i, &self))?,
        };

        Ok(res)
      }
    }

    deserializer.deserialize_str(KnownBadVisitor)
  }
}

impl Display for KnownBad {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      KnownBad::Single(i) => i.fmt(f),
      KnownBad::Range(i) => f.write_fmt(format_args!("{}..={}", i.start(), i.end())),
    }
  }
}

fn main() {
  let csv = std::fs::read_to_string("known_bad_fmt.csv").unwrap();

  let mut csv = csv::Reader::from_reader(csv.as_bytes());

  for res in csv.deserialize() {
    let res: KnownBad = res.unwrap();
    dbg!(&res);
    dbg!(res.to_string());
  }
}
