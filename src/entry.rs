use super::*;

#[derive(Debug, Clone)]
pub(crate) struct Entry {
  pub(crate) content: Option<String>,
  pub(crate) level: String,
  pub(crate) terms: Vec<String>,
  pub(crate) url: String,
}

impl Entry {
  pub(crate) fn set_content(self, content: &str) -> Self {
    Self {
      content: Some(content.to_owned()),
      ..self
    }
  }
}
