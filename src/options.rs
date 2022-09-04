use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Options {
  /// Path to the datasource
  #[clap(long)]
  pub(crate) datasource: Option<PathBuf>,
}
