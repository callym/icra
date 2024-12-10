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
pub struct Details {
  pub id: u32,
  pub genus: String,
  pub epithet: String,
  pub synonym: bool,
  pub synonym_details: Option<Synonym>,
  pub registrant_name: Option<String>,
  pub originator_name: Option<String>,
  pub date_of_registration: Option<Date>,
  pub seed_parent: Option<Parent>,
  pub pollen_parent: Option<Parent>,
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
      .get("https://apps.rhs.org.uk/horticulturaldatabase/orchidregister/orchiddetails.asp")
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
  let table = Selector::parse("table.results").unwrap();
  let tbody = Selector::parse("tbody").unwrap();

  let mut tables = html.select(&table);

  let details_table = tables
    .next()
    .ok_or(Error::NotFound)?
    .select(&tbody)
    .next()
    .ok_or(Error::NotFound)?;

  let parent_table = tables
    .next()
    .map(|el| el.select(&tbody).next().ok_or(Error::NotFound));

  let mut details = Details {
    id,
    genus: String::new(),
    epithet: String::new(),
    synonym: false,
    synonym_details: None,
    registrant_name: None,
    originator_name: None,
    date_of_registration: None,
    seed_parent: None,
    pollen_parent: None,
  };

  for el in details_table.select(&Selector::parse("tr").unwrap()) {
    let mut children = el.child_elements();

    let key = children.next().unwrap();
    let value = children.next().unwrap();

    let key = to_string(key);
    let value = to_string(value);

    match key.as_str() {
      "Genus" => details.genus = value,
      "Epithet" => details.epithet = value,
      "Synonym Flag" => details.synonym = !value.contains("not"),
      "Synonym Genus Name" => match &mut details.synonym_details {
        Some(details) => details.genus = value,
        None => {
          details.synonym_details = Some(Synonym {
            genus: value,
            epithet: None,
          })
        },
      },
      "Synonym Epithet Name" => match &mut details.synonym_details {
        Some(details) => details.epithet = Some(value),
        None => {
          details.synonym_details = Some(Synonym {
            genus: details.genus.clone(),
            epithet: Some(value),
          })
        },
      },
      "Registrant Name" if name_exists(&value) => details.registrant_name = Some(value),
      "Originator Name" if name_exists(&value) => details.originator_name = Some(value),
      "Date of registration" => {
        let format = format_description!("[day]/[month]/[year]");
        let date = Date::parse(&value, &format)?;
        details.date_of_registration = Some(date)
      },
      _ => (),
    }
  }

  if let Some(Ok(el)) = parent_table {
    let mut children = el.child_elements().map(|el| {
      let mut children = el.child_elements();

      let key = to_string(children.next().unwrap());
      let seed = to_string(children.next().unwrap());
      let pollen = to_string(children.next().unwrap());

      (key, seed, pollen)
    });

    let (genus_key, seed_genus, pollen_genus) = children.next().unwrap();
    let (epithet_key, seed_epithet, pollen_epithet) = children.next().unwrap();

    assert_eq!(genus_key, "Genus");
    assert_eq!(epithet_key, "Epithet");

    details.seed_parent = Some(Parent {
      genus: seed_genus,
      epithet: seed_epithet,
    });

    details.pollen_parent = Some(Parent {
      genus: pollen_genus,
      epithet: pollen_epithet,
    });
  }

  if details.genus.is_empty() {
    Err(Error::NotFound)?;
  }

  if details.epithet.is_empty() {
    Err(Error::NotFound)?;
  }

  Ok(details)
}

#[tokio::test]
async fn latest() -> Result<(), Box<dyn std::error::Error>> {
  let mut id = 1064207;

  let mut res = None;
  loop {
    let get = Get { id };

    let new_res = get.lookup().await;

    if let Ok(new_res) = new_res {
      res = Some(new_res);

      id += 1;
    } else {
      break;
    }
  }

  dbg!(res);

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
