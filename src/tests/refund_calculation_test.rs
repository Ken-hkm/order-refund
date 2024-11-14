use crate::model::calculate_refund_request::CalculateRefundRequestDTO;
use crate::model::calculate_refund_response::CalculateRefundResponseDTO;

#[cfg(test)]
mod tests {
    use super::*;

    // Test for the `calculate_refund` function
    #[test]
    fn test_calculate_refund() {
        let request = CalculateRefundRequestDTO {
            sales_order_no: "SO12345".to_string(),
            service_type: "FOOD".to_string(),
            // Add other necessary fields with sample data as needed
            ..Default::default()
        };

        let response = calculate_refund(request);

        // Assert that the response contains the expected values
        assert_eq!(response.sales_order_no.unwrap(), "SO12345");
        assert_eq!(response.service_type.unwrap(), "FOOD");
        assert_eq!(response.refund_type, None);
        assert_eq!(response.refund_infos, None);
        assert_eq!(response.products_to_refund, None);
        assert_eq!(response.fair_promo_infos, None);
        assert_eq!(response.approval_status, None);
        assert_eq!(response.refund_payment, None);
    }
}