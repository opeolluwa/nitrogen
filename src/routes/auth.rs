use crate::controllers;
use axum::{routing::post, Router};

pub fn routes() -> axum::Router {
    Router::new().route("/sign-up", post(controllers::auth::sign_up))
}
