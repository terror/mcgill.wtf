use {
  crate::{
    arguments::Arguments, cmd_ext::CmdExt, course::Course, entry::Entry,
    extractor::Extractor, index::Index, loader::Loader, page::Page,
    params::Params, payload::Payload, select::Select, server::Server,
    subcommand::Subcommand,
  },
  anyhow::anyhow,
  axum::{
    extract::{Extension, Query},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
  },
  axum_server::Handle,
  clap::Parser,
  http::Method,
  redis::Cmd,
  scraper::{ElementRef, Html, Selector},
  serde::{Deserialize, Serialize},
  std::{
    cell::RefCell,
    collections::BTreeMap,
    fs,
    net::SocketAddr,
    path::PathBuf,
    process,
    sync::{Arc, Mutex},
    thread,
    time::Instant,
  },
  tokio::runtime::Runtime,
  tower_http::cors::{Any, CorsLayer},
  uuid::Uuid,
};

const BASE_URL: &str = "https://www.mcgill.ca";

mod arguments;
mod cmd_ext;
mod course;
mod entry;
mod extractor;
mod index;
mod loader;
mod page;
mod params;
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
