use crate::controllers::auth;
use axum::{routing::post, Router};

pub fn routes() -> axum::Router {
    Router::new().route("/sign-up", post(auth::sign_up))
}