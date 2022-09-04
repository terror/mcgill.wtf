use super::*;

#[derive(Debug, Parser)]
pub(crate) enum Subcommand {
  #[clap(help = "Extract course data")]
  Download(Extractor),
  #[clap(help = "Run the backend server")]
  Serve(Server),
}

impl Subcommand {
  pub(crate) fn run(self, options: Options) -> Result {
    match self {
      Self::Download(extractor) => fs::write(
        options
          .datasource
          .unwrap_or_else(|| PathBuf::from("data.json")),
        serde_json::to_string(&extractor.run()?)?,
      )
      .map_err(anyhow::Error::from),
      Self::Serve(server) => server.run(options),
    }
  }
}
