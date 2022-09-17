use super::*;

pub(crate) struct Extractor;

impl Extractor {
  pub(crate) fn extract_page(page: Page) -> Result<Option<Vec<Entry>>> {
    log::info!("Parsing html on page: {}...", page.number);

    let html = Html::parse_fragment(&page.content);

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
            content: None,
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
    let html = Html::parse_fragment(&entry.content.ok_or_else(|| {
      anyhow!("Entry must have content in order to be parsed")
    })?);

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
  use {super::*, indoc::indoc};

  const PAGE: &str = indoc! {"
    <div class=\"view-content\">
      <div class=\"views-row views-row-1 views-row-odd views-row-first\">
        <div class=\"views-field views-field-field-course-title-long\">
          <h4 class=\"field-content\">
            <a href=\"/study/2022-2023/courses/cmsc-000\"
              >CMSC 000 Foundations of Mathematics (3 credits)</a
            >
          </h4>
        </div>
        <span class=\"views-field views-field-field-faculty-code\">
          <span class=\"field-content\">School of Continuing Studies</span>
        </span>
        |
        <span class=\"views-field views-field-field-dept-code\">
          <span class=\"field-content\">Adaptive &amp; Integrated Learning</span>
        </span>
        |
        <span class=\"views-field views-field-level\">
          <span class=\"field-content\">Undergraduate</span>
        </span>
        |
        <span class=\"views-field views-field-terms\">
          <span class=\"field-content\">Fall 2022, Winter 2023</span>
        </span>
      </div>
      <div class=\"views-row views-row-2 views-row-even\">
        <div class=\"views-field views-field-field-course-title-long\">
          <h4 class=\"field-content\">
            <a href=\"/study/2022-2023/courses/fmt4-001\"
              >FMT4 001 Fall Stage (152-VSA-MC) (1.33 credits)</a
            >
          </h4>
        </div>
        <span class=\"views-field views-field-field-faculty-code\">
          <span class=\"field-content\"
            >Agricultural &amp; Environmental Sciences</span
          >
        </span>
        |
        <span class=\"views-field views-field-field-dept-code\">
          <span class=\"field-content\">Farm Mgmt &amp; Technology Program</span>
        </span>
        |
        <span class=\"views-field views-field-level\">
          <span class=\"field-content\">Undergraduate</span>
        </span>
        |
        <span class=\"views-field views-field-terms\">
          <span class=\"field-content\">Fall 2022</span>
        </span>
      </div>
      <div class=\"views-row views-row-3 views-row-odd not-offered\">
        <div class=\"views-field views-field-field-course-title-long\">
          <h4 class=\"field-content\">
            <a href=\"/study/2022-2023/courses/fmtp-001\"
              >FMTP 001 Farm Practice 1 (152-001-MC) (1.33 credits)</a
            >
          </h4>
        </div>
        <span class=\"views-field views-field-field-faculty-code\">
          <span class=\"field-content\"
            >Agricultural &amp; Environmental Sciences</span
          >
        </span>
        |
        <span class=\"views-field views-field-field-dept-code\">
          <span class=\"field-content\">Farm Mgmt &amp; Technology Program</span>
        </span>
        |
        <span class=\"views-field views-field-level\">
          <span class=\"field-content\">Undergraduate</span>
        </span>
        |
        <span class=\"views-field views-field-terms\">
          <span class=\"field-content\">Not Offered</span>
        </span>
      </div>
    </div>
  "};

  const ENTRY: &str = indoc! {"
    <div id=\"container\" class=\"clearfix\">
      <div class=\"breadcrumb\">
        <a href=\"https://www.mcgill.ca\">McGill.ca</a> /
        <a href=\"/study/2022-2023/\">Overview</a> /
        <a href=\"/study/2022-2023/courses/search\" title=\"Search all courses\"
          >All Courses</a
        >
      </div>
      <div id=\"inner-container\">
        <div id=\"tabs\"></div>
        <div id=\"top-page\"></div>
        <h1 id=\"page-title\" class=\" \">
          CMSC 000 Foundations of Mathematics (3 credits)
        </h1>
        <div id=\"main-column\">
          <div id=\"top-content\"></div>
          <div id=\"content\">
            <div id=\"content-inner\">
              <div class=\"region region-content\">
                <div
                  id=\"block-system-main\"
                  class=\"block block-system region-content\"
                >
                  <div class=\"block-inner\">
                    <div class=\"content\">
                      <div id=\"node-12638\" class=\"node node-catalog clearfix\">
                        <div class=\"meta\">
                          <p>
                            Offered by: Adaptive &amp; Integrated Learning (<a
                              href=\"/study/2022-2023/faculties/continuing\"
                              >School of Continuing Studies</a
                            >)
                          </p>
                        </div>
                        <div class=\"content\">
                          <h3>Overview</h3>
                          <p>
                            Management Science (CCE) : First-degree equations and
                            applied word problems, polynomials, factoring,
                            fractions, exponents, roots and radicals, inequalities,
                            quadratic equations and functions, composite and inverse
                            functions, arithmetic and geometric sequences and
                            series.
                          </p>
                          <p class=\"catalog-terms\">Terms: Fall 2022, Winter 2023</p>
                          <p class=\"catalog-instructors\">
                            Instructors: Chouha, Paul (Fall)
                          </p>
                          <ul class=\"catalog-notes\">
                            <li><p>The passing grade for this course is B-</p></li>
                          </ul>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
          <div id=\"bottom-content\"></div>
        </div>
        <div id=\"sidebar-column\">
          <h2 class=\"offscreen\">Related Content</h2>
          <div class=\"region region-sidebar-second\">
            <div
              id=\"block-views-catalog-program-block-1\"
              class=\"block block-views region-sidebar_second\"
            >
              <div class=\"block-inner\">
                <div class=\"content\">
                  <div
                    class=\"view view-catalog-program view-id-catalog_program view-display-id-block_1 view-dom-id-744adde821df5bfbf03cd12177d3f6c7\"
                  >
                    <div class=\"view-header\">
                      <i
                        >This course may be used as a required or complementary
                        course in the following programs:</i
                      ><br />
                    </div>
                    <div class=\"view-content\">
                      <div
                        class=\"views-row views-row-1 views-row-odd views-row-first\"
                      >
                        <div class=\"views-field views-field-field-calendar-title\">
                          <div class=\"field-content\">
                            <a
                              href=\"https://www.mcgill.ca/study/2022-2023/faculties/continuing/undergraduate/programs/certificate-cert-indigenous-business-management\"
                              >Certificate (Cert.) Indigenous Business Management</a
                            >
                          </div>
                        </div>
                      </div>
                      <div class=\"views-row views-row-2 views-row-even\">
                        <div class=\"views-field views-field-field-calendar-title\">
                          <div class=\"field-content\">
                            <a
                              href=\"https://www.mcgill.ca/study/2022-2023/faculties/continuing/undergraduate/programs/certificate-cert-stem-foundations-science-technology-engineering-math\"
                              >Certificate (Cert.) STEM Foundations (Science,
                              Technology, Engineering &amp; Math)</a
                            >
                          </div>
                        </div>
                      </div>
                      <div
                        class=\"views-row views-row-3 views-row-odd views-row-last\"
                      >
                        <div class=\"views-field views-field-field-calendar-title\">
                          <div class=\"field-content\">
                            <a
                              href=\"https://www.mcgill.ca/study/2022-2023/faculties/continuing/undergraduate/programs/certificate-cert-supply-chain-management-and-logistics\"
                              >Certificate (Cert.) Supply Chain Management and
                              Logistics</a
                            >
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  "};

  #[test]
  fn page() {
    let page = Page {
      number: 1,
      content: PAGE.to_owned(),
    };

    let entries = Extractor::extract_page(page).unwrap();

    assert!(entries.is_some());
  }

  #[test]
  fn course() {
    let entry = Entry {
      content: Some(ENTRY.to_owned()),
      level: String::from("Undergraduate"),
      terms: vec![String::from("Fall 2022"), String::from("Winter 2022")],
      url: String::new(),
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
      description,
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

    assert_eq!(
      description,
      "First-degree equations and\n                        applied word problems, polynomials, factoring,\n                        fractions, exponents, roots and radicals, inequalities,\n                        quadratic equations and functions, composite and inverse\n                        functions, arithmetic and geometric sequences and\n                        series."
    );

    assert_eq!(instructors, "Chouha, Paul (Fall)");
  }
}
