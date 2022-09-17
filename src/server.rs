use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Server {
  #[clap(long, help = "Datasource to read from.")]
  datasource: PathBuf,
  #[clap(long, help = "Optional port to listen on.")]
  port: Option<u16>,
}

impl Server {
  pub(crate) fn run(self) -> Result {
    Runtime::new()?.block_on(async {
      log::info!("Initializing index...");

      let index = Arc::new(Index::open()?);

      let clone = index.clone();

      thread::spawn(move || {
        if let Err(error) = clone.index(self.datasource) {
          log::error!("error: {error}");
        }
      });

      let addr = SocketAddr::from(([127, 0, 0, 1], self.port.unwrap_or(7500)));

      log::info!("Listening on port {}...", addr.port());

      axum_server::Server::bind(addr)
        .handle(Handle::new())
        .serve(
          Router::new()
            .route("/", get(|| async { "Hello, world!" }))
            .route("/search", get(Self::search))
            .layer(Extension(index))
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

  async fn search(
    Query(params): Query<Params>,
    index: Extension<Arc<Index>>,
  ) -> impl IntoResponse {
    match index.search(&params.query) {
      Ok(payload) => (StatusCode::OK, Json(Some(payload))),
      Err(error) => {
        eprintln!("Error serving request for query {}: {error}", params.query);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
      }
    }
  }
}
