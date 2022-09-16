use super::*;

#[derive(Debug, Clone)]
pub(crate) struct Entry {
  pub(crate) url: String,
  pub(crate) level: String,
  pub(crate) terms: Vec<String>,
}

impl Entry {
  pub(crate) fn content(&self) -> Result<String> {
    Ok(reqwest::blocking::get(&self.url)?.text()?)
  }
}
