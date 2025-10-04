use crate::cache::Cache;
use crate::models::config::Environment;
use crate::models::{Config, Status};
use axum::{
  Router, extract::State, response::Html, routing::get,
};
use std::sync::Arc;

pub struct AppState {
  pub cache: Cache,
  pub config: Config,
}

pub async fn start_server(config: Config, cache: Cache) {
  let state = Arc::new(AppState { cache, config });

  let app = Router::new()
    .route("/", get(dashboard))
    .with_state(state);

  let listener =
    tokio::net::TcpListener::bind("0.0.0.0:3000")
      .await
      .unwrap();

  println!("Dashboard running on http://0.0.0.0:3000");
  axum::serve(listener, app).await.unwrap();
}

async fn dashboard(
  State(state): State<Arc<AppState>>,
) -> Html<String> {
  let mut html = String::from(
    r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta http-equiv="refresh" content="5">
    <title>Healthcheck Dashboard</title>
    <style>
        :root {
            --bg-primary: #1e1e1e;
            --bg-secondary: #252526;
            --bg-tertiary: #2d2d2d;
            --border-color: #3e3e3e;
            --text-primary: #d4d4d4;
            --text-accent: #569cd6;
            --text-version: #9cdcfe;
            --status-healthy: #4ec9b0;
            --status-unhealthy: #f48771;
            --status-error: #ce9178;
        }
        body { 
            font-family: monospace; 
            margin: 10px; 
            background: var(--bg-primary); 
            color: var(--text-primary); 
            font-size: 12px;
        }
        h1 { 
            color: var(--status-healthy); 
            margin: 10px 0; 
            font-size: 18px;
        }
        .env { 
            margin: 10px 0; 
            padding: 10px;
            background: var(--bg-secondary);
            border-radius: 4px;
        }
        .env-name {
            font-size: 14px;
            font-weight: bold;
            margin-bottom: 10px;
            color: var(--text-accent);
        }
        .checks-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 10px;
            margin-bottom: 10px;
        }
        .check { 
            padding: 8px; 
            border-left: 3px solid;
            background: var(--bg-primary);
        }
        .check.healthy { 
            border-left-color: var(--status-healthy);
            background: color-mix(in srgb, var(--status-healthy) 5%, var(--bg-primary));
        }
        .check.unhealthy { 
            border-left-color: var(--status-unhealthy);
            background: color-mix(in srgb, var(--status-unhealthy) 5%, var(--bg-primary));
        }
        .check.error { 
            border-left-color: var(--status-error);
            background: color-mix(in srgb, var(--status-error) 5%, var(--bg-primary));
        }
        .check-name {
            font-weight: bold;
            margin-bottom: 5px;
        }
        .version { 
            color: var(--text-version); 
            font-size: 11px;
        }
        table { 
            width: 100%; 
            margin-top: 5px; 
            border-collapse: collapse; 
            font-size: 11px;
        }
        th { 
            text-align: left; 
            padding: 3px 5px; 
            background: var(--bg-tertiary); 
            font-weight: normal;
        }
        td { 
            padding: 2px 5px; 
            border-top: 1px solid var(--border-color); 
        }
        .status-cell {
            font-weight: bold;
        }
        tr.healthy .status-cell { color: var(--status-healthy); }
        tr.unhealthy .status-cell { color: var(--status-unhealthy); }
    </style>
</head>
<body>
"#,
  );

  html.push_str(&format!(
    "<h1>{}</h1>",
    state.config.project_name
  ));

  for env in &state.config.environments {
    html.push_str(&format!(
      "<div class='env'><div class='env-name'>{}</div>",
      env.name
    ));

    // Split into backends and frontends
    let backends: Vec<_> = env
      .checks
      .iter()
      .filter(|c| {
        matches!(
          c.check_type,
          crate::models::CheckType::Health
        )
      })
      .collect();
    let frontends: Vec<_> = env
      .checks
      .iter()
      .filter(|c| {
        matches!(
          c.check_type,
          crate::models::CheckType::Keyword
        )
      })
      .collect();

    // Render backends
    if !backends.is_empty() {
      html.push_str("<div class='checks-grid'>");
      for check in backends {
        render_check(&mut html, env, check, &state.cache);
      }
      html.push_str("</div>");
    }

    // Render frontends
    if !frontends.is_empty() {
      html.push_str("<div class='checks-grid'>");
      for check in frontends {
        render_check(&mut html, env, check, &state.cache);
      }
      html.push_str("</div>");
    }

    html.push_str("</div>");
  }

  html.push_str("</body></html>");
  Html(html)
}

fn render_check(
  html: &mut String,
  env: &Environment,
  check: &crate::models::Check,
  cache: &Cache,
) {
  let key = format!("{}:{}", env.name, check.name);

  if let Some(result) = cache.get(&key) {
    let status_class = match result.status {
      Status::Healthy => "healthy",
      Status::Unhealthy => "unhealthy",
      Status::Error => "error",
    };

    let status_text = match result.status {
      Status::Healthy => "Healthy ✓",
      Status::Unhealthy => "Unhealthy ✗",
      Status::Error => "Error ⚠",
    };

    html.push_str(&format!(
      "<div class='check {}'><div class='check-name'>{}",
      status_class, result.name
    ));

    if let Some(version) = &result.version {
      html.push_str(&format!(
        " <span class='version'>v{}</span>",
        version
      ));
    }
    html.push_str("</div>");

    html.push_str(
      "<table><tr><th>Component</th><th>Status</th></tr>",
    );

    if result.sub_checks.is_empty() {
      html.push_str(&format!(
                "<tr class='{}'><td>Overall</td><td class='status-cell'>{}</td></tr>",
                status_class, status_text
            ));
    } else {
      for sub in &result.sub_checks {
        let sub_class = if sub.status == "Healthy" {
          "healthy"
        } else {
          "unhealthy"
        };
        let sub_text = if sub.status == "Healthy" {
          "Healthy ✓"
        } else {
          "Unhealthy ✗"
        };
        html.push_str(&format!(
                    "<tr class='{}'><td>{}</td><td class='status-cell'>{}</td></tr>",
                    sub_class, sub.name, sub_text
                ));
      }
    }
    html.push_str("</table></div>");
  }
}
