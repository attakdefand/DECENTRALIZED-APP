use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, ToSchema, Validate, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Market {
    #[validate(length(min = 1, max = 100))]
    pub id: String,
    
    #[validate(length(min = 1, max = 50))]
    pub base_token: String,
    
    #[validate(length(min = 1, max = 50))]
    pub quote_token: String,
    
    #[validate(range(min = 0.0))]
    pub price: f64,
}

#[derive(Serialize, Deserialize, ToSchema, Validate, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarketRequest {
    #[validate(length(min = 1, max = 50))]
    pub base_token: String,
    
    #[validate(length(min = 1, max = 50))]
    pub quote_token: String,
}

#[derive(Serialize, Deserialize, ToSchema, Validate, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MarketResponse {
    pub markets: Vec<Market>,
}