use std::str::FromStr;

use scraper::{Html, Selector};

use crate::api::SearchResult;

#[derive(Debug, Default, serde::Serialize)]
pub struct ParentageSearch {
  #[serde(rename = "seedgen")]
  pub seed_genus: Option<String>,
  #[serde(rename = "seedgrex")]
  pub seed_grex: Option<String>,
  #[serde(rename = "pollgen")]
  pub pollen_genus: Option<String>,
  #[serde(rename = "pollgrex")]
  pub pollen_grex: Option<String>,
}

impl ParentageSearch {
  pub async fn search(&self) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let res = client
      .post("https://apps.rhs.org.uk/horticulturaldatabase/orchidregister/parentageresults.asp")
      .form(self)
      .send()
      .await?;

    let html = res.text().await?;
    let html = Html::parse_document(&html);

    parse(html)
  }
}

fn parse(html: Html) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
  let table = Selector::parse("table.results")?;
  let table = html
    .select(&table)
    .next()
    .unwrap()
    .select(&Selector::parse("tbody")?)
    .next()
    .unwrap();

  let mut current_genus = None;
  let mut res = Vec::new();
  for el in table.select(&Selector::parse("tr")?) {
    let mut children = el.child_elements();
    let genus = children.next().unwrap();
    let grex = children.next().unwrap();

    if let Some(genus) = genus.child_elements().next() {
      current_genus = Some(genus.inner_html());
    }

    if let Some(grex) = grex.child_elements().next() {
      let href = grex.attr("href").unwrap();
      let href = href.strip_prefix("orchiddetails.asp?ID=").unwrap();
      let href = u32::from_str(href)?;

      let text = grex.text().next().unwrap();

      res.push(SearchResult {
        genus: current_genus.clone().unwrap(),
        grex: text.into(),
        id: href,
      })
    }
  }

  Ok(res)
}

#[tokio::test]
async fn rhs() -> Result<(), Box<dyn std::error::Error>> {
  let search = ParentageSearch {
    seed_genus: Some("Phalaenopsis".into()),
    seed_grex: Some("schilleriana".into()),
    ..Default::default()
  };

  let res = search.search().await?;

  dbg!(res);

  Ok(())
}
