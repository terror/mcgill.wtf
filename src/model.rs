use super::*;

#[derive(Debug, Clone)]
pub(crate) struct Entry {
  pub(crate) url: String,
  pub(crate) level: String,
  pub(crate) terms: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(crate) struct Course {
  /// A unique alphanumeric identifier
  pub(crate) id: String,
  /// The full name of the course
  pub(crate) title: String,
  /// The mcgill subject code, e.g. MATH, COMP
  pub(crate) subject: String,
  /// Thhe mcgill course code, e.g. 140, 141
  pub(crate) code: String,
  /// The course level, e.g. Undergraduate, Graduate
  pub(crate) level: String,
  /// Points to a link to the course on https://www.mcgill.ca
  pub(crate) url: String,
  /// The department in which this course is offered, e.g. Management
  pub(crate) department: String,
  /// Points to the department page in which this course is offered
  pub(crate) department_url: String,
  /// Terms in which this course is offered, e.g. Fall 2022, Winter 2023
  pub(crate) terms: Vec<String>,
  /// The full text description of the course
  pub(crate) description: String,
  /// All instructors that teach this course for all offered terms
  pub(crate) instructors: String,
}
