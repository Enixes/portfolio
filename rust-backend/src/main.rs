use std::env;
use axum::{routing::get, Json, Router};
use axum::extract::State;
use dotenvy::dotenv;
use serde::Serialize;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

#[derive(Serialize, sqlx::FromRow)]
struct Project {
    id: i32,
    title: String,
    description: String,
    route: String,
}

impl Project {
    fn new(id: i32, title: String, description: String) -> Self {
        Self {
            id,
            title,
            description,
            route: String::from(""),
        }

    }
}

async fn get_projects(State(pool): State<PgPool>) -> Json<Vec<Project>> {
    let projects = sqlx::query_as::<_, Project>("select id, title, description, route from projects")
        .fetch_all(&pool)
        .await
        .unwrap_or_else(|_| vec![]);
    Json(projects)
}

async fn root() -> &'static str {
    "Backed is up now. Do some shit!"
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let cors = CorsLayer::new().allow_origin(Any);

    let database_url = env::var("DATABASE_URL")?;
    let pool = PgPool::connect(&database_url).await?;


    // Build the router
    let app = Router::new()
        .route("/", get(root))
        .route("/projects", get(get_projects))
        .with_state(pool)
        .layer(cors);

    // Bind manually with TcpListener (new in Axum 0.7)
    let listener = TcpListener::bind("127.0.0.1:8000").await?;
    println!("🚀 Backend running on http://127.0.0.1:8000");

    axum::serve(listener, app).await?;

    Ok(())
}
