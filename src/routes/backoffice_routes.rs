use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/rfd/back-office")
            // .route("/test", web::get().to(json_example)),
            // .route("", web::get().to(get_users))
            // .route("/{id}", web::get().to(get_user_by_id))
            // .route("/{id}", web::put().to(update_user))
            // .route("/{id}", web::delete().to(delete_user)),
    );
}