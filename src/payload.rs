use super::*;

#[derive(Serialize)]
pub(crate) struct Payload {
  pub(crate) time: u128,
  pub(crate) courses: Vec<Course>,
}
