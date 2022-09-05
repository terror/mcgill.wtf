use super::*;

#[derive(Debug, Clone)]
pub(crate) struct Search {
  pub(crate) client: redis::Client,
  pub(crate) courses: BTreeMap<String, Course>,
}

use std::collections::HashMap;

impl Search {
  pub(crate) async fn search(
    self,
    Path(params): Path<HashMap<String, String>>,
  ) -> impl IntoResponse + 'static {
    let query = params.get("q").unwrap();

    log::info!("Received query: {:?}", query);

    let mut command = redis::cmd("FT.SEARCH");

    for argument in format!("courses '{query}' RETURN 0 LIMIT 0 100")
      .trim()
      .split(' ')
      .filter(|arg| !arg.is_empty())
      .map(|arg| arg.trim())
      .collect::<Vec<&str>>()
    {
      command = command.arg(argument).to_owned();
    }

    Json(
      command
        .query::<Vec<String>>(&mut self.client.get_connection().unwrap())
        .unwrap()
        .iter()
        .map(|course_id| self.courses.get(course_id).unwrap())
        .cloned()
        .collect::<Vec<Course>>(),
    )
  }
}
