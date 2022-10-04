use super::*;

#[derive(Debug, Clone, Serialize)]
pub(crate) struct Payload {
  pub(crate) time: f64,
  pub(crate) courses: Vec<Course>,
}
