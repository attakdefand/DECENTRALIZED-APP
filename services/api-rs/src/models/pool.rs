use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, ToSchema, Validate, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pool {
    #[validate(length(min = 1, max = 100))]
    pub id: String,
    
    #[validate(length(min = 1, max = 50))]
    pub token_a: String,
    
    #[validate(length(min = 1, max = 50))]
    pub token_b: String,
    
    #[validate(range(min = 0.0))]
    pub reserve_a: f64,
    
    #[validate(range(min = 0.0))]
    pub reserve_b: f64,
}

#[derive(Serialize, Deserialize, ToSchema, Validate, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PoolRequest {
    #[validate(length(min = 1, max = 50))]
    pub token_a: String,
    
    #[validate(length(min = 1, max = 50))]
    pub token_b: String,
}

#[derive(Serialize, Deserialize, ToSchema, Validate, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PoolResponse {
    pub pools: Vec<Pool>,
}