use super::*;

pub(crate) struct Extractor;

impl Extractor {
  pub(crate) fn page(page: Page) -> Result<Option<Vec<Entry>>> {
    log::info!("Parsing html on page: {}...", page.number);

    let html = Html::parse_fragment(&page.content()?);

    if let Some(content) = html
      .root_element()
      .select_optional("div[class='view-content']")?
    {
      log::info!("Parsing found content on page {}...", page.number);

      let results = content
        .select_many("div[class~='views-row']")?
        .iter()
        .map(|entry| -> Result<Entry> {
          Ok(Entry {
            department: entry
              .select_single("span[class~='views-field-field-dept-code']")?
              .select_single("span[class='field-content']")?
              .inner_html(),
            faculty: entry
              .select_single("span[class~='views-field-field-faculty-code']")?
              .select_single("span[class='field-content']")?
              .inner_html(),
            level: entry
              .select_single("span[class~='views-field-level']")?
              .select_single("span[class='field-content']")?
              .inner_html(),
            terms: entry
              .select_single("span[class~='views-field-terms']")?
              .select_single("span[class='field-content']")?
              .inner_html()
              .split(", ")
              .map(|term| term.to_owned())
              .collect::<Vec<String>>(),
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
          })
        })
        .collect::<Result<Vec<Entry>, _>>();

      let entries = results?
        .into_iter()
        .filter(|entry| !entry.terms.contains(&String::from("Not Offered")))
        .collect::<Vec<Entry>>();

      log::info!("Scraped entries on page {}: {:?}", page.number, entries);

      return Ok(Some(entries));
    }

    log::info!("Did not find any content on page {}", page.number);

    Ok(None)
  }

  pub(crate) fn course(entry: Entry) -> Result<Course> {
    let html = Html::parse_fragment(&entry.content()?);

    let full_title = html
      .root_element()
      .select_single("h1[id='page-title']")?
      .inner_html()
      .trim()
      .to_owned();

    let full_code = full_title
      .split(' ')
      .take(2)
      .collect::<Vec<&str>>()
      .join(" ");

    let subject = full_code
      .split(' ')
      .take(1)
      .collect::<Vec<&str>>()
      .join(" ");

    let code = full_code
      .split(' ')
      .skip(1)
      .collect::<Vec<&str>>()
      .join(" ");

    let content = html
      .root_element()
      .select_single("div[class='node node-catalog clearfix']")?;

    log::info!("Parsed course {}{}", subject, code);

    Ok(Course {
      id: Uuid::new_v5(
        &Uuid::NAMESPACE_X500,
        format!("{}-{}", subject, code).as_bytes(),
      )
      .to_string(),
      title: full_title
        .split(' ')
        .skip(2)
        .collect::<Vec<&str>>()
        .join(" "),
      subject,
      code,
      level: entry.level,
      url: entry.url,
      department: entry.department,
      faculty: entry.faculty,
      faculty_url: format!(
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
    })
  }
}

#[cfg(test)]
mod tests {
  use {super::*, pretty_assertions::assert_eq};

  #[test]
  fn page() {
    let entries = Extractor::page(Page {
      number: 1,
      url: "https://www.mcgill.ca/study/2022-2023/courses/search".into(),
    })
    .unwrap();

    assert!(entries.is_some());

    let entries = entries.unwrap();

    assert_eq!(entries.len(), 16);

    let first = entries.first().unwrap();

    assert_eq!(first.department, "Student Services");

    assert_eq!(first.faculty, "School of Continuing Studies");

    assert_eq!(first.level, "Undergraduate");

    assert_eq!(first.terms, vec!["Fall 2022", "Winter 2023"]);

    assert_eq!(
      first.url,
      "https://www.mcgill.ca/study/2022-2023/courses/aaaa-100"
    );
  }

  #[test]
  fn course() {
    let entry = Entry {
      department: "Computer Science".into(),
      faculty: "Faculty of Science".into(),
      level: "Undergraduate".into(),
      terms: vec!["Fall 2022".into(), "Winter 2023".into()],
      url: "https://www.mcgill.ca/study/2022-2023/courses/comp-251".into(),
    };

    let Course {
      title,
      subject,
      code,
      level,
      url,
      department,
      faculty,
      faculty_url,
      terms,
      description,
      instructors,
      ..
    } = Extractor::course(entry.clone()).unwrap();

    assert_eq!(title, "Algorithms and Data Structures (3 credits)");

    assert_eq!(subject, "COMP");

    assert_eq!(code, "251");

    assert_eq!(level, entry.level);

    assert_eq!(url, entry.url);

    assert_eq!(department, entry.department);

    assert_eq!(faculty, entry.faculty);

    assert_eq!(
      faculty_url,
      "https://www.mcgill.ca/study/2022-2023/faculties/science"
    );

    assert_eq!(terms, entry.terms);

    assert_eq!(
      description,
      "Introduction to algorithm design and analysis. Graph algorithms, greedy algorithms, data structures, dynamic programming, maximum flows."
    );

    assert_eq!(
      instructors,
      "Waldispuhl, Jérôme; Alberini, Giulia (Fall) Becerra, David (Winter)"
    );
  }
}
