use axum::{routing::get, Json, Router};
use serde::Serialize;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

#[derive(Serialize)]
struct Project {
    id: i32,
    title: String,
    description: String,
}

async fn get_projects() -> Json<Vec<Project>> {
    Json(vec![
        Project {
            id: 1,
            title: "Rust + Next.js Portfolio".into(),
            description: "Full-stack site".into(),
        },
        Project {
            id: 2,
            title: "Streaming Setup".into(),
            description: "Moonlight 1080p 75Hz".into(),
        },
    ])
}

async fn root() -> &'static str {
    "Backed is up now. Do some shit!"
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cors = CorsLayer::new().allow_origin(Any);

    // Build the router
    let app = Router::new()
        .route("/", get(root))
        .route("/projects", get(get_projects))
        .layer(cors);

    // Bind manually with TcpListener (new in Axum 0.7)
    let listener = TcpListener::bind("127.0.0.1:8000").await?;
    println!("🚀 Backend running on http://127.0.0.1:8000");

    axum::serve(listener, app).await?;

    Ok(())
}
