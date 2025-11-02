use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, ToSchema, Validate, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    #[validate(length(min = 1, max = 100))]
    pub id: String,
    
    #[validate(length(min = 1, max = 100))]
    pub user: String,
    
    #[validate(length(min = 1, max = 50))]
    pub market: String,
    
    #[validate(length(equal = 3))]
    pub side: String,
    
    #[validate(range(min = 0.0))]
    pub price: f64,
    
    #[validate(range(min = 0.0))]
    pub amount: f64,
}

#[derive(Serialize, Deserialize, ToSchema, Validate, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderRequest {
    #[validate(length(min = 1, max = 100))]
    pub user: String,
    
    #[validate(length(min = 1, max = 50))]
    pub market: String,
}

#[derive(Serialize, Deserialize, ToSchema, Validate, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrderResponse {
    pub orders: Vec<Order>,
}