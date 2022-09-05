use {
  crate::{
    arguments::Arguments,
    extractor::Extractor,
    model::{Course, Entry},
    search::Search,
    select::Select,
    server::Server,
    spawn::{Spawn, SpawnOptions},
    subcommand::Subcommand,
  },
  anyhow::anyhow,
  axum::{extract::Path, response::IntoResponse, routing::get, Json, Router},
  axum_server::Handle,
  clap::Parser,
  http::Method,
  scraper::{ElementRef, Html, Selector},
  serde::{Deserialize, Serialize},
  std::{
    collections::BTreeMap,
    fs,
    net::SocketAddr,
    path::PathBuf,
    process::{self, Command},
  },
  tokio::runtime::Runtime,
  tower_http::cors::{Any, CorsLayer},
};

mod arguments;
mod extractor;
mod model;
mod search;
mod select;
mod server;
mod spawn;
mod subcommand;

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

fn main() {
  env_logger::init();

  if let Err(error) = Arguments::parse().run() {
    eprintln!("error: {error}");
    process::exit(1);
  }
}
