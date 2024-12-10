use std::fmt::Display;

use serde::de::Visitor;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KnownBad {
  Single(u32),
  Range(std::ops::RangeInclusive<u32>),
}

impl KnownBad {
  pub fn contains(&self, value: u32) -> bool {
    match self {
      KnownBad::Single(i) => *i == value,
      KnownBad::Range(i) => i.contains(&value),
    }
  }

  fn start(&self) -> u32 {
    match self {
      KnownBad::Single(i) => *i,
      KnownBad::Range(i) => *i.start(),
    }
  }
}

impl PartialOrd for KnownBad {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for KnownBad {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    Ord::cmp(&self.start(), &other.start())
  }
}

impl serde::Serialize for KnownBad {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(&self.to_string())
  }
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
