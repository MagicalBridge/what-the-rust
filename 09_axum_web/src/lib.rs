pub mod config;
pub mod db;
pub mod error;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod utils;

pub use config::Config;
pub use db::{create_connection_pool, run_migrations, DbPool};
pub use error::AppError;
pub use handlers::user::*;
pub use models::user::*;
pub use routes::user::create_user_routes;