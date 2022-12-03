use super::*;

#[derive(Debug, Clone)]
pub(crate) struct Index {
  client: redis::Client,
  courses: Arc<Mutex<BTreeMap<String, Course>>>,
}

impl Index {
  pub(crate) fn open() -> Result<Self> {
    Ok(Self {
      client: redis::Client::open("redis://localhost:7501")?,
      courses: Arc::new(Mutex::new(BTreeMap::new())),
    })
  }

  pub(crate) fn index(&self, datasource: PathBuf) -> Result {
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

    command.query(&mut self.client.get_connection()?)?;

    log::info!("Populating redis...");

    let mut pipeline = redis::Pipeline::new();

    let datasource = datasource.display().to_string();

    serde_json::from_str::<Vec<Course>>(
      &match datasource.starts_with("http") {
        true => blocking::get(datasource)?.text()?,
        false => fs::read_to_string(datasource)?,
      },
    )?
    .iter()
    .try_for_each(|course| -> Result {
      log::info!("Writing course {}{}", course.subject, course.code);

      self
        .courses
        .lock()
        .unwrap()
        .insert(format!("course:{}", course.id), course.clone());

      pipeline
        .cmd("JSON.SET")
        .arg(format!("course:{}", course.id))
        .arg("$")
        .arg(serde_json::to_string(&course)?)
        .query(&mut self.client.get_connection()?)?;

      Ok(())
    })
  }

  pub(crate) fn search(&self, query: &str) -> Result<Payload> {
    log::info!("Received query: {query}");

    let mut command = redis::cmd("FT.SEARCH");

    command.build(&format!(
      "courses '{}' RETURN 0 LIMIT 0 100",
      query.split(' ').collect::<Vec<&str>>().join(r"\")
    ));

    let now = Instant::now();

    let identifiers =
      command.query::<Vec<String>>(&mut self.client.get_connection()?)?;

    let elapsed =
      f64::trunc((now.elapsed().as_secs_f64() * 1000.0) * 100.0) / 100.0;

    Ok(Payload {
      time: elapsed,
      courses: identifiers
        .iter()
        .map(|identifier| {
          self
            .courses
            .lock()
            .unwrap()
            .get(identifier)
            .cloned()
            .ok_or_else(|| {
              anyhow!("Failed to find course with identifier {identifier}")
            })
        })
        .collect::<Result<Vec<Course>, _>>()?,
    })
  }
}
