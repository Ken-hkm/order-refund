use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CalculateRefundResponseDTO {
    #[serde(rename = "salesOrderNo")]
    pub sales_order_no: Option<String>,

    #[serde(rename = "serviceType")]
    pub service_type: Option<String>,

    #[serde(rename = "refundType")]
    pub refund_type: Option<String>,

    #[serde(rename = "refundInfos")]
    pub refund_infos: Option<Vec<RefundInfo>>,

    #[serde(rename = "productsToRefund")]
    pub products_to_refund: Option<Vec<PromoPerProduct>>,

    #[serde(rename = "fairPromoInfos")]
    pub fair_promo_infos: Option<Vec<FairPromoInfo>>,

    #[serde(rename = "approvalStatus")]
    pub approval_status: Option<String>,

    #[serde(rename = "refundPayment")]
    pub refund_payment: Option<Payment>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefundInfo {
    #[serde(rename = "orderDetailPromoInfoId")]
    pub order_detail_promo_info_id: Option<i64>,

    #[serde(rename = "totalRefundAmount")]
    pub total_refund_amount: Option<Decimal>,

    #[serde(rename = "totalFulfillmentAmount")]
    pub total_fulfillment_amount: Option<Decimal>,

    #[serde(rename = "totalPromoRefundAmount")]
    pub total_promo_refund_amount: Option<Decimal>,

    #[serde(rename = "totalPromoFulfillmentAmount")]
    pub total_promo_fulfillment_amount: Option<Decimal>,

    pub tax: Option<i64>,

    #[serde(rename = "ppnValue")]
    pub ppn_value: Option<i64>,

    #[serde(rename = "sellerCode")]
    pub seller_code: Option<String>,

    #[serde(rename = "groupSellerCode")]
    pub group_seller_code: Option<String>,

    #[serde(rename = "sellerSettlementGroup")]
    pub seller_settlement_group: Option<bool>,

    #[serde(rename = "products")]
    pub products: Option<Vec<Product>>,
}

impl RefundInfo {
    pub fn total_pair_product_refund_amount(&self) -> Decimal {
        if let Some(products) = &self.products {
            return products
                .iter()
                .map(|product| product.total_pair_product_refund_amount())
                .fold(Decimal::ZERO, |acc, val| acc + val);
        }
        Decimal::ZERO
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    #[serde(rename = "productKlikUuid")]
    pub product_klik_uuid: Option<String>,

    #[serde(rename = "productCode")]
    pub product_code: Option<String>,

    #[serde(rename = "productName")]
    pub product_name: Option<String>,

    pub qty: Option<i32>,

    #[serde(rename = "fulfillmentQty")]
    pub fulfillment_qty: Option<i32>,

    #[serde(rename = "refundQty")]
    pub refund_qty: Option<i32>,

    #[serde(rename = "productAmount")]
    pub product_amount: Option<Decimal>,

    #[serde(rename = "productAmountPerQty")]
    pub product_amount_per_qty: Option<Decimal>,

    #[serde(rename = "promoAmount")]
    pub promo_amount: Option<Decimal>,

    #[serde(rename = "promoAmountPerQty")]
    pub promo_amount_per_qty: Option<Decimal>,

    #[serde(rename = "refundAmount")]
    pub refund_amount: Option<Decimal>,

    #[serde(rename = "promoRefundAmount")]
    pub promo_refund_amount: Option<Decimal>,

    #[serde(rename = "fairRefundAmount")]
    pub fair_refund_amount: Option<Decimal>,

    #[serde(rename = "fairFulfillmentAmount")]
    pub fair_fulfillment_amount: Option<Decimal>,

    pub tax: Option<i32>,

    #[serde(rename = "ppnValue")]
    pub ppn_value: Option<Decimal>,

    #[serde(rename = "sellerCode")]
    pub seller_code: Option<String>,

    #[serde(rename = "groupSellerCode")]
    pub group_seller_code: Option<String>,

    #[serde(rename = "sellerSettlementGroup")]
    pub seller_settlement_group: Option<bool>,

    #[serde(rename = "noCup")]
    pub no_cup: Option<Vec<i32>>,

    #[serde(rename = "pairProduct")]
    pub pair_product: Option<Vec<RefundInfo>>,
}

impl Product {
    pub fn total_pair_product_refund_amount(&self) -> Decimal {
        if let Some(pair_product) = &self.pair_product {
            return pair_product
                .iter()
                .map(|refund_info| refund_info.total_refund_amount.unwrap_or(Decimal::ZERO))
                .fold(Decimal::ZERO, |acc, val| acc + val);
        }
        Decimal::ZERO
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromoPerProduct {
    #[serde(rename = "productCode")]
    pub product_code: Option<String>,

    #[serde(rename = "refundAmount")]
    pub refund_amount: Option<Decimal>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payment {
    #[serde(rename = "hasLimit")]
    pub has_limit: Option<bool>,

    #[serde(rename = "IsLimitSaldo")]
    pub limit_saldo: Option<bool>,

    #[serde(rename = "approvalStatus")]
    pub approval_status: Option<String>,

    pub status: Option<String>,

    #[serde(rename = "bankAccountStatus")]
    pub bank_account_status: Option<String>,

    #[serde(rename = "refundPaymentType")]
    pub refund_payment_type: Option<String>,

    #[serde(rename = "refundAdminFee")]
    pub refund_admin_fee: Option<Decimal>,

    #[serde(rename = "refundAdditional")]
    pub refund_additional: Option<Decimal>,

    #[serde(rename = "refundShipmentAmount")]
    pub refund_shipment_amount: Option<Decimal>,

    #[serde(rename = "paymentDetails")]
    pub payment_details: Option<Vec<PaymentDetail>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentDetail {
    #[serde(rename = "payCenterCode")]
    pub pay_center_code: Option<String>,

    #[serde(rename = "paymentTypeCode")]
    pub payment_type_code: Option<String>,

    #[serde(rename = "paymentTypeName")]
    pub payment_type_name: Option<String>,

    #[serde(rename = "externalTransactionCode")]
    pub external_transaction_code: Option<String>,

    pub amount: Option<Decimal>,

    #[serde(rename = "refundAmount")]
    pub refund_amount: Option<Decimal>,

    #[serde(rename = "forceRefund")]
    pub force_refund: Option<bool>,

    #[serde(rename = "transactionCode")]
    pub transaction_code: Option<String>,

    #[serde(rename = "isLimit")]
    pub is_limit: Option<bool>,

    #[serde(rename = "totalSaldoAfterRefund")]
    pub total_saldo_after_refund: Option<Decimal>,

    #[serde(rename = "soToken")]
    pub so_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FairPromoInfo {
    #[serde(rename = "orderPromoId")]
    pub order_promo_id: Option<i64>,

    #[serde(rename = "promoCode")]
    pub promo_code: Option<String>,

    #[serde(rename = "fulfillmentAmount")]
    pub fulfillment_amount: Option<Decimal>,

    #[serde(rename = "refundAmount")]
    pub refund_amount: Option<Decimal>,

    #[serde(rename = "promoQty")]
    pub promo_qty: Option<i32>,
}