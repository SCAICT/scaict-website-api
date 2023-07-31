use actix_web::{
  get,
  HttpResponse,
  http::header::{ContentType, self},
  web,
  HttpRequest
};
use serde_json::json;
use tracing::log::debug;

use crate::notion::{
  types::NotionDataType,
  cache::CacheStorage,
  client::fetch_data
};


static API_VERSION: &str = "1.0.0";


async fn handle_no_cache(req: &HttpRequest, data_type: &NotionDataType) {
  if let Some(cache_control) = req.headers().get(header::CACHE_CONTROL) {
    if cache_control.to_str().unwrap_or("") == "no-cache" {
      debug!("Receive `no-cache`, cleaning cache...");
      CacheStorage::get().update(
        &data_type,
        fetch_data(&data_type).await.unwrap()
      ).await;
    }
  }
}

#[get("/version")]
async fn get_version() -> HttpResponse {
  HttpResponse::Ok()
    .content_type(ContentType::json())
    .json(
      json!(
        {
          "version": API_VERSION
        }
      )
    )
}

#[get("/members")]
async fn get_members(req: HttpRequest) -> HttpResponse {
  handle_no_cache(&req, &NotionDataType::Member).await;

  HttpResponse::Ok()
    .content_type(ContentType::json())
    .json(
      CacheStorage::get().request_all(
        &NotionDataType::Member
      ).await
    )
}

#[get("/members/{id}")]
async fn get_member_by_id(req: HttpRequest, id: web::Path<String>) -> HttpResponse {
  handle_no_cache(&req, &NotionDataType::Member).await;

  match CacheStorage::get().request(
    &id.to_string(),
    &NotionDataType::Member
  ).await {
    Some(data) => {
      HttpResponse::Ok().content_type(ContentType::json()).json(data)
    },
    None => HttpResponse::NotFound().finish()
  }
}

#[get("/groups")]
async fn get_groups(req: HttpRequest) -> HttpResponse {
  handle_no_cache(&req, &NotionDataType::Member).await;

  HttpResponse::Ok()
    .content_type(ContentType::json())
    .json(
      CacheStorage::get().request_all(
        &NotionDataType::Group
      ).await
    )
}

#[get("/groups/{id}")]
async fn get_group_by_id(req: HttpRequest, id: web::Path<String>) -> HttpResponse {
  handle_no_cache(&req, &NotionDataType::Member).await;

  match CacheStorage::get().request(
    &id.to_string(),
    &NotionDataType::Group
  ).await {
    Some(data) => {
      HttpResponse::Ok().content_type(ContentType::json()).json(data)
    },
    None => HttpResponse::NotFound().finish()
  }
}

#[get("/clubs")]
async fn get_clubs(req: HttpRequest) -> HttpResponse {
  handle_no_cache(&req, &NotionDataType::Member).await;

  HttpResponse::Ok()
    .content_type(ContentType::json())
    .json(
      CacheStorage::get().request_all(
        &NotionDataType::Club
      ).await
    )
}

#[get("/clubs/{id}")]
async fn get_club_by_id(req: HttpRequest, id: web::Path<String>) -> HttpResponse {
  handle_no_cache(&req, &NotionDataType::Member).await;

  match CacheStorage::get().request(
    &id.to_string(),
    &NotionDataType::Club
  ).await {
    Some(data) => {
      HttpResponse::Ok().content_type(ContentType::json()).json(data)
    },
    None => HttpResponse::NotFound().finish()
  }
}

#[get("/events")]
async fn get_events(req: HttpRequest) -> HttpResponse {
  handle_no_cache(&req, &NotionDataType::Member).await;

  HttpResponse::Ok()
    .content_type(ContentType::json())
    .json(
      CacheStorage::get().request_all(
        &NotionDataType::Event
      ).await
    )
}

#[get("/events/{id}")]
async fn get_event_by_id(req: HttpRequest, id: web::Path<String>) -> HttpResponse {
  handle_no_cache(&req, &NotionDataType::Member).await;

  match CacheStorage::get().request(
    &id.to_string(),
    &NotionDataType::Event
  ).await {
    Some(data) => {
      HttpResponse::Ok().content_type(ContentType::json()).json(data)
    },
    None => HttpResponse::NotFound().finish()
  }
}


#[get("/articles")]
async fn get_articles(req: HttpRequest) -> HttpResponse {
  handle_no_cache(&req, &NotionDataType::Member).await;

  HttpResponse::Ok()
    .content_type(ContentType::json())
    .json(
      CacheStorage::get().request_all(
        &NotionDataType::Article
      ).await
    )
}

#[get("/articles/{id}")]
async fn get_article_by_id(req: HttpRequest, id: web::Path<String>) -> HttpResponse {
  handle_no_cache(&req, &NotionDataType::Member).await;

  match CacheStorage::get().request(
    &id.to_string(),
    &NotionDataType::Article
  ).await {
    Some(data) => {
      HttpResponse::Ok().content_type(ContentType::json()).json(data)
    },
    None => HttpResponse::NotFound().finish()
  }
}


#[get("/sponsors")]
async fn get_sponsors(req: HttpRequest) -> HttpResponse {
  handle_no_cache(&req, &NotionDataType::Member).await;

  HttpResponse::Ok()
    .content_type(ContentType::json())
    .json(
      CacheStorage::get().request_all(
        &NotionDataType::Sponsor
      ).await
    )
}

#[get("/sponsors/{id}")]
async fn get_sponsor_by_id(req: HttpRequest, id: web::Path<String>) -> HttpResponse {
  handle_no_cache(&req, &NotionDataType::Member).await;

  match CacheStorage::get().request(
    &id.to_string(),
    &NotionDataType::Sponsor
  ).await {
    Some(data) => {
      HttpResponse::Ok().content_type(ContentType::json()).json(data)
    },
    None => HttpResponse::NotFound().finish()
  }
}
