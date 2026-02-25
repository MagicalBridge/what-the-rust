use crate::db::DbPool;
use crate::handlers::{
    create_user, delete_user, get_user, get_users, update_user,
};
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn create_user_routes() -> Router<DbPool> {
    Router::new()
        .route("/", post(create_user))
        .route("/", get(get_users))
        .route("/:id", get(get_user))
        .route("/:id", put(update_user))
        .route("/:id", delete(delete_user))
}