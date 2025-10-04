use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum CheckType {
  Health,
  Keyword,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
  Healthy,
  Unhealthy,
  Error,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Check {
  pub name: String,
  pub url: String,
  pub check_type: CheckType,
  pub keyword: Option<String>,
}

use super::health::HealthCheck;

#[derive(Debug, Clone)]
pub struct CheckResult {
  pub name: String,
  pub url: String,
  pub status: Status,
  pub version: Option<String>,
  pub sub_checks: Vec<HealthCheck>,
  pub last_checked: std::time::Instant,
}
