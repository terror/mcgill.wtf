use super::*;

#[derive(Debug, Clone)]
pub(crate) struct Entry {
  pub(crate) department: String,
  pub(crate) faculty: String,
  pub(crate) level: String,
  pub(crate) terms: Vec<String>,
  pub(crate) url: String,
}

impl Entry {
  pub(crate) fn content(&self) -> Result<String> {
    Ok(reqwest::blocking::get(self.url.clone())?.text()?)
  }
}
