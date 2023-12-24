use axum::{
    extract::State,
    http::StatusCode,
    response::Response,
    Form,
};
use serde::Deserialize;
use askama::Template;
use axum::response::IntoResponse;
use axum::response::Html;
use crate::{AppState, templates};
use tracing::warn;

#[derive(Deserialize, Debug)]
pub struct SignupForm {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
}



pub(crate) async fn post_signup(
    State(state): State<AppState>,
    Form(form): Form<SignupForm>,
) -> (StatusCode, Response) {
    let user = match sqlx::query_as!(
        User,
        "INSERT INTO USERS (username, password) VALUES ($1, $2) RETURNING id, username",
        form.username,
        form.password
    )
    .fetch_one(&state.db)
    .await {
        Ok(user) => user,
        Err(e) => match e {
            sqlx::Error::Database(err) =>  {
                if err.is_unique_violation() {
                    return (
                        StatusCode::BAD_REQUEST,
                        Html("Username already exists").into_response(),
                    );
                } else {
                    warn!("Failed to create user: {:?}", err);
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Html("Failed to create user").into_response(),
                    );
                }
            }
            _ => {
                warn!("Failed to create user: {:?}", e);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Html("Failed to create user").into_response(),
                );
            }
        }
    };
    

    (
        StatusCode::OK,
        templates:: UserLoggedIn { user }
            .render()
            .unwrap()
            .into_response(),
    )
}