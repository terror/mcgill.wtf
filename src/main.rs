use {
  crate::{
    arguments::Arguments, command::Command, course::Course,
    extractor::Extractor, index::Index, payload::Payload, select::Select,
    server::Server, subcommand::Subcommand,
  },
  anyhow::anyhow,
  axum::{extract::Query, response::IntoResponse, routing::get, Json, Router},
  axum_server::Handle,
  clap::Parser,
  http::Method,
  redis::Cmd,
  scraper::{ElementRef, Html, Selector},
  serde::{Deserialize, Serialize},
  std::{
    collections::BTreeMap, fs, net::SocketAddr, path::PathBuf, process,
    time::Instant,
  },
  tokio::runtime::Runtime,
  tower_http::cors::{Any, CorsLayer},
  uuid::Uuid,
};

mod arguments;
mod command;
mod course;
mod extractor;
mod index;
mod payload;
mod select;
mod server;
mod subcommand;

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

fn main() {
  env_logger::init();

  if let Err(error) = Arguments::parse().run() {
    eprintln!("error: {error}");
    process::exit(1);
  }
}
