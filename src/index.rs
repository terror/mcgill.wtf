use super::*;

#[derive(Debug, Clone)]
pub(crate) struct Index {
  client: redis::Client,
  courses: BTreeMap<String, Course>,
}

#[derive(Deserialize)]
pub(crate) struct Params {
  query: String,
}

impl Index {
  pub(crate) fn initialize(datasource: PathBuf) -> Result<Self> {
    log::info!("Setting up redis client...");

    let client = redis::Client::open("redis://localhost:7501")?;

    log::info!("Initializing redis full-text search...");

    let mut command = redis::cmd("FT.CREATE");

    command.build(
      "
        courses ON JSON PREFIX 1 course: NOOFFSETS
        SCHEMA
        $.title AS title TEXT WEIGHT 2
        $.description AS description TEXT
        $.subject AS subject TEXT NOSTEM WEIGHT 2
        $.code AS code TEXT NOSTEM WEIGHT 2
        $.level AS level TAG
      ",
    );

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
    Query(params): Query<Params>,
  ) -> impl IntoResponse + 'static {
    let query = params.query;

    log::info!("Received query: {query}");

    let mut command = redis::cmd("FT.SEARCH");

    command.build(&format!("courses '{query}' RETURN 0 LIMIT 0 100"));

    let now = Instant::now();

    let identifiers = command
      .query::<Vec<String>>(&mut self.client.get_connection().unwrap())
      .unwrap();

    let elapsed = now.elapsed().as_millis();

    Json(Payload {
      time: elapsed,
      courses: identifiers
        .iter()
        .map(|identifier| self.courses.get(identifier).unwrap())
        .cloned()
        .collect::<Vec<Course>>(),
    })
  }
}
