use std::{
  sync::{OnceLock, Arc},
  env,
  io::Read,
  time::Duration
};

use flate2::read::GzDecoder;
use hyper::{
  Client,
  client::HttpConnector,
  Body,
  Request,
  http::request::Builder,
  header,
  Response,
  body
};
use hyper_rustls::{
  HttpsConnector as rustls_HttpsConnector,
  HttpsConnectorBuilder
};
use serde_json::Value;
use anyhow::{Result, anyhow};
use tokio::time::sleep;
use tracing::log::debug;

use super::{types::{
  Member,
  Group,
  Club,
  NotionDataType,
  NotionData,
  Event,
  Article,
  Sponsor
}, cache::CacheStorage};


type HttpsConnector = rustls_HttpsConnector<HttpConnector>;


static HTTP_CLIENT: OnceLock<Client<HttpsConnector, Body>> = OnceLock::new();
static INTEGRATION_SECRET: OnceLock<Arc<str>> = OnceLock::new();
static NOTION_VERSION: &str = "2022-06-28";


fn get_http_client() -> Client<HttpsConnector, Body> {
  HTTP_CLIENT.get_or_init(
    || {
      Client::builder().build(
        HttpsConnectorBuilder::new()
        .with_native_roots()
        .https_only()
        .enable_http1()
        .build()
      )
    }
  ).clone()
}

fn build_request(
  url: &str
) -> Result<Request<Body>> {
  let builder: Builder = Request::post(url)
    .header(
      header::USER_AGENT,
      "Rust@2021/hyper@0.14.26/hyper-rustls@0.24.0"
    )
    .header(
      header::AUTHORIZATION,
      format!(
        "Bearer {token}",
        token = INTEGRATION_SECRET.get_or_init(
          || {
            env::var("INTEGRATION_SECRET")
              .expect("INTEGRATION_SECRET is not set.")
              .into()
          }
        )
      )
    )
    .header(
      "Notion-Version",
      NOTION_VERSION
    )
    .header(
      header::ACCEPT_ENCODING,
      "gzip"
    )
    .header(
      header::ACCEPT,
      "*/*"
    )
    .header(
      header::CONNECTION,
      "keep-alive"
    )
    .header(
      header::CONTENT_TYPE,
      "application/json"
    )
    .header(
      header::CONTENT_LENGTH,
      0
    );

  let request: Request<Body> = builder.body(Body::empty())?;

  debug!("Updated headers: {:?}", request.headers());

  Ok(request)
}

async fn request(url: &str) -> Result<Value> {
  debug!("Sending request: {:?}", url);

  let response: Response<Body> = get_http_client().request(
    build_request(url)?
  ).await?;

  let mut body: String = String::new();

  GzDecoder::new(
    &*body::to_bytes(response).await?
  ).read_to_string(&mut body)?;

  debug!("Decoded body: {:?}", body);
  
  Ok(serde_json::from_str(&body)?)
}

pub async fn update_all() {
  for data_type in NotionDataType::iterator() {
    CacheStorage::get().update(
      &data_type,
      fetch_data(&data_type).await.unwrap()
    ).await;
    sleep(Duration::from_millis(500)).await;
  }
}

pub async fn fetch_data(
  data_type: &NotionDataType,
) -> Result<Vec<NotionData>> {
  let url: Arc<str> = format!(
    "https://api.notion.com/v1/databases/{database_id}/query",
    database_id = data_type.get_databse_id()
  ).into();

  let response: Value = request(&url).await?;

  let mut data: Vec<NotionData> = Vec::new();

  for json_data in response["results"].as_array().ok_or(
    anyhow!("Parse JSON failed.")
  )?.into_iter() {
    data.push(
      match data_type {
        NotionDataType::Member => NotionData::Member(
          Member::from_json(json_data).await.unwrap_or(Member::default())
        ),
        NotionDataType::Group => NotionData::Group(
          Group::from_json(json_data).await.unwrap_or(Group::default())
        ),
        NotionDataType::Club => NotionData::Club(
          Club::from_json(json_data).await.unwrap_or(Club::default())
        ),
        NotionDataType::Event => NotionData::Event(
          Event::from_json(json_data).await.unwrap_or(Event::default())
        ),
        NotionDataType::Article => NotionData::Article(
          Article::from_json(json_data).await.unwrap_or(Article::default())
        ),
        NotionDataType::Sponsor => NotionData::Sponsor(
          Sponsor::from_json(json_data).await.unwrap_or(Sponsor::default())
        ),
      }
    )
  }

  Ok(data)
}
