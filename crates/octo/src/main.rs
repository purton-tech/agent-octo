use axum::Router;
use axum::http::header;
use axum::response::{Html, IntoResponse};
use axum::routing::get;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .with_target(false)
        .init();

    let app = Router::new().route("/", get(index));
    let bind_addr = std::env::var("OCTO_BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());
    let listener = tokio::net::TcpListener::bind(&bind_addr).await?;

    info!(bind_addr, "octo web server started");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn index() -> impl IntoResponse {
    (
        [(header::CONTENT_TYPE, "text/html; charset=utf-8")],
        Html(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Octo Under Construction</title>
  <style>
    :root {
      color-scheme: light;
    }

    body {
      margin: 0;
      min-height: 100vh;
      display: grid;
      place-items: center;
      background:
        radial-gradient(circle at top left, #ffff66 0, transparent 28%),
        radial-gradient(circle at bottom right, #00ffff 0, transparent 24%),
        repeating-linear-gradient(
          45deg,
          #001a66 0,
          #001a66 14px,
          #003399 14px,
          #003399 28px
        );
      color: #00ff33;
      font-family: "Courier New", monospace;
    }

    .frame {
      width: min(92vw, 760px);
      border: 6px ridge #c0c0c0;
      background: #000033;
      box-shadow: 0 0 0 6px #000, 18px 18px 0 rgba(0, 0, 0, 0.45);
      padding: 2rem;
      text-align: center;
    }

    h1 {
      margin: 0;
      font-size: clamp(2.5rem, 8vw, 5rem);
      line-height: 0.95;
      letter-spacing: 0.12em;
      text-transform: uppercase;
      text-shadow: 3px 3px 0 #ff00ff;
    }

    .blink {
      margin: 1.5rem 0;
      font-size: 1.2rem;
      color: #ffff00;
      animation: blink 1s steps(2, start) infinite;
    }

    p {
      margin: 0.75rem 0;
      font-size: 1rem;
      line-height: 1.5;
    }

    .marquee {
      margin-top: 1.5rem;
      overflow: hidden;
      border: 3px inset #c0c0c0;
      background: #111;
      color: #ffcc00;
      white-space: nowrap;
    }

    .marquee span {
      display: inline-block;
      padding: 0.5rem 1rem;
      animation: scroll 12s linear infinite;
    }

    @keyframes blink {
      to {
        opacity: 0;
      }
    }

    @keyframes scroll {
      from {
        transform: translateX(100%);
      }

      to {
        transform: translateX(-100%);
      }
    }
  </style>
</head>
<body>
  <main class="frame">
    <h1>Under<br>Construction</h1>
    <div class="blink">[ WORK IN PROGRESS ]</div>
    <p>Welcome to <strong>Octo</strong>, the future home of a dangerously serious web app.</p>
    <p>Please excuse the neon debris while we assemble the server-rendered empire.</p>
    <div class="marquee">
      <span>Best viewed with unreasonable optimism and a fresh copy of Netscape Navigator.</span>
    </div>
  </main>
</body>
</html>"#,
        ),
    )
}
