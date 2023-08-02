use std::{
  time::Duration,
  env,
  path::PathBuf,
  net::SocketAddr
};

use axum::{Router, routing::get};
use axum_server::tls_rustls::RustlsConfig;
use notion::client::update_all;
use tokio::time::sleep;
use tower_http::trace::{TraceLayer, self};
use tracing::log::debug;
use tracing_subscriber::{
  layer::SubscriberExt,
  util::SubscriberInitExt
};
use dotenv::dotenv;

use crate::api::*;


mod notion;
mod api;


static HTTPS_PORT: u16 = 443;
static MAX_CACHE_AGE: Duration = Duration::from_secs(86400);


#[tokio::main]
async fn main() {
  tracing_subscriber::registry()
    .with(
      tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(
          |_| {
            "scaict_website_api=info,tower_http=trace,axum::rejection=trace".into()
          }
        )
    )
    .with(tracing_subscriber::fmt::layer())
    .init();

  dotenv().ok();

  let config: RustlsConfig = RustlsConfig::from_pem_file(
    PathBuf::from(env::var("SSL_CERT_PATH").unwrap()),
    PathBuf::from(env::var("SSL_CERT_KEY_PATH").unwrap())
  )
  .await
  .unwrap();

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

  let app: Router = Router::new()
    .route("/version", get(get_version))
    .route("/members", get(get_members))
    .route("/members/:id", get(get_member_by_id))
    .route("/groups", get(get_groups))
    .route("/groups/:id", get(get_group_by_id))
    .route("/clubs", get(get_clubs))
    .route("/clubs/:id", get(get_club_by_id))
    .route("/events", get(get_events))
    .route("/events/:id", get(get_event_by_id))
    .route("/articles", get(get_articles))
    .route("/articles/:id", get(get_article_by_id))
    .route("/sponsors", get(get_sponsors))
    .route("/sponsors/:id", get(get_sponsor_by_id))
    .layer(
      TraceLayer::new_for_http()
        .on_request(trace::DefaultOnRequest::new())
        .on_response(trace::DefaultOnResponse::new())
        .on_failure(trace::DefaultOnFailure::new())
    );

  let addr: SocketAddr = SocketAddr::from(
    ([0, 0, 0, 0], HTTPS_PORT)
  );

  axum_server::bind_rustls(addr, config)
    .serve(app.into_make_service())
    .await
    .unwrap();
}
