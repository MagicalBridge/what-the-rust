use crate::handlers;
use actix_web::{Scope, web};

/// 配置用户相关路由
pub fn user_routes() -> Scope {
    web::scope("/api/users")
        .route("", web::post().to(handlers::create_user))
        .route("", web::get().to(handlers::get_all_users))
        .route("/{id}", web::get().to(handlers::get_user_by_id))
        .route(
            "/username/{username}",
            web::get().to(handlers::get_user_by_username),
        )
        .route("/{id}", web::put().to(handlers::update_user))
        .route("/{id}", web::delete().to(handlers::delete_user))
}

/// 配置健康检查路由
pub fn health_routes() -> impl Fn(&mut web::ServiceConfig) {
    |cfg: &mut web::ServiceConfig| {
        cfg.route("/health", web::get().to(handlers::health_check));
    }
}
