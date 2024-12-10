use std::{collections::HashSet, sync::Arc};

use rhs_international_orchid_register::{
  api::{Details, Get},
  csv::{Dump, DumpError},
};
use tokio::{io::AsyncWriteExt as _, sync::RwLock};

const START: u32 = 816;
const END: u32 = 1_066_954;
const CHUNK: u32 = 10;

#[tokio::main]
async fn main() -> Result<(), DumpError> {
  let mut known_bad: HashSet<u32> = HashSet::new();
  let mut reader = csv::ReaderBuilder::new()
    .has_headers(false)
    .from_path("known_bad.csv")?;

  for record in reader.deserialize() {
    let record = record?;
    known_bad.insert(record);
  }

  let dump = Dump::from_data_and_known_bad_files("dump.csv", "known_bad.csv").await?;
  let mut start = START;
  println!("starting at {start}");

  let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel();

  let dump = Arc::new(RwLock::new(dump));
  let known_bad = Arc::new(RwLock::new(known_bad));

  let join = tokio::spawn({
    let dump = dump.clone();
    let known_bad = known_bad.clone();

    async move {
      while let Some(res) = receiver.recv().await {
        let mut dump = dump.write().await;

        for (i, details) in res {
          if let Ok(details) = details {
            let details: Details = details;
            println!("{i}: {} {}", details.genus, details.epithet);
            dump.insert(details);
          } else {
            println!("{i}: not found");
            let mut known_bad = known_bad.write().await;
            known_bad.insert(i);

            let mut file = tokio::fs::OpenOptions::new()
              .append(true)
              .open("known_bad.csv")
              .await
              .unwrap();
            file.write_all(format!("{i}\n").as_bytes()).await.unwrap();
            file.flush().await.unwrap();
          }
        }

        dump.write("dump.csv", "known_bad.csv").await.unwrap();
      }
    }
  });

  let semaphore = tokio::sync::Semaphore::new(10);
  let semaphore = Box::leak(Box::new(semaphore));

  while start <= END {
    {
      let dump = dump.read().await;
      let known_bad = known_bad.read().await;
      if dump.has(Get { id: start }) || known_bad.contains(&start) {
        start += 1;

        continue;
      }
    }

    let sender = sender.clone();
    let dump = dump.clone();
    let known_bad = known_bad.clone();
    let acq = semaphore.acquire().await.unwrap();

    tokio::spawn(async move {
      let acq = acq;
      let mut res = Vec::new();

      println!("getting {start}");

      for i in start..start + CHUNK {
        {
          let known_bad = known_bad.read().await;
          if known_bad.contains(&i) {
            continue;
          }
        }

        {
          let dump = dump.read().await;
          if dump.has(Get { id: i }) {
            continue;
          }
        }

        let get = Get { id: i };
        let details = get.lookup().await;
        res.push((i, details));
      }
      sender.send(res).unwrap();
      std::mem::drop(acq);
    });

    start += CHUNK;
  }

  join.await.unwrap();

  Ok(())
}
