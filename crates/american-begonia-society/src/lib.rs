use std::{
  collections::HashMap,
  fmt::Display,
  num::{IntErrorKind, ParseIntError},
  str::FromStr,
};

use scraper::{Html, Selector};
use time::{macros::format_description, Date};

mod csv;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Not found")]
  NotFound,
  #[error(transparent)]
  ParseInt(#[from] ParseIntError),
  #[error(transparent)]
  Reqwest(#[from] reqwest::Error),
  #[error(transparent)]
  Time(#[from] time::error::Parse),
}

#[derive(Clone, Copy, Debug, Default, serde::Serialize)]
pub struct Get {
  pub id: u32,
}

#[derive(Clone, Debug, serde::Serialize, PartialEq, Eq, Hash, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BegoniaType {
  CaneLike,
  RexCultorum,
  Rhizome,
  SemiTuberous,
  Semperflorens,
  ShrubLike,
  Superba,
  ThickStem,
  TrailingScandent,
  XTuberhybrida,
  XTuberhybridaPendula,
  Tuberous,
  Hiemalis,
}

#[derive(Clone, Debug)]
pub struct BegoniaTypeFromStrError {
  pub ty: String,
}

impl FromStr for BegoniaType {
  type Err = BegoniaTypeFromStrError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "cane-like" => Ok(Self::CaneLike),
      "rex cultorum" => Ok(Self::RexCultorum),
      "rhizome" => Ok(Self::Rhizome),
      "semi tuberous" => Ok(Self::SemiTuberous),
      "semperflorens" => Ok(Self::Semperflorens),
      "shrub-like" => Ok(Self::ShrubLike),
      "superba" => Ok(Self::Superba),
      "thick/stem" => Ok(Self::ThickStem),
      "trailing/scandent" => Ok(Self::TrailingScandent),
      "tuberous" => Ok(Self::Tuberous),
      "x tuberhybrida" => Ok(Self::XTuberhybrida),
      "x tuberhybrida pendula" => Ok(Self::XTuberhybridaPendula),
      "hiemalis" => Ok(Self::Hiemalis),
      other => Err(BegoniaTypeFromStrError {
        ty: other.to_string(),
      }),
    }
  }
}

impl Display for BegoniaType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let s = match self {
      Self::CaneLike => "cane-like",
      Self::RexCultorum => "rex cultorum",
      Self::Rhizome => "rhizome",
      Self::SemiTuberous => "semi tuberous",
      Self::Semperflorens => "semperflorens",
      Self::ShrubLike => "shrub-like",
      Self::Superba => "superba",
      Self::ThickStem => "thick/stem",
      Self::TrailingScandent => "trailing/scandent",
      Self::Tuberous => "tuberous",
      Self::XTuberhybrida => "x tuberhybrida",
      Self::XTuberhybridaPendula => "x tuberhybrida pendula",
      Self::Hiemalis => "hiemalis",
    };

    f.write_str(s)
  }
}

#[derive(Clone, Debug, serde::Serialize, PartialEq, Eq, Hash, serde::Deserialize)]
pub struct Synonym {
  pub genus: String,
  pub epithet: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Details {
  pub id: u32,
  pub name: String,
  pub ty: Vec<BegoniaType>,
  pub parents: String,
  pub hybridizer: String,
  pub location: String,
  pub origin: String,
  pub pub_date: String,
}

impl Get {
  pub async fn lookup(&self) -> Result<Details, Error> {
    let client = reqwest::Client::new();

    let res = client
      .get("https://www.begonias.org/cultivar-preservation/registered-cultivars/")
      .send()
      .await?;

    let html = res.text().await?;

    let html = Html::parse_document(&html);

    parse(self.id, html)
  }
}

fn to_string(el: scraper::ElementRef<'_>) -> String {
  let text = el.text().map(|t| t.to_string()).collect::<String>();

  text.trim().to_string()
}

fn parse(id: u32, html: Html) -> Result<Details, Error> {
  let table = Selector::parse("table > tbody").unwrap();

  let table = html.select(&table).next().unwrap();

  let mut details_map = HashMap::new();

  for row in table.child_elements() {
    let mut children = row.child_elements();

    let id = to_string(children.next().unwrap());
    let id = match id.parse() {
      Ok(id) => id,
      Err(e) => {
        let e: ParseIntError = e;
        let kind = e.kind();

        if *kind == IntErrorKind::Empty {
          continue;
        }

        Err(e)?
      },
    };

    let name = to_string(children.next().unwrap());

    let ty = to_string(children.next().unwrap());
    let ty = ty
      .split(",")
      .map(|ty| ty.trim().parse())
      .collect::<Result<Vec<_>, _>>()
      .unwrap();

    let parents = to_string(children.next().unwrap());
    let hybridizer = to_string(children.next().unwrap());
    let location = to_string(children.next().unwrap());
    let origin = to_string(children.next().unwrap());
    let pub_date = to_string(children.next().unwrap());

    details_map.insert(
      id,
      Details {
        id,
        name,
        ty,
        parents,
        hybridizer,
        location,
        origin,
        pub_date,
      },
    );
  }

  let details = details_map.get(&id).unwrap().clone();

  Ok(details)
}

#[tokio::test]
async fn get_id() {
  let id = Get { id: 94 };

  let details = id.lookup().await.unwrap();

  dbg!(details);
}
