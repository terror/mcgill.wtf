use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Options {
  #[clap(long)]
  pub(crate) datasource: Option<PathBuf>,
}
