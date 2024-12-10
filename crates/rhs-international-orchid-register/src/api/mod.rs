pub(crate) mod get;
mod parentage;
mod search;

pub use get::{Details, Get};

#[derive(Debug)]
pub struct SearchResult {
  pub genus: String,
  pub grex: String,
  pub id: u32,
}

pub enum Search {
  Parentage(parentage::ParentageSearch),
  Name(search::Search),
}

impl Search {
  pub async fn search(&self) -> Result<Vec<SearchResult>, Box<dyn std::error::Error>> {
    match self {
      Search::Parentage(parentage_search) => parentage_search.search().await,
      Search::Name(search) => search.search().await,
    }
  }
}
