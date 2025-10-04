use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HealthResponse {
  pub status: String,
  pub version: Option<String>,
  pub checks: Vec<HealthCheck>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HealthCheck {
  pub name: String,
  pub status: String,
  pub details: Option<String>,
  pub duration: Option<u64>,
}
