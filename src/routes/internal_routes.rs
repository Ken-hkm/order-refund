use actix_web::web;
use crate::handlers::refund_calculation::{get_calculation};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/rfd/internal")
            .route("/calculate-refund",web::post().to(get_calculation)),
    );
}