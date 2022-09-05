use super::*;

const BASE_URL: &str = "https://www.mcgill.ca";

#[derive(Debug, Parser)]
pub(crate) struct Extractor {
  // Starting page at which to start downloading courses
  #[clap(long)]
  starting_page: Option<usize>,
  /// Optional file path in which data is written to
  #[clap(long)]
  datasource: Option<PathBuf>,
}

impl Extractor {
  pub(crate) fn run(&self) -> Result {
    log::info!("Running scraper...");

    let mut entries = Vec::new();

    let mut page = self.starting_page.unwrap_or(0);

    while let Some(page_entries) = self.page(page)? {
      entries.extend(page_entries);
      page += 1;
    }

    fs::write(
      self
        .datasource
        .clone()
        .unwrap_or_else(|| PathBuf::from("data.json")),
      serde_json::to_string(
        &entries
          .iter()
          .map(|entry| self.course(entry.clone()).unwrap())
          .collect::<Vec<Course>>(),
      )?,
    )
    .map_err(anyhow::Error::from)
  }

  fn page(&self, page: usize) -> Result<Option<Vec<Entry>>> {
    log::info!("Fetching html on page: {page}...");

    let html = Html::parse_fragment(
      &reqwest::blocking::get(format!(
        "{}/study/2022-2023/courses/search?page={}",
        BASE_URL, page
      ))?
      .text()?,
    );

    if let Some(content) = html
      .root_element()
      .select_optional("div[class='view-content']")?
    {
      log::info!("Scraping found content on page: {page}...");

      let results = content
        .select_many("div[class~='views-row']")?
        .iter()
        .map(|entry| -> Result<Entry> {
          Ok(Entry {
            url: format!(
              "{}{}",
              BASE_URL,
              entry
                .select_single(
                  "div[class~='views-field-field-course-title-long']",
                )?
                .select_single("a")?
                .value()
                .attr("href")
                .ok_or_else(|| anyhow!("Failed to get attribute"))?
            ),
            level: entry
              .select_single("span[class~='views-field-level']")?
              .select_single("span[class='field-content']")?
              .inner_html(),
            terms: entry
              .select_single("span[class~='views-field-terms']")?
              .select_single("span[class='field-content']")?
              .inner_html()
              .split(", ")
              .map(|s| s.to_owned())
              .collect::<Vec<String>>(),
          })
        })
        .collect::<Result<Vec<Entry>, _>>();

      let entries = results?
        .into_iter()
        .filter(|entry| !entry.terms.contains(&String::from("Not Offered")))
        .collect::<Vec<Entry>>();

      log::info!("Scraped entries on page {}: {:?}", page, entries);

      return Ok(Some(entries));
    }

    log::info!("Did not find any content on page {}", page);

    Ok(None)
  }

  fn course(&self, entry: Entry) -> Result<Course> {
    let html =
      Html::parse_fragment(&reqwest::blocking::get(&entry.url)?.text()?);

    let full_title = html
      .root_element()
      .select_single("h1[id='page-title']")?
      .inner_html()
      .trim()
      .to_owned();

    let content = html
      .root_element()
      .select_single("div[class='node node-catalog clearfix']")?;

    let course = Course {
      title: full_title
        .split(' ')
        .skip(2)
        .collect::<Vec<&str>>()
        .join(" "),
      code: full_title
        .split(' ')
        .take(2)
        .collect::<Vec<&str>>()
        .join(" "),
      level: entry.level,
      url: entry.url,
      department: content
        .select_single("div[class='meta']")?
        .select_single("p")?
        .inner_html()
        .split('(')
        .take(1)
        .collect::<Vec<&str>>()
        .join(" ")
        .split(':')
        .skip(1)
        .collect::<Vec<&str>>()
        .join(" ")
        .trim()
        .to_owned(),
      department_url: format!(
        "{}{}",
        BASE_URL,
        content
          .select_single("div[class='meta']")?
          .select_single("p")?
          .select_single("a")?
          .value()
          .attr("href")
          .ok_or_else(|| anyhow!("Failed to get attribute"))?
      ),
      description: content
        .select_single("div[class='content']")?
        .select_single("p")?
        .inner_html()
        .trim()
        .split(':')
        .skip(1)
        .collect::<Vec<&str>>()
        .join(" ")
        .trim()
        .to_owned(),
      terms: entry.terms,
      instructors: content
        .select_single("p[class='catalog-instructors']")?
        .inner_html()
        .trim()
        .split(' ')
        .skip(1)
        .collect::<Vec<&str>>()
        .join(" ")
        .trim()
        .to_owned(),
    };

    log::info!("Parsed course: {:?}", course);

    Ok(course)
  }
}
