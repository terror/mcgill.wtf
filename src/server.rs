use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Server {
  /// Optional port to listen on
  #[clap(long)]
  port: Option<u16>,
  /// Datasource to read from
  #[clap(long)]
  datasource: PathBuf,
  /// Specifies whether or not we're serving locally
  #[clap(long)]
  local: bool,
}

impl Server {
  pub(crate) fn run(self) -> Result {
    Runtime::new()?.block_on(async {
      log::info!("Setting up redis client...");

      let client = redis::Client::spawn(SpawnOptions {
        local: self.local,
        port: 7501,
      })?;

      log::info!("Initializing redis full-text search");

      let mut command = redis::cmd("FT.CREATE");

      for argument in "
        courses ON JSON PREFIX 1 course:
        SCHEMA
        $.title AS title TEXT WEIGHT 2
        $.description AS description TEXT
      "
      .trim()
      .split(' ')
      .filter(|arg| !arg.is_empty())
      .map(|arg| arg.trim())
      .collect::<Vec<&str>>()
      {
        command = command.arg(argument).to_owned();
      }

      command.query(&mut client.get_connection()?)?;

      log::info!("Populating redis...");

      let mut pipeline = redis::Pipeline::new();

      let mut courses = BTreeMap::new();

      serde_json::from_str::<Vec<Course>>(&fs::read_to_string(
        self.datasource,
      )?)?
      .iter()
      .try_for_each(|course| -> Result {
        log::info!("{:?}", course);

        courses.insert(format!("course:{}", course.code), course.clone());

        pipeline
          .cmd("JSON.SET")
          .arg(format!("course:{}", course.code))
          .arg("$")
          .arg(serde_json::to_string(&course)?)
          .query(&mut client.get_connection()?)?;

        Ok(())
      })?;

      let addr = SocketAddr::from(([127, 0, 0, 1], self.port.unwrap_or(7500)));

      log::info!("Listening on port {}...", addr.port());

      let search = Search { client, courses };

      axum_server::Server::bind(addr)
        .handle(Handle::new())
        .serve(
          Router::new()
            .route("/", get(|| async { "Hello, world!" }))
            .route(
              "/search/:q",
              get(|params| async move { search.search(params).await }),
            )
            .layer(
              CorsLayer::new()
                .allow_methods([Method::GET])
                .allow_origin(Any),
            )
            .into_make_service(),
        )
        .await?;

      Ok(())
    })
  }
}
