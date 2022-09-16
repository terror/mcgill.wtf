use super::*;

#[derive(Debug, Clone, Serialize)]
pub(crate) struct Payload {
  pub(crate) time: u128,
  pub(crate) courses: Vec<Course>,
}
