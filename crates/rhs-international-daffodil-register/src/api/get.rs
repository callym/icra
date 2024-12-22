use scraper::{Html, Selector};
use time::{macros::format_description, Date};

#[derive(Clone, Copy, Debug, Default, serde::Serialize)]
pub struct Get {
  pub id: u32,
}

#[derive(Clone, Debug, serde::Serialize, PartialEq, Eq, Hash, serde::Deserialize)]
pub struct Parent {
  pub genus: String,
  pub epithet: String,
}

#[derive(Clone, Debug, serde::Serialize, PartialEq, Eq, Hash, serde::Deserialize)]
pub struct Synonym {
  pub genus: String,
  pub epithet: Option<String>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub enum FirstFlowering {
  Pre(u32),
  Year(u32),
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Details {
  pub id: u32,
  pub name: String,
  pub registered: bool,
  pub division: String,
  pub perianth_colors: String,
  pub originator_name: String,
  pub first_flowering_date: FirstFlowering,
  // pub synonyms: String,
  // pub awards: String,
  pub last_modified: Option<Date>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("Not found")]
  NotFound,
  #[error(transparent)]
  Reqwest(#[from] reqwest::Error),
  #[error(transparent)]
  Time(#[from] time::error::Parse),
}

impl Get {
  pub async fn lookup(&self) -> Result<Details, Error> {
    let client = reqwest::Client::new();

    let res = client
      .get("https://apps.rhs.org.uk/horticulturaldatabase/daffodilregister/daffdetails.asp")
      .query(&[("ID", self.id)])
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

fn name_exists(name: &str) -> bool {
  !matches!(name, "O/U" | "Unknown" | "")
}

fn parse(id: u32, html: Html) -> Result<Details, Error> {
  let name = Selector::parse("h2.specimen").unwrap();
  let table = Selector::parse("table.results").unwrap();
  let tbody = Selector::parse("tbody").unwrap();

  let name = html.select(&name).next().ok_or(Error::NotFound)?;
  let name = to_string(name);

  let mut tables = html.select(&table);

  let details_table = tables
    .next()
    .ok_or(Error::NotFound)?
    .select(&tbody)
    .next()
    .ok_or(Error::NotFound)?;

  let mut details = Details {
    id,
    name,
    registered: false,
    division: String::new(),
    perianth_colors: String::new(),
    originator_name: String::new(),
    first_flowering_date: FirstFlowering::Pre(0),
    last_modified: None,
  };

  for el in details_table.select(&Selector::parse("tr").unwrap()) {
    let mut children = el.child_elements();

    let key = children.next().unwrap();
    let value = children.next().unwrap();

    let key = to_string(key);
    let value = to_string(value);

    match key.as_str() {
      "Registered?" => details.registered = value == "Yes",
      "Division" => details.division = value,
      "Perianth colour(s)" => details.perianth_colors = value,
      "Originator name" => details.originator_name = value,
      "Date of first flowering" => {
        if value.starts_with("pre") {
          let value = value.split_once("pre");
          if let Some((_, year)) = value {
            let year = year.trim();
            details.first_flowering_date = FirstFlowering::Pre(year.parse().unwrap());
          }
        } else if value.starts_with("c.") {
          let value = value.split_once("c.");
          if let Some((_, year)) = value {
            let year = year.trim();
            details.first_flowering_date = FirstFlowering::Pre(year.parse().unwrap());
          }
        } else {
          details.first_flowering_date = FirstFlowering::Year(value.parse().unwrap());
        }
      },
      "Date when entry last modified" => {
        let format = format_description!("[day] [month repr:long] [year]");
        let date = Date::parse(&value, &format)?;
        details.last_modified = Some(date);
      },
      _ => (),
    }
  }

  // if details.genus.is_empty() {
  //   Err(Error::NotFound)?;
  // }

  // if details.epithet.is_empty() {
  //   Err(Error::NotFound)?;
  // }

  Ok(details)
}

#[tokio::test]
async fn get() -> Result<(), Box<dyn std::error::Error>> {
  let start = 200_001;
  let start = 200067;
  for i in start..start + 100 {
    let get: Get = Get { id: i };
    let res = get.lookup().await?;

    dbg!(res);
  }

  Ok(())
}

#[tokio::test]
async fn species() -> Result<(), Box<dyn std::error::Error>> {
  let get: Get = Get { id: 1437 };
  let res = get.lookup().await?;

  dbg!(res);

  Ok(())
}

#[tokio::test]
async fn species_synonym() -> Result<(), Box<dyn std::error::Error>> {
  let get: Get = Get { id: 1475 };
  let res = get.lookup().await?;

  dbg!(res);

  Ok(())
}

#[tokio::test]
async fn hybrid() -> Result<(), Box<dyn std::error::Error>> {
  let get = Get { id: 1064207 };
  let res = get.lookup().await?;

  dbg!(res);

  Ok(())
}

#[tokio::test]
async fn hybrid_synonym() -> Result<(), Box<dyn std::error::Error>> {
  let get = Get { id: 134249 };
  let res = get.lookup().await?;

  dbg!(res);

  Ok(())
}
