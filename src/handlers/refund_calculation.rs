use actix_web::{web, HttpResponse, Responder};
use log::info;
use crate::model::calculate_refund_request::CalculateRefundRequestDTO;
use crate::model::calculate_refund_response::CalculateRefundResponseDTO;


pub async fn get_calculation(calculation_request: web::Json<CalculateRefundRequestDTO>) -> impl Responder {
    let req = calculation_request.into_inner();

    let response_data = calculate_refund(req);
    HttpResponse::Ok().json({response_data
    })
}

pub fn calculate_refund(request: CalculateRefundRequestDTO) -> CalculateRefundResponseDTO {
    let result = CalculateRefundResponseDTO {
        sales_order_no: Option::from(request.sales_order_no.clone()),
        service_type: Option::from(request.service_type.clone()),
        refund_type: None,
        refund_infos: None,
        products_to_refund :None,
        fair_promo_infos:None,
        approval_status:None,
        refund_payment: None,
    };

    result
}