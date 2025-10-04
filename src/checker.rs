use crate::models::{
  Check, CheckResult, CheckType, HealthResponse, Status,
};

pub async fn fetch_health(
  url: &str,
) -> Result<HealthResponse, reqwest::Error> {
  let client = reqwest::Client::new();
  let response = client
    .get(url)
    .timeout(std::time::Duration::from_secs(5))
    .send()
    .await?;

  response.json::<HealthResponse>().await
}
pub async fn check_keyword(
  url: &str,
  keyword: &str,
) -> Result<bool, reqwest::Error> {
  let client = reqwest::Client::new();
  let response = client
    .get(url)
    .timeout(std::time::Duration::from_secs(5))
    .send()
    .await?;

  let body = response.text().await?;
  Ok(body.contains(keyword))
}

pub async fn run_check(check: &Check) -> CheckResult {
  let now = std::time::Instant::now();

  match check.check_type {
    CheckType::Health => {
      match fetch_health(&check.url).await {
        Ok(health) => CheckResult {
          name: check.name.clone(),
          url: check.url.clone(),
          status: if health.status == "Healthy" {
            Status::Healthy
          } else {
            Status::Unhealthy
          },
          version: health.version,
          sub_checks: health.checks,
          last_checked: now,
        },
        Err(_) => CheckResult {
          name: check.name.clone(),
          url: check.url.clone(),
          status: Status::Error,
          version: None,
          sub_checks: vec![],
          last_checked: now,
        },
      }
    }

    CheckType::Keyword => {
      if let Some(keyword) = &check.keyword {
        match check_keyword(&check.url, keyword).await {
          Ok(true) => CheckResult {
            name: check.name.clone(),
            url: check.url.clone(),
            status: Status::Healthy,
            version: None,
            last_checked: now,
            sub_checks: vec![],
          },
          Ok(false) => CheckResult {
            name: check.name.clone(),
            url: check.url.clone(),
            status: Status::Unhealthy,
            version: None,
            last_checked: now,
            sub_checks: vec![],
          },
          Err(_) => CheckResult {
            name: check.name.clone(),
            url: check.url.clone(),
            status: Status::Error,
            version: None,
            last_checked: now,
            sub_checks: vec![],
          },
        }
      } else {
        CheckResult {
          name: check.name.clone(),
          url: check.url.clone(),
          status: Status::Error,
          version: None,
          last_checked: now,
          sub_checks: vec![],
        }
      }
    }
  }
}
