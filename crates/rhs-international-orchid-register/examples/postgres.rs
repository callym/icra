use rhs_international_orchid_register::{
  csv::{Dump, Patches},
  sql::details::Details,
};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect("postgres://postgres:password@localhost:15432/postgres")
    .await?;

  sqlx::migrate!("./migrations").run(&pool).await?;

  std::env::set_current_dir(env!("CARGO_MANIFEST_DIR")).unwrap();

  let mut dump =
    Dump::from_data_and_known_bad_files("./data/dump.csv", "./data/known_bad.csv").await?;

  let patches = Patches::from_data_file("./data/patches.csv").await?;

  patches.apply_all(dump.data_mut());

  let mut details = dump.data().clone();
  let len = details.len();

  // for details in details.values() {
  //   Genus::insert(&details.genus, &pool).await?;
  // }

  // https://orchidex.org/phalaenopsis/wisley-happy-sun/967153
  // let d = dump.get(Get { id: 1601 }).unwrap();
  // dbg!(&d);
  // Details::insert(d, &pool).await.unwrap();

  let mut prev_len = len;
  let mut asc = true;
  while !details.is_empty() {
    let mut values = details.values().cloned().collect::<Vec<_>>();

    if asc {
      values.sort_by_key(|v| v.date_of_registration);
    } else {
      values.sort_by_key(|v| v.date_of_registration);
      values.reverse();
    }

    println!("{}/{}", details.len(), len);

    for value in &values {
      let res = Details::insert(value, &pool).await;

      if res.is_ok() {
        details.remove(&value.id);
      }
    }

    if prev_len == details.len() {
      println!("{}/{}", details.len(), len);
      dbg!(&values[0]);
      dbg!(Details::insert(&values[0], &pool).await);

      if asc {
        asc = false;
        continue;
      }

      break;
    }

    prev_len = details.len();
    asc = true;
  }

  Ok(())
}
