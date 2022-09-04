use super::*;

#[derive(Debug, Parser)]
pub(crate) enum Subcommand {
  Download,
  Serve,
}

impl Subcommand {
  pub(crate) fn run(self, options: Options) -> Result {
    match self {
      Self::Download => fs::write(
        options
          .datasource
          .unwrap_or_else(|| PathBuf::from("data.json")),
        serde_json::to_string(&Scraper::new().run()?)?,
      )
      .map_err(|error| anyhow!("IO Error: {error}")),
      Self::Serve => todo!(),
    }
  }
}
