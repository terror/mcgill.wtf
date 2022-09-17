use super::*;

#[derive(Debug, Parser)]
pub(crate) struct Loader {
  #[clap(long, help = "Optional file path in which data is written to.")]
  datasource: Option<PathBuf>,
  #[clap(long, help = "Starting page at which to start downloading courses.")]
  starting_page: Option<usize>,
}

impl Loader {
  pub(crate) fn run(self) -> Result {
    log::info!("Running loader...");

    let mut courses = Vec::new();

    let mut page = self.starting_page.unwrap_or(0);

    while let Some(entries) = Extractor::page(Page {
      number: page,
      url: format!("{}/study/2022-2023/courses/search?page={}", BASE_URL, page),
    })? {
      courses.extend(
        entries
          .iter()
          .map(|entry| Extractor::course(entry.clone()))
          .collect::<Result<Vec<Course>, _>>()?,
      );
      page += 1;
    }

    fs::write(
      self
        .datasource
        .unwrap_or_else(|| PathBuf::from("data.json")),
      serde_json::to_string(&courses)?,
    )
    .map_err(anyhow::Error::from)
  }
}
