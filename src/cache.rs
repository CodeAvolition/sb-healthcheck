use crate::checker::run_check;
use crate::models::{CheckResult, Config};
use dashmap::DashMap;
use std::sync::Arc;
use std::time::Duration;

pub type Cache = Arc<DashMap<String, CheckResult>>;

pub fn create_cache() -> Cache {
  Arc::new(DashMap::new())
}

fn cache_key(env_name: &str, check_name: &str) -> String {
  format!("{}:{}", env_name, check_name)
}

pub async fn start_polling_loop(
  config: Config,
  cache: Cache,
) {
  loop {
    for env in &config.environments {
      for check in &env.checks {
        let key = cache_key(&env.name, &check.name);
        let result = run_check(check).await;
        cache.insert(key, result);
      }
    }

    tokio::time::sleep(Duration::from_secs(
      config.stale_timeout_seconds,
    ))
    .await;
  }
}
