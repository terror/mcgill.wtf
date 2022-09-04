use super::*;

#[derive(Debug)]
pub(crate) struct Scraper<'a> {
  base: &'a str,
  page: Cell<usize>,
}

impl<'a> Scraper<'a> {
  pub(crate) fn new() -> Scraper<'a> {
    Self {
      base: "https://www.mcgill.ca/study/2022-2023/courses/search",
      page: Cell::new(0),
    }
  }

  pub(crate) fn run(&self) -> Result<Vec<Course>> {
    log::info!("Running scraper...");

    let mut body = Html::parse_fragment(
      &reqwest::blocking::get(format!(
        "{}?page={}",
        self.base,
        self.page.get()
      ))?
      .text()?,
    );

    let mut entries = Vec::new();

    // while let Some(content) = body
    //   .select(&Selector::parse("div[class='view-content']").unwrap())
    //   .next()
    // {

    let content = body
      .select(&Selector::parse("div[class='view-content']").unwrap())
      .next()
      .unwrap();

    log::info!("Parsing entries on page: {}", self.page.get());

    let page_entries = content
      .select(&Selector::parse("div[class~='views-row']").unwrap())
      .map(|entry| Entry {
        url: entry
          .select(
            &Selector::parse(
              "div[class~='views-field-field-course-title-long']",
            )
            .unwrap(),
          )
          .next()
          .unwrap()
          .select(&Selector::parse("a").unwrap())
          .next()
          .unwrap()
          .value()
          .attr("href")
          .unwrap()
          .to_string(),
        level: entry
          .select(&Selector::parse("span[class~='views-field-level']").unwrap())
          .next()
          .unwrap()
          .select(&Selector::parse("span[class='field-content']").unwrap())
          .next()
          .unwrap()
          .inner_html(),
        terms: entry
          .select(&Selector::parse("span[class~='views-field-terms']").unwrap())
          .next()
          .unwrap()
          .select(&Selector::parse("span[class='field-content']").unwrap())
          .next()
          .unwrap()
          .inner_html(),
      })
      .filter(|entry| entry.terms != "Not Offered")
      .collect::<Vec<Entry>>();

    log::info!(
      "Fetched entries on page {}: {:?}",
      self.page.get(),
      page_entries
    );

    entries.extend(page_entries);

    self.page.set(self.page.get() + 1);

    body = Html::parse_fragment(
      &reqwest::blocking::get(format!(
        "{}?page={}",
        self.base,
        self.page.get()
      ))?
      .text()?,
    );
    // }

    Ok(
      entries
        .iter()
        .map(|entry| self.course(entry.clone()).unwrap())
        .collect::<Vec<Course>>(),
    )
  }

  fn course(&self, entry: Entry) -> Result<Course> {
    // sleep(Duration::from_millis(100));

    let body = Html::parse_fragment(
      &reqwest::blocking::get(format!("https://www.mcgill.ca{}", entry.url))?
        .text()?,
    );

    let title = body
      .select(&Selector::parse("h1[id='page-title']").unwrap())
      .next()
      .unwrap()
      .inner_html();

    let content = body
      .select(
        &Selector::parse("div[class='node node-catalog clearfix']").unwrap(),
      )
      .next()
      .unwrap();

    let course = Course {
      title: title.trim().to_owned(),
      level: entry.level,
      terms: entry.terms,
      description: content
        .select(&Selector::parse("div[class='content']").unwrap())
        .next()
        .unwrap()
        .select(&Selector::parse("p").unwrap())
        .next()
        .unwrap()
        .inner_html()
        .trim()
        .split(" ")
        .skip(2)
        .collect::<Vec<&str>>()
        .join(" ")
        .trim()
        .to_owned(),
      department: content
        .select(&Selector::parse("div[class='meta']").unwrap())
        .next()
        .unwrap()
        .select(&Selector::parse("p").unwrap())
        .next()
        .unwrap()
        .inner_html()
        .trim()
        .to_owned(),
      instructors: content
        .select(&Selector::parse("p[class='catalog-instructors']").unwrap())
        .next()
        .unwrap()
        .inner_html()
        .trim()
        .split(" ")
        .skip(1)
        .collect::<Vec<&str>>()
        .join(" ")
        .trim()
        .to_owned(),
      ..Default::default()
    };

    log::info!("Parsed course: {:?}", course);

    Ok(course)
  }
}
