use {
  crate::{
    arguments::Arguments,
    model::{Course, Entry},
    options::Options,
    scraper::Scraper,
    subcommand::Subcommand,
  },
  anyhow::anyhow,
  clap::Parser,
  serde::Serialize,
  std::{cell::Cell, fs, path::PathBuf, process},
  web_scraper::{Html, Selector},
};

mod arguments;
mod model;
mod options;
mod scraper;
mod subcommand;

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

fn main() {
  env_logger::init();

  if let Err(error) = Arguments::parse().run() {
    eprintln!("error: {error}");
    process::exit(1);
  }
}
