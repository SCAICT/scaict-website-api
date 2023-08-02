use axum::{
  extract::Path,
  http::{header, HeaderMap, StatusCode},
  Json,
  response::{Response, IntoResponse}
};
use serde_json::json;
use tracing::log::debug;

use crate::notion::{
  types::NotionDataType,
  cache::CacheStorage,
  client::fetch_data
};


static API_VERSION: &str = "1.0.0";
static ROBOTS_TXT: &str = r#"
User-agent: *

Disallow: /members
Disallow: /groups
Disallow: /clubs
Disallow: /sponsors
"#;

async fn handle_no_cache(
  headers: &HeaderMap,
  data_type: &NotionDataType
) {
  if let Some(
    cache_control
  ) = headers.get(header::CACHE_CONTROL) {
    if cache_control.to_str().unwrap_or("") == "no-cache" {
      debug!("Receive `no-cache`, cleaning cache...");
      CacheStorage::get().update(
        &data_type,
        fetch_data(&data_type).await.unwrap()
      ).await;
    }
  }
}

pub async fn get_robots_txt() -> Response {
  (
    StatusCode::OK,
    ROBOTS_TXT
  ).into_response()
}

pub async fn get_version() -> Response {
  (
    StatusCode::OK,
    Json(
      json!(
        {"version": API_VERSION}
      )
    )
  ).into_response()
}

pub async fn get_members(
  headers: HeaderMap
) -> Response {
  handle_no_cache(
    &headers,
    &NotionDataType::Member
  ).await;

  (
    StatusCode::OK,
    Json(
      CacheStorage::get().request_all(
        &NotionDataType::Member
      ).await
    )
  ).into_response()
}

pub async fn get_member_by_id(
  headers: HeaderMap,
  Path(id): Path<String>
) -> Response {
  handle_no_cache(
    &headers,
    &NotionDataType::Member
  ).await;

  match CacheStorage::get().request(
    &id.to_string(),
    &NotionDataType::Member
  ).await {
    Some(data) => (
      StatusCode::OK,
      Json(data)
    ).into_response(),
    None => StatusCode::NOT_FOUND.into_response()
  }
}

pub async fn get_groups(
  headers: HeaderMap
) -> Response {
  handle_no_cache(
    &headers,
    &NotionDataType::Group
  ).await;

  (
    StatusCode::OK,
    Json(
      CacheStorage::get().request_all(
        &NotionDataType::Group
      ).await
    )
  ).into_response()
}

pub async fn get_group_by_id(
  headers: HeaderMap,
  Path(id): Path<String>
) -> Response {
  handle_no_cache(
    &headers,
    &NotionDataType::Group
  ).await;

  match CacheStorage::get().request(
    &id.to_string(),
    &NotionDataType::Group
  ).await {
    Some(data) => (
      StatusCode::OK,
      Json(data)
    ).into_response(),
    None => StatusCode::NOT_FOUND.into_response()
  }
}

pub async fn get_clubs(
  headers: HeaderMap
) -> Response {
  handle_no_cache(
    &headers,
    &NotionDataType::Club
  ).await;

  (
    StatusCode::OK,
    Json(
      CacheStorage::get().request_all(
        &NotionDataType::Club
      ).await
    )
  ).into_response()
}

pub async fn get_club_by_id(
  headers: HeaderMap,
  Path(id): Path<String>
) -> Response {
  handle_no_cache(
    &headers,
    &NotionDataType::Club
  ).await;

  match CacheStorage::get().request(
    &id.to_string(),
    &NotionDataType::Club
  ).await {
    Some(data) => (
      StatusCode::OK,
      Json(data)
    ).into_response(),
    None => StatusCode::NOT_FOUND.into_response()
  }
}

pub async fn get_events(
  headers: HeaderMap
) -> Response {
  handle_no_cache(
    &headers,
    &NotionDataType::Event
  ).await;

  (
    StatusCode::OK,
    Json(
      CacheStorage::get().request_all(
        &NotionDataType::Event
      ).await
    )
  ).into_response()
}

pub async fn get_event_by_id(
  headers: HeaderMap,
  Path(id): Path<String>
) -> Response {
  handle_no_cache(
    &headers,
    &NotionDataType::Event
  ).await;

  match CacheStorage::get().request(
    &id.to_string(),
    &NotionDataType::Event
  ).await {
    Some(data) => (
      StatusCode::OK,
      Json(data)
    ).into_response(),
    None => StatusCode::NOT_FOUND.into_response()
  }
}


pub async fn get_articles(
  headers: HeaderMap
) -> Response {
  handle_no_cache(
    &headers,
    &NotionDataType::Article
  ).await;

  (
    StatusCode::OK,
    Json(
      CacheStorage::get().request_all(
        &NotionDataType::Article
      ).await
    )
  ).into_response()
}

pub async fn get_article_by_id(
  headers: HeaderMap,
  Path(id): Path<String>
) -> Response {
  handle_no_cache(
    &headers,
    &NotionDataType::Article
  ).await;

  match CacheStorage::get().request(
    &id.to_string(),
    &NotionDataType::Article
  ).await {
    Some(data) => (
      StatusCode::OK,
      Json(data)
    ).into_response(),
    None => StatusCode::NOT_FOUND.into_response()
  }
}

pub async fn get_sponsors(
  headers: HeaderMap
) -> Response {
  handle_no_cache(
    &headers,
    &NotionDataType::Sponsor
  ).await;

  (
    StatusCode::OK,
    Json(
      CacheStorage::get().request_all(
        &NotionDataType::Sponsor
      ).await
    )
  ).into_response()
}

pub async fn get_sponsor_by_id(
  headers: HeaderMap,
  Path(id): Path<String>
) -> Response {
  handle_no_cache(
    &headers,
    &NotionDataType::Sponsor
  ).await;

  match CacheStorage::get().request(
    &id.to_string(),
    &NotionDataType::Sponsor
  ).await {
    Some(data) => (
      StatusCode::OK,
      Json(data)
    ).into_response(),
    None => StatusCode::NOT_FOUND.into_response()
  }
}
