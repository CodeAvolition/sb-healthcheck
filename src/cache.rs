use crate::checker::run_check;
use crate::models::{Check, CheckResult, Config};
use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub type Cache = Arc<DashMap<String, CheckResult>>;

pub fn create_cache() -> Cache {
  Arc::new(DashMap::new())
}

fn cache_key(env_name: &str, check_name: &str) -> String {
  format!("{}:{}", env_name, check_name)
}

pub async fn poll_all_checks(
  config: &Config,
  cache: Cache,
) {
  for env in &config.environments {
    for check in &env.checks {
      let key = cache_key(&env.name, &check.name);
      let result = run_check(check).await;
      cache.insert(key, result);
    }
  }
}

pub fn is_stale(
  result: &CheckResult,
  timeout_secs: u64,
) -> bool {
  result.last_checked.elapsed()
    > Duration::from_secs(timeout_secs)
}

pub async fn start_polling_loop(
  config: Config,
  cache: Cache,
) {
  let stale_timeout = config.stale_timeout_seconds;

  loop {
    for env in &config.environments {
      for check in &env.checks {
        let key = cache_key(&env.name, &check.name);

        // Check if we need to refresh
        let should_poll = cache
          .get(&key)
          .map(|entry| is_stale(&entry, stale_timeout))
          .unwrap_or(true); // Poll if not in cache yet

        if should_poll {
          let result = run_check(check).await;
          cache.insert(key, result);
        }
      }
    }

    // Sleep briefly before next iteration
    tokio::time::sleep(Duration::from_secs(1)).await;
  }
}
