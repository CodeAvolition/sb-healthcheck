use super::check::Check;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
  pub project_name: String,
  pub stale_timeout_seconds: u64,
  pub environments: Vec<Environment>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Environment {
  pub name: String,
  pub checks: Vec<Check>,
}
