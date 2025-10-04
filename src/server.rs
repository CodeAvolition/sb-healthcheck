use crate::cache::Cache;
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
    <title>Healthcheck Dashboard</title>
    <style>
      body { 
    font-family: monospace; 
    margin: 10px; 
    background: #1e1e1e; 
    color: #d4d4d4; 
    font-size: 12px;
}
h1 { 
    color: #4ec9b0; 
    margin: 10px 0; 
    font-size: 18px;
}
h2 {
    font-size: 14px;
    margin: 5px 0;
    padding: 5px;
    background: #2d2d2d;
    border-left: 3px solid #569cd6;
}
.env { 
    margin: 15px 0; 
    padding: 10px;
    background: #252526;
    border-radius: 4px;
}
.check { 
    margin: 8px 0; 
    padding: 8px; 
    border-left: 3px solid #555; 
    background: #1e1e1e;
}
.healthy { border-left-color: #4ec9b0; }
.unhealthy { border-left-color: #f48771; }
.error { border-left-color: #ce9178; }
.version { 
    color: #9cdcfe; 
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
    background: #2d2d2d; 
    font-weight: normal;
}
td { 
    padding: 2px 5px; 
    border-top: 1px solid #3e3e3e; 
}
tr.healthy td:nth-child(2) { color: #4ec9b0; }
tr.unhealthy td:nth-child(2) { color: #f48771; }

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
      "<div class='env'><h2>{}</h2>",
      env.name
    ));

    for check in &env.checks {
      let key = format!("{}:{}", env.name, check.name);

      if let Some(result) = state.cache.get(&key) {
        let status_class = match result.status {
          Status::Healthy => "healthy",
          Status::Unhealthy => "unhealthy",
          Status::Error => "error",
        };

        let status_text = match result.status {
          Status::Healthy => "✓",
          Status::Unhealthy => "✗",
          Status::Error => "⚠",
        };

        html.push_str(&format!(
          "<div class='check {}'><strong>{}</strong>",
          status_class, result.name
        ));

        if let Some(version) = &result.version {
          html.push_str(&format!(
            " <span class='version'>v{}</span>",
            version
          ));
        }

        // Always show a table (even for frontends with just overall status)
        html.push_str("<table><tr><th>Component</th><th>Status</th><th>Details</th></tr>");

        if result.sub_checks.is_empty() {
          // Frontend or simple check - show overall status
          html.push_str(&format!(
                "<tr class='{}'><td>Overall</td><td>{}</td><td>-</td></tr>",
                status_class, status_text
            ));
        } else {
          // Backend with sub-checks
          for sub in &result.sub_checks {
            let sub_class = if sub.status == "Healthy" {
              "healthy"
            } else {
              "unhealthy"
            };
            let sub_icon = if sub.status == "Healthy" {
              "✓"
            } else {
              "✗"
            };
            html.push_str(&format!(
                    "<tr class='{}'><td>{}</td><td>{}</td><td>{}</td></tr>",
                    sub_class,
                    sub.name,
                    sub_icon,
                    sub.details.as_deref().unwrap_or("-")
                ));
          }
        }
        html.push_str("</table></div>");
      }
    }

    html.push_str("</div>");
  }

  html.push_str("</body></html>");
  Html(html)
}
