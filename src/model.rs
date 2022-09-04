use super::*;

#[derive(Debug, Clone)]
pub(crate) struct Entry {
  pub(crate) url: String,
  pub(crate) level: String,
  pub(crate) terms: String,
}

#[derive(Debug, Default, Serialize)]
pub(crate) struct Course {
  pub(crate) title: String,
  pub(crate) level: String,
  pub(crate) department: String,
  pub(crate) terms: String,
  pub(crate) description: String,
  pub(crate) instructors: String,
}
