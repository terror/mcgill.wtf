use {
  crate::{
    arguments::Arguments,
    extractor::Extractor,
    model::{Course, Entry},
    options::Options,
    select::Select,
    server::Server,
    subcommand::Subcommand,
  },
  anyhow::anyhow,
  axum::{routing::get, Router},
  axum_server::Handle,
  clap::Parser,
  http::Method,
  scraper::{ElementRef, Html, Selector},
  serde::Serialize,
  std::{fs, net::SocketAddr, path::PathBuf, process},
  tokio::runtime::Runtime,
  tower_http::cors::{Any, CorsLayer},
};

mod arguments;
mod extractor;
mod model;
mod options;
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
