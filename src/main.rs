mod templates;

use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{delete, get, post},
    Form, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::warn;

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

#[derive(Serialize, Clone, Debug, Deserialize, sqlx::FromRow)]
pub struct Todo {
    pub id: i32,
    pub description: String,
}
#[derive(Deserialize, Debug)]
pub struct TodoNew {
    pub description: String,
}

async fn handle_main() -> (StatusCode, Response) {
    let reply_html = templates::IndexTemplate {}.render().unwrap();
    (StatusCode::OK, Html(reply_html).into_response())
}

async fn delete_todo(State(state): State<AppState>, Path(id): Path<i32>) -> StatusCode {
    sqlx::query!("DELETE FROM TODOS WHERE id = $1", id)
        .execute(&state.db)
        .await
        .unwrap();
    StatusCode::OK
}

async fn list_todos(State(state): State<AppState>) -> (StatusCode, Response) {
    let todos = sqlx::query_as!(Todo, "SELECT * FROM TODOS")
        .fetch_all(&state.db)
        .await
        .unwrap();
    (
        StatusCode::OK,
        templates::Records { todos }
            .render()
            .unwrap()
            .into_response(),
    )
}

async fn create_todo(
    State(state): State<AppState>,
    Form(form): Form<TodoNew>,
) -> (StatusCode, Response) {
    let todo = sqlx::query_as!(
        Todo,
        "INSERT INTO TODOS (description) VALUES ($1) RETURNING *",
        form.description
    )
    .fetch_one(&state.db)
    .await
    .unwrap();

    (
        StatusCode::OK,
        templates::TodoNewTemplate { todo }
            .render()
            .unwrap()
            .into_response(),
    )
}

#[shuttle_runtime::main]
pub async fn axum(
    #[shuttle_shared_db::Postgres(local_uri = "{secrets.db}")] pool: PgPool,
    #[shuttle_secrets::Secrets] _secrets: shuttle_secrets::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    match sqlx::migrate!().run(&pool).await {
        Ok(_) => (),
        Err(e) => {
            warn!("Error: {}", e);
        }
    }

    let state = AppState { db: pool };

    let router = Router::new()
        .route("/", get(handle_main))
        .route("/todos", post(create_todo))
        .route("/todos", get(list_todos))
        .route("/todos/:id", delete(delete_todo))
        .with_state(state);

    Ok(router.into())
}
