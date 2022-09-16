use super::*;

use std::cell::RefCell;

#[derive(Debug, Clone)]
pub(crate) struct Index {
  client: redis::Client,
  courses: Arc<Mutex<RefCell<BTreeMap<String, Course>>>>,
}

impl Index {
  pub(crate) fn open() -> Result<Self> {
    Ok(Self {
      client: redis::Client::open("redis://localhost:7501")?,
      courses: Arc::new(Mutex::new(RefCell::new(BTreeMap::new()))),
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

    serde_json::from_str::<Vec<Course>>(&fs::read_to_string(datasource)?)?
      .iter()
      .try_for_each(|course| -> Result {
        log::info!("Writing course {}{}", course.subject, course.code);

        self
          .courses
          .lock()
          .unwrap()
          .borrow_mut()
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

    command.build(&format!("courses '{query}' RETURN 0 LIMIT 0 100"));

    let now = Instant::now();

    let identifiers =
      command.query::<Vec<String>>(&mut self.client.get_connection()?)?;

    let elapsed = now.elapsed().as_millis();

    Ok(Payload {
      time: elapsed,
      courses: identifiers
        .iter()
        .map(|identifier| {
          self
            .courses
            .lock()
            .unwrap()
            .borrow_mut()
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
