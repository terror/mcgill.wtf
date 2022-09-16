use super::*;

#[derive(Debug, Parser)]
pub(crate) enum Subcommand {
  /// Extract and download course data
  Download(Loader),
  /// Spawn the backend server
  Serve(Server),
}

impl Subcommand {
  pub(crate) fn run(self) -> Result {
    match self {
      Self::Download(loader) => loader.run(),
      Self::Serve(server) => server.run(),
    }
  }
}
