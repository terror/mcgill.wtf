use super::*;

pub(crate) struct Extractor;

impl Extractor {
  pub(crate) fn extract_page(page: Page) -> Result<Option<Vec<Entry>>> {
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

  pub(crate) fn extract_course(entry: Entry) -> Result<Course> {
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
    })
  }
}

#[cfg(test)]
mod tests {
  use {super::*, pretty_assertions::assert_eq};

  #[test]
  fn page() {
    let entries = Extractor::extract_page(Page {
      number: 1,
      url: "https://www.mcgill.ca/study/2022-2023/courses/search".into(),
    })
    .unwrap();

    assert!(entries.is_some());

    assert_eq!(entries.unwrap().len(), 16);
  }

  #[test]
  fn course() {
    let entry = Entry {
      level: "Undergraduate".into(),
      terms: vec!["Fall 2022".into(), "Winter 2022".into()],
      url: "https://www.mcgill.ca/study/2022-2023/courses/cmsc-000".into(),
    };

    let Course {
      title,
      subject,
      code,
      level,
      url,
      department,
      department_url,
      terms,
      instructors,
      ..
    } = Extractor::extract_course(entry.clone()).unwrap();

    assert_eq!(title, "Foundations of Mathematics (3 credits)");

    assert_eq!(subject, "CMSC");

    assert_eq!(code, "000");

    assert_eq!(level, entry.level);

    assert_eq!(url, entry.url);

    assert_eq!(terms, entry.terms);

    assert_eq!(department, "Adaptive &amp; Integrated Learning");

    assert_eq!(
      department_url,
      "https://www.mcgill.ca/study/2022-2023/faculties/continuing"
    );

    assert_eq!(terms, vec!["Fall 2022", "Winter 2022"]);

    assert_eq!(instructors, "Chouha, Paul (Fall)");
  }
}
