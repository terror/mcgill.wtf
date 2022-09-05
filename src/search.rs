use super::*;

#[derive(Debug, Clone)]
pub(crate) struct Search {
  pub(crate) client: redis::Client,
  pub(crate) courses: BTreeMap<String, Course>,
}

impl Search {
  pub(crate) async fn search(self) -> impl IntoResponse {
    Json(self.courses)
  }
}
