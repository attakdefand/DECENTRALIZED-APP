//! Pool card component
//!
//! This component displays information about a liquidity pool.

use yew::prelude::*;

/// Pool data structure
#[derive(Properties, PartialEq, Clone)]
pub struct PoolData {
    pub id: String,
    pub token_a: String,
    pub token_b: String,
    pub liquidity: f64,
    pub volume_24h: f64,
    pub apr: f64,
}

/// Pool card properties
#[derive(Properties, PartialEq)]
pub struct PoolCardProps {
    pub pool: PoolData,
}

/// Pool card component
#[function_component(PoolCard)]
pub fn pool_card(props: &PoolCardProps) -> Html {
    let PoolCardProps { pool } = props;

    html! {
        <div class="bg-white overflow-hidden shadow rounded-lg">
            <div class="px-4 py-5 sm:p-6">
                <div class="flex items-center">
                    <div class="flex-shrink-0 bg-indigo-500 rounded-md p-2">
                        <svg class="h-6 w-6 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
                        </svg>
                    </div>
                    <div class="ml-4">
                        <h3 class="text-lg font-medium text-gray-900">
                            {format!("{}-{}", pool.token_a, pool.token_b)}
                        </h3>
                        <p class="text-sm text-gray-500">
                            {"Pool ID: "}{&pool.id}
                        </p>
                    </div>
                </div>
                <div class="mt-4 grid grid-cols-2 gap-4">
                    <div>
                        <p class="text-sm font-medium text-gray-500">{"Liquidity"}</p>
                        <p class="text-lg font-semibold text-gray-900">
                            {format!("${:.2}", pool.liquidity)}
                        </p>
                    </div>
                    <div>
                        <p class="text-sm font-medium text-gray-500">{"Volume (24h)"}</p>
                        <p class="text-lg font-semibold text-gray-900">
                            {format!("${:.2}", pool.volume_24h)}
                        </p>
                    </div>
                    <div>
                        <p class="text-sm font-medium text-gray-500">{"APR"}</p>
                        <p class="text-lg font-semibold text-green-600">
                            {format!("{:.2}%", pool.apr)}
                        </p>
                    </div>
                </div>
                <div class="mt-6">
                    <button class="w-full bg-indigo-600 hover:bg-indigo-700 text-white font-medium py-2 px-4 rounded-md">
                        {"Trade"}
                    </button>
                </div>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_pool_data_creation() {
        let pool = PoolData {
            id: "test-pool".to_string(),
            token_a: "ETH".to_string(),
            token_b: "USDC".to_string(),
            liquidity: 1000000.0,
            volume_24h: 50000.0,
            apr: 15.0,
        };
        
        assert_eq!(pool.id, "test-pool");
        assert_eq!(pool.token_a, "ETH");
        assert_eq!(pool.token_b, "USDC");
        assert_eq!(pool.liquidity, 1000000.0);
        assert_eq!(pool.volume_24h, 50000.0);
        assert_eq!(pool.apr, 15.0);
    }
}