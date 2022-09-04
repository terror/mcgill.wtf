use super::*;

#[derive(Debug, Parser)]
#[clap(version)]
pub(crate) struct Arguments {
  #[clap(subcommand)]
  subcommand: Subcommand,
  #[clap(flatten)]
  options: Options,
}

impl Arguments {
  pub(crate) fn run(self) -> Result {
    self.subcommand.run(self.options)
  }
}
