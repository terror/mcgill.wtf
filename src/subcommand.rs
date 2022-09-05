use super::*;

#[derive(Debug, Parser)]
pub(crate) enum Subcommand {
  /// Extract and download course data
  Download(Extractor),
  /// Spawn the backend server
  Serve(Server),
}

impl Subcommand {
  pub(crate) fn run(self) -> Result {
    match self {
      Self::Download(extractor) => extractor.run(),
      Self::Serve(server) => server.run(),
    }
  }
}
