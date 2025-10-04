#[cfg(test)]
mod tests {
  use crate::models::Config;

  #[test]
  fn test_valid_config_parsing() {
    let json = r#"{
            "project_name": "test-project",
            "stale_timeout_seconds": 30,
            "environments": [
                {
                    "name": "prod",
                    "checks": [
                        {
                            "name": "Backend",
                            "url": "https://example.com/health",
                            "check_type": "health"
                        }
                    ]
                }
            ]
        }"#;

    let config: Result<Config, _> =
      serde_json::from_str(json);
    assert!(config.is_ok());
    let config = config.unwrap();
    assert_eq!(config.project_name, "test-project");
    assert_eq!(config.environments.len(), 1);
  }

  #[test]
  fn test_config_with_keyword_check() {
    let json = r#"{
            "project_name": "test",
            "stale_timeout_seconds": 30,
            "environments": [{
                "name": "prod",
                "checks": [{
                    "name": "Frontend",
                    "url": "https://example.com",
                    "check_type": "keyword",
                    "keyword": "<title>Test</title>"
                }]
            }]
        }"#;

    let config: Result<Config, _> =
      serde_json::from_str(json);
    assert!(config.is_ok());
  }

  #[test]
  fn test_invalid_check_type() {
    let json = r#"{
            "project_name": "test",
            "stale_timeout_seconds": 30,
            "environments": [{
                "name": "prod",
                "checks": [{
                    "name": "Test",
                    "url": "https://example.com",
                    "check_type": "invalid_type"
                }]
            }]
        }"#;

    let config: Result<Config, _> =
      serde_json::from_str(json);
    assert!(config.is_err());
  }

  #[test]
  fn test_missing_keyword_for_keyword_check() {
    let json = r#"{
            "project_name": "test",
            "stale_timeout_seconds": 30,
            "environments": [{
                "name": "prod",
                "checks": [{
                    "name": "Frontend",
                    "url": "https://example.com",
                    "check_type": "keyword"
                }]
            }]
        }"#;

    let config: Result<Config, _> =
      serde_json::from_str(json);
    assert!(config.is_ok());
    let config = config.unwrap();
    assert!(
      config.environments[0].checks[0].keyword.is_none()
    );
  }

  #[test]
  fn test_empty_environments() {
    let json = r#"{
            "project_name": "test",
            "stale_timeout_seconds": 30,
            "environments": []
        }"#;

    let config: Result<Config, _> =
      serde_json::from_str(json);
    assert!(config.is_ok());
  }
}
