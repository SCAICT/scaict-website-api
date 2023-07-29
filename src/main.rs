use actix_web::{App, web, HttpServer};
use tracing::Level;
use dotenv::dotenv;

use crate::notion::{client::fetch_data, cache::CacheStorage, types::{NotionDataType, NotionData}};

mod notion;

#[tokio::main]
async fn main() -> std::io::Result<()> {
  tracing_subscriber::fmt()
    .with_max_level(Level::DEBUG)
    .init();
  
  dotenv().ok();
  
  CacheStorage::get().update(
    NotionDataType::Club,
    fetch_data(NotionDataType::Club).await.unwrap()
  ).await;
  CacheStorage::get().update(
    NotionDataType::Group,
    fetch_data(NotionDataType::Group).await.unwrap()
  ).await;
  CacheStorage::get().update(
    NotionDataType::Member,
    fetch_data(NotionDataType::Member).await.unwrap()
  ).await;

  println!("{:#?}", CacheStorage::get().request_all(NotionDataType::Member).await);

  // HttpServer::new(|| {
  //   App::new()
  // })
  //   .bind(("127.0.0.1", 8080))?
  //   .run()
  //   .await

  Ok(())
}
