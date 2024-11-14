use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CalculateRefundRequestDTO {
    pub sales_order_no: String,
    pub service_type: String,
    pub order_promo_infos: Option<Vec<OrderPromoInfo>>,
    pub item_refunds: Option<Vec<ItemRefund>>,
    pub payments: Option<Vec<Payment>>,
    pub fair_promo_infos: Option<Vec<FairPromoInfo>>,
    pub customer_saldo: Option<Decimal>,
    pub refund_additional: Option<Decimal>,
    pub admin_fee: Option<Decimal>,
    pub amount_edited: Option<Decimal>,
    pub shipping_code: Option<String>,
    pub shipment_amount: Option<Decimal>,
    pub is_prorate: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderPromoInfo {
    pub order_detail_promo_info_id: Option<i64>,
    pub promo_qty: Option<i32>,
    pub promo_amount: Option<Decimal>,
    pub point_amount: Option<Decimal>,
    pub stamp_amount: Option<Decimal>,
    pub promo_kind_id: Option<i64>,
    pub products: Option<Vec<Product>>,
    pub summary: Option<Summary>,
    pub promo_kind_priority: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub product_klik_uuid: Option<String>,
    pub product_code: Option<String>,
    pub product_name: Option<String>,
    pub qty: Option<i32>,
    pub final_amount: Option<Decimal>,
    pub tax: Option<i32>,
    pub ppn_value: Option<Decimal>,
    pub virtual_admin_fee: Option<Decimal>,
    pub seller_code: Option<String>,
    pub group_seller_code: Option<String>,
    pub seller_settlement_group: Option<bool>,
    pub no_cup: Option<Vec<i32>>,
    pub pair_product: Option<Vec<OrderPromoInfo>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Summary {
    pub product_amount: Option<Decimal>,
    pub promo_amount: Option<Decimal>,
    pub final_amount: Option<Decimal>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemRefund {
    pub order_detail_promo_info_id: Option<i64>,
    pub product_code: Option<String>,
    pub qty_after: Option<i32>,
    pub qty_before: Option<i32>,
    pub qty_refund: Option<i32>,
    pub qty_after_konversi: Option<i32>,
    pub qty_before_konversi: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    pub pay_center_code: Option<String>,
    pub payment_type_code: Option<String>,
    pub payment_type_name: Option<String>,
    pub external_transaction_code: Option<String>,
    pub amount: Option<Decimal>,
    pub refund_amount: Option<Decimal>,
    pub force_refund: Option<bool>,
    pub transaction_code: Option<String>,
    pub is_limit: Option<bool>,
    pub total_saldo_after_refund: Option<Decimal>,
    pub so_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FairPromoInfo {
    pub order_promo_id: Option<i64>,
    pub promo_code: Option<String>,
    pub plu_list: Option<Vec<String>>,
    pub min_transaction_amount: Option<Decimal>,
    pub total_transaction_amount: Option<Decimal>,
    pub amount: Option<Decimal>,
    pub final_amount: Option<Decimal>,
    pub percentage_amount: Option<Decimal>,
    pub promo_qty: Option<i32>,
}

