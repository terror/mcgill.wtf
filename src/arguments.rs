use super::*;

#[derive(Debug, Parser)]
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
