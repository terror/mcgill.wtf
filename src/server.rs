use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Server {
  #[clap(long)]
  port: Option<u16>,
}

impl Server {
  pub(crate) fn run(self, _options: Options) -> Result {
    Runtime::new()?.block_on(async {
      let addr = SocketAddr::from(([127, 0, 0, 1], self.port.unwrap_or(7500)));

      log::info!("Listening on port {}...", addr.port());

      axum_server::Server::bind(addr)
        .handle(Handle::new())
        .serve(
          Router::new()
            .route("/", get(|| async { "Hello, world!" }))
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
