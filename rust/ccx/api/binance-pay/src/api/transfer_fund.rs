use std::str::FromStr;

#[cfg(feature = "db")]
use diesel_derives::{AsExpression, FromSqlRow};
use rust_decimal::Decimal;
use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use crate::api::Api;
use crate::error::LibResult;
use crate::types::enums::StatusRequest;
use crate::types::time::Time;
use crate::uuid_simple;

pub const BINANCEPAY_OPENAPI_TRANSFER_FUND: &str = "/binancepay/openapi/wallet/transfer";

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "db", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "db", sql_type = "diesel::sql_types::Text")]
pub enum TransferType {
    #[serde(rename = "TO_MAIN")]
    ToMain,
    #[serde(rename = "TO_PAY")]
    ToPay,
}
forward_display_to_serde!(TransferType);
forward_from_str_to_serde!(TransferType);

impl TransferType {
    pub fn from_name(name: &str) -> Option<Self> {
        Self::from_str(name).ok()
    }

    pub fn name(&self) -> String {
        self.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "db", derive(AsExpression, FromSqlRow))]
#[cfg_attr(feature = "db", sql_type = "diesel::sql_types::Text")]
pub enum TransferStatus {
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "FAILURE")]
    Failure,
    #[serde(rename = "PROCESS")]
    Process,
}
forward_display_to_serde!(TransferStatus);
forward_from_str_to_serde!(TransferStatus);

impl TransferStatus {
    pub fn from_name(name: &str) -> Option<Self> {
        Self::from_str(name).ok()
    }

    pub fn name(&self) -> String {
        self.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransferFundRequest {
    #[serde(rename = "requestId", with = "uuid_simple")]
    pub request_id: Uuid, //string	Y	maximum length 32	Represents the unique ID of each transfer request.Generated by the merchant
    #[serde(rename = "merchantId")]
    pub merchant_id: u64, //long	Y	-	The merchant account id, issued when merchant been created at Binance.
    pub currency: String, //string	Y	Not limited, as long as it is within the range.	transfer currency, e.g. "BUSD"
    pub amount: Decimal,  //  string	Y	Greater than 0	the transfer amount
    #[serde(rename = "transferType")]
    pub transfer_type: TransferType, //    string  Y   Only "TO_MAIN" OR "TO_PAY"	The transfer direction specified by the merchant
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransferResult {
    #[serde(rename = "tranId")]
    pub transfer_id: String, //	string	Y	-	Used to query the transfer status, query the necessary fields for the transfer status
    pub amount: Decimal,        //	string	Y	-	the transfer amount
    pub status: TransferStatus, //	string	Y	SUCCESS OR FAILURE OR PROCESS	SUCCESS (indicating that the transfer is completely successful), FAILURE (indicating that the transfer has failed, it may be that the transferor has a problem with the transferee), PROCESS (the transfer is in progress)
    pub currency: String, //	string	Y	Not limited, as long as it is within the range.	transfer currency, e.g. "BUSD"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransferFundResponse {
    pub status: StatusRequest, // string	            Y	"SUCCESS" or "FAIL"	status of the API request
    pub code: String,          // string	            Y	-	request result code, refer to
    pub data: Option<TransferResult>, // TransferResult	    N	-	response body, refer to
    #[serde(rename = "errorMessage")]
    pub error_message: Option<String>, // string	            Y	-
}

impl<S: crate::client::BinancePaySigner> Api<S> {
    pub async fn transfer_fund(
        &self,
        request: TransferFundRequest,
        time_window: impl Into<Time>,
    ) -> LibResult<TransferFundResponse> {
        self.client
            .post_json(BINANCEPAY_OPENAPI_TRANSFER_FUND, request)?
            .signed(time_window)?
            .random_nonce()?
            .send()
            .await
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_serde_transfer_request() {
        let json = r#"
        {
            "requestId": "9a1c04a06dbc432e94fa4e2bd693c663",
            "merchantId": 98765987,
            "currency": "BNB",
            "amount": "0.01",
            "transferType": "TO_MAIN"
        }
        "#;
        let request: TransferFundRequest = serde_json::from_str(json).expect("Failed from_str");
        println!("test_serde_transfer_request :: {:#?}", request);

        let request = TransferFundRequest {
            request_id: uuid::Uuid::parse_str("9a1c04a0-6dbc-432e-94fa-4e2bd693c663")
                .expect("Failed parse_str"),
            merchant_id: 134697918,
            currency: "BUSD".to_string(),
            amount: Decimal::new(1, 2),
            transfer_type: TransferType::ToMain,
        };
        let json = serde_json::to_string(&request).expect("Failed to_string");
        println!("test_serde_transfer_request :: {}", json);
    }

    #[test]
    fn test_serde_transfer_response() {
        let example = r#"
        {
            "status": "SUCCESS",
            "code": "000000",
            "data": {
              "tranId": "4069044573",
              "amount": "0.01",
              "status": "SUCCESS",
              "currency": "BNB"
            },
            "errorMessage": ""
        }
        "#;
        let response: TransferFundResponse =
            serde_json::from_str(example).expect("Failed from_str");
        println!("test_serde_transfer_response response :: {:#?}", response);
    }
}
