use std::{
  time::Duration,
  io::BufReader,
  fs::File,
  env
};

use actix_web::{App, HttpServer, middleware};
use api::get_member_by_id;
use notion::client::update_all;
use rustls::{
  ServerConfig,
  Certificate,
  PrivateKey,
  ConfigBuilder,
  server::WantsServerCert
};
use rustls_pemfile::{pkcs8_private_keys, certs};
use tokio::time::sleep;
use tracing::{Level, log::{debug, error}};
use dotenv::dotenv;

use crate::api::*;


mod notion;
mod api;


static WORKERS: usize = 8;
static HTTP_PORT: u16 = 443;
static MAX_CACHE_AGE: Duration = Duration::from_secs(86400);


fn load_rustls_config() -> ServerConfig {
  let config: ConfigBuilder<ServerConfig, WantsServerCert> = ServerConfig::builder()
    .with_safe_defaults()
    .with_no_client_auth();

  let cert_file: &mut BufReader<File> = &mut BufReader::new(
    File::open(
      env::var("SSL_CERT_PATH").unwrap()
    ).unwrap()
  );
  let key_file: &mut BufReader<File> = &mut BufReader::new(
    File::open(
      env::var("SSL_CERT_KEY_PATH").unwrap()
    ).unwrap()
  );

  let cert_chain: Vec<Certificate> = certs(cert_file)
    .unwrap()
    .into_iter()
    .map(Certificate)
    .collect();
  let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
    .unwrap()
    .into_iter()
    .map(PrivateKey)
    .collect();

  if keys.is_empty() {
    error!("Could not locate PKCS 8 private keys.");
    std::process::exit(1);
  }

  config.with_single_cert(cert_chain, keys.remove(0)).unwrap()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
  tracing_subscriber::fmt()
    .with_max_level(Level::INFO)
    .init();

  dotenv().ok();

  let config: ServerConfig = load_rustls_config();

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
      .wrap(middleware::Logger::default())
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
    .workers(WORKERS)
    .bind_rustls(("0.0.0.0", HTTP_PORT), config)?
    .run()
    .await
}
