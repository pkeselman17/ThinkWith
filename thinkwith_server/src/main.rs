use axum::{
    routing::{post},
    Json, Router,
};
use sqlx::PgPool;
use tracing::{debug, error, info, trace, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Clone)]
struct AppState {
    _db: PgPool,
}

#[derive(Debug, serde::Deserialize)]
struct User {
    id: i32,
    name: String,
}

async fn hello_world(Json(user): Json<User>) -> Json<String> {
    Json(format!("Hello, {}! Your ID is {}", user.name, user.id))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
     let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO) // Set the maximum log level
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set global subscriber");

    info!("Application started");

    dotenv::dotenv().ok();

    let db = PgPool::connect(&dotenv::var("DATABASE_URL")?).await?;

    info!("Connected to database...");

    sqlx::migrate!()
        .run(&db)
        .await
        .expect("Failed to run migrations");

    info!("Migrations applied successfully...");

    let state = AppState { _db: db };
    let app= Router::new()
        .route("/", post(hello_world))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await?;

    info!("Server is running on http://0.0.0.0:3001");
    debug!("Listening on port 3001...");
    
    axum::serve(listener, app).await
        .map_err(|e| {
            error!("Server error: {}", e);
            e
        })?;

    Ok(())
}