use std::time::Duration;

use actix_web::{App, HttpServer};
use api::get_member_by_id;
use tokio::time::sleep;
use tracing::{Level, log::debug};
use dotenv::dotenv;

use crate::{notion::{client::fetch_data, cache::CacheStorage, types::NotionDataType}, api::*};


mod notion;
mod api;


static MAX_CACHE_AGE: Duration = Duration::from_secs(86400);


async fn update_all() {
  for data_type in NotionDataType::iterator() {
    CacheStorage::get().update(
      &data_type,
      fetch_data(&data_type).await.unwrap()
    ).await;
    sleep(Duration::from_millis(500)).await;
  }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
  tracing_subscriber::fmt()
    .with_max_level(Level::INFO)
    .init();

  dotenv().ok();

  tokio::spawn(
    async move {
      update_all().await;
      loop {
        debug!("Updating cache...");
        update_all().await;
        sleep(MAX_CACHE_AGE).await;
      }
    }
  );

  HttpServer::new(|| {
    App::new()
      .service(get_members)
      .service(get_member_by_id)
      .service(get_groups)
      .service(get_group_by_id)
      .service(get_clubs)
      .service(get_club_by_id)
      .service(get_events)
      .service(get_event_by_id)
      .service(get_articles)
      .service(get_article_by_id)
      .service(get_sponsors)
      .service(get_sponsor_by_id)
  })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
