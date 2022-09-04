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

    let mut entries = Vec::new();

    while let Some(page_entries) = self.page(self.page.get())? {
      entries.extend(page_entries);
      self.page.set(self.page.get() + 1);
    }

    Ok(
      entries
        .iter()
        .map(|entry| self.course(entry.clone()).unwrap())
        .collect::<Vec<Course>>(),
    )
  }

  fn page(&self, page: usize) -> Result<Option<Vec<Entry>>> {
    log::info!("Fetching html on page: {page}...");

    let body = Html::parse_fragment(
      &reqwest::blocking::get(format!(
        "{}?page={}",
        self.base,
        self.page.get()
      ))?
      .text()?,
    );

    let content = body
      .select(&Selector::parse("div[class='view-content']").unwrap())
      .next();

    if content.is_none() {
      log::info!("Did not find any content on page {}", self.page.get());
      return Ok(None);
    }

    log::info!("Scraping found content on page: {}...", self.page.get());

    let entries = content
      .unwrap()
      .select(&Selector::parse("div[class~='views-row']").unwrap())
      .map(|entry| Entry {
        url: entry
          .select_single("div[class~='views-field-field-course-title-long']")
          .unwrap()
          .select_single("a")
          .unwrap()
          .value()
          .attr("href")
          .unwrap()
          .to_string(),
        level: entry
          .select_single("span[class~='views-field-level']")
          .unwrap()
          .select_single("span[class='field-content']")
          .unwrap()
          .inner_html(),
        terms: entry
          .select_single("span[class~='views-field-terms']")
          .unwrap()
          .select_single("span[class='field-content']")
          .unwrap()
          .inner_html()
          .split(", ")
          .map(|s| s.to_owned())
          .collect::<Vec<String>>(),
      })
      .filter(|entry| !entry.terms.contains(&String::from("Not Offered")))
      .collect::<Vec<Entry>>();

    log::info!("Scraped entries on page {}: {:?}", self.page.get(), entries);

    Ok(Some(entries))
  }

  fn course(&self, entry: Entry) -> Result<Course> {
    let body = Html::parse_fragment(
      &reqwest::blocking::get(format!("https://www.mcgill.ca{}", entry.url))?
        .text()?,
    );

    let full_title = body
      .select(&Selector::parse("h1[id='page-title']").unwrap())
      .next()
      .unwrap()
      .inner_html()
      .trim()
      .to_owned();

    let title = full_title
      .split(' ')
      .skip(2)
      .collect::<Vec<&str>>()
      .join(" ");

    let code = full_title
      .split(' ')
      .take(2)
      .collect::<Vec<&str>>()
      .join(" ");

    let content = body
      .select(
        &Selector::parse("div[class='node node-catalog clearfix']").unwrap(),
      )
      .next()
      .unwrap();

    let description = content
      .select_single("div[class='content']")?
      .select_single("p")?
      .inner_html()
      .trim()
      .split(':')
      .skip(1)
      .collect::<Vec<&str>>()
      .join(" ")
      .trim()
      .to_owned();

    let department = content
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
      .to_owned();

    let department_url = format!(
      "https://www.mcgill.ca{}",
      content
        .select_single("div[class='meta']")?
        .select_single("p")?
        .select_single("a")?
        .value()
        .attr("href")
        .unwrap()
        .to_string()
    );

    let instructors = content
      .select_single("p[class='catalog-instructors']")?
      .inner_html()
      .trim()
      .split(' ')
      .skip(1)
      .collect::<Vec<&str>>()
      .join(" ")
      .trim()
      .to_owned();

    let course = Course {
      title,
      code,
      level: entry.level,
      department,
      department_url,
      description,
      terms: entry.terms,
      instructors,
    };

    log::info!("Parsed course: {:?}", course);

    Ok(course)
  }
}
