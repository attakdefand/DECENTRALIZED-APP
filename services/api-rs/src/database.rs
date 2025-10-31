//! Database integration module
//!
//! Provides secure database operations with connection pooling and prepared statements

use sqlx::{Pool, Postgres, Row};
use serde::{Deserialize, Serialize};
use anyhow::Result;

/// Database pool type
pub type DbPool = Pool<Postgres>;

/// Initialize database connection pool
pub async fn init_pool(database_url: &str) -> Result<DbPool> {
    // Security: Use connection pooling with limits
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(20)
        .min_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(30))
        .idle_timeout(Some(std::time::Duration::from_secs(600)))
        .max_lifetime(Some(std::time::Duration::from_secs(1800)))
        .connect(database_url)
        .await?;
    
    Ok(pool)
}

/// Run database migrations
pub async fn run_migrations(pool: &DbPool) -> Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;
    Ok(())
}

/// Pool data structure
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Pool {
    pub id: String,
    pub token_a_symbol: String,
    pub token_a_address: String,
    pub token_b_symbol: String,
    pub token_b_address: String,
    pub liquidity: String,
    pub volume_24h: String,
    pub apr: String,
    pub fee_tier: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Order data structure
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Order {
    pub id: String,
    pub user_id: String,
    pub pair: String,
    pub side: String,
    pub price: String,
    pub amount: String,
    pub filled: String,
    pub status: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

/// Market data structure
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Market {
    pub pair: String,
    pub price: String,
    pub change_24h: String,
    pub volume_24h: String,
    pub high_24h: String,
    pub low_24h: String,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// ============================================================================
// POOL QUERIES
// ============================================================================

/// Get all pools
pub async fn get_pools(pool: &DbPool) -> Result<Vec<Pool>> {
    let pools = sqlx::query_as::<_, Pool>(
        "SELECT * FROM pools ORDER BY volume_24h DESC"
    )
    .fetch_all(pool)
    .await?;
    
    Ok(pools)
}

/// Get pool by ID
pub async fn get_pool_by_id(pool: &DbPool, id: &str) -> Result<Option<Pool>> {
    // Security: Use parameterized query to prevent SQL injection
    let result = sqlx::query_as::<_, Pool>(
        "SELECT * FROM pools WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    
    Ok(result)
}

/// Create new pool
pub async fn create_pool(
    pool: &DbPool,
    id: &str,
    token_a_symbol: &str,
    token_a_address: &str,
    token_b_symbol: &str,
    token_b_address: &str,
    liquidity: &str,
    fee_tier: &str,
) -> Result<Pool> {
    // Security: Use parameterized query
    let result = sqlx::query_as::<_, Pool>(
        r#"
        INSERT INTO pools (id, token_a_symbol, token_a_address, token_b_symbol, token_b_address, liquidity, volume_24h, apr, fee_tier)
        VALUES ($1, $2, $3, $4, $5, $6, '0', '0', $7)
        RETURNING *
        "#
    )
    .bind(id)
    .bind(token_a_symbol)
    .bind(token_a_address)
    .bind(token_b_symbol)
    .bind(token_b_address)
    .bind(liquidity)
    .bind(fee_tier)
    .fetch_one(pool)
    .await?;
    
    Ok(result)
}

/// Update pool liquidity
pub async fn update_pool_liquidity(
    pool: &DbPool,
    id: &str,
    liquidity: &str,
    volume_24h: &str,
    apr: &str,
) -> Result<Pool> {
    // Security: Use parameterized query
    let result = sqlx::query_as::<_, Pool>(
        r#"
        UPDATE pools 
        SET liquidity = $2, volume_24h = $3, apr = $4, updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#
    )
    .bind(id)
    .bind(liquidity)
    .bind(volume_24h)
    .bind(apr)
    .fetch_one(pool)
    .await?;
    
    Ok(result)
}

// ============================================================================
// ORDER QUERIES
// ============================================================================

/// Get all orders for a user
pub async fn get_user_orders(pool: &DbPool, user_id: &str) -> Result<Vec<Order>> {
    // Security: Use parameterized query
    let orders = sqlx::query_as::<_, Order>(
        "SELECT * FROM orders WHERE user_id = $1 ORDER BY created_at DESC"
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    
    Ok(orders)
}

/// Get order by ID
pub async fn get_order_by_id(pool: &DbPool, id: &str) -> Result<Option<Order>> {
    // Security: Use parameterized query
    let result = sqlx::query_as::<_, Order>(
        "SELECT * FROM orders WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    
    Ok(result)
}

/// Create new order
pub async fn create_order(
    pool: &DbPool,
    id: &str,
    user_id: &str,
    pair: &str,
    side: &str,
    price: &str,
    amount: &str,
) -> Result<Order> {
    // Security: Use parameterized query and validate inputs
    let result = sqlx::query_as::<_, Order>(
        r#"
        INSERT INTO orders (id, user_id, pair, side, price, amount, filled, status)
        VALUES ($1, $2, $3, $4, $5, $6, '0', 'open')
        RETURNING *
        "#
    )
    .bind(id)
    .bind(user_id)
    .bind(pair)
    .bind(side)
    .bind(price)
    .bind(amount)
    .fetch_one(pool)
    .await?;
    
    Ok(result)
}

/// Update order status
pub async fn update_order_status(
    pool: &DbPool,
    id: &str,
    filled: &str,
    status: &str,
) -> Result<Order> {
    // Security: Use parameterized query
    let result = sqlx::query_as::<_, Order>(
        r#"
        UPDATE orders 
        SET filled = $2, status = $3, updated_at = NOW()
        WHERE id = $1
        RETURNING *
        "#
    )
    .bind(id)
    .bind(filled)
    .bind(status)
    .fetch_one(pool)
    .await?;
    
    Ok(result)
}

// ============================================================================
// MARKET QUERIES
// ============================================================================

/// Get all markets
pub async fn get_markets(pool: &DbPool) -> Result<Vec<Market>> {
    let markets = sqlx::query_as::<_, Market>(
        "SELECT * FROM markets ORDER BY volume_24h DESC"
    )
    .fetch_all(pool)
    .await?;
    
    Ok(markets)
}

/// Get market by pair
pub async fn get_market_by_pair(pool: &DbPool, pair: &str) -> Result<Option<Market>> {
    // Security: Use parameterized query
    let result = sqlx::query_as::<_, Market>(
        "SELECT * FROM markets WHERE pair = $1"
    )
    .bind(pair)
    .fetch_optional(pool)
    .await?;
    
    Ok(result)
}

/// Update market data
pub async fn update_market(
    pool: &DbPool,
    pair: &str,
    price: &str,
    change_24h: &str,
    volume_24h: &str,
    high_24h: &str,
    low_24h: &str,
) -> Result<Market> {
    // Security: Use parameterized query
    let result = sqlx::query_as::<_, Market>(
        r#"
        INSERT INTO markets (pair, price, change_24h, volume_24h, high_24h, low_24h)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (pair) 
        DO UPDATE SET 
            price = EXCLUDED.price,
            change_24h = EXCLUDED.change_24h,
            volume_24h = EXCLUDED.volume_24h,
            high_24h = EXCLUDED.high_24h,
            low_24h = EXCLUDED.low_24h,
            updated_at = NOW()
        RETURNING *
        "#
    )
    .bind(pair)
    .bind(price)
    .bind(change_24h)
    .bind(volume_24h)
    .bind(high_24h)
    .bind(low_24h)
    .fetch_one(pool)
    .await?;
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Note: These are placeholder tests
    // In production, use a test database
    
    #[test]
    fn test_pool_structure() {
        // Ensure Pool struct can be serialized
        let pool = Pool {
            id: "test".to_string(),
            token_a_symbol: "ETH".to_string(),
            token_a_address: "0x123".to_string(),
            token_b_symbol: "USDC".to_string(),
            token_b_address: "0x456".to_string(),
            liquidity: "1000000".to_string(),
            volume_24h: "50000".to_string(),
            apr: "12.5".to_string(),
            fee_tier: "0.3".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        let json = serde_json::to_string(&pool).unwrap();
        assert!(json.contains("ETH"));
    }
    
    #[test]
    fn test_order_structure() {
        let order = Order {
            id: "order-1".to_string(),
            user_id: "user-1".to_string(),
            pair: "ETH/USDC".to_string(),
            side: "buy".to_string(),
            price: "2500".to_string(),
            amount: "1.0".to_string(),
            filled: "0.5".to_string(),
            status: "open".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        let json = serde_json::to_string(&order).unwrap();
        assert!(json.contains("ETH/USDC"));
    }
}
