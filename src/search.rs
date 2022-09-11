use super::*;

#[derive(Debug, Clone)]
pub(crate) struct Search {
  client: redis::Client,
  courses: BTreeMap<String, Course>,
}

impl Search {
  pub(crate) fn initialize(datasource: PathBuf) -> Result<Self> {
    log::info!("Setting up redis client...");

    let client = redis::Client::open("redis://localhost:7501")?;

    log::info!("Initializing redis full-text search...");

    let mut command = redis::cmd("FT.CREATE");

    "
      courses ON JSON PREFIX 1 course: NOOFFSETS
      SCHEMA
      $.title AS title TEXT WEIGHT 2
      $.description AS description TEXT
      $.subject AS subject TEXT NOSTEM WEIGHT 2
      $.code AS code TEXT NOSTEM WEIGHT 2
      $.level AS level TAG
    "
    .trim()
    .split(' ')
    .filter(|argument| !argument.is_empty())
    .map(|argument| argument.trim())
    .for_each(|argument| command = command.arg(argument).to_owned());

    command.query(&mut client.get_connection()?)?;

    log::info!("Populating redis...");

    let mut pipeline = redis::Pipeline::new();

    let mut courses = BTreeMap::new();

    serde_json::from_str::<Vec<Course>>(&fs::read_to_string(datasource)?)?
      .iter()
      .try_for_each(|course| -> Result {
        log::info!("Writing course {}{}", course.subject, course.code);

        courses.insert(format!("course:{}", course.id), course.clone());

        pipeline
          .cmd("JSON.SET")
          .arg(format!("course:{}", course.id))
          .arg("$")
          .arg(serde_json::to_string(&course)?)
          .query(&mut client.get_connection()?)?;

        Ok(())
      })?;

    Ok(Self { client, courses })
  }

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
