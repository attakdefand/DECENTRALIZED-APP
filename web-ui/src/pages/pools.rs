//! Pools page
//!
//! This page displays liquidity pools and allows users to interact with them.

use yew::prelude::*;

use crate::components::pool_card::{PoolCard, PoolData};

/// Pools page component
#[function_component(Pools)]
pub fn pools() -> Html {
    // Sample pool data - in a real app this would come from an API
    let pools = vec![
        PoolData {
            id: "1".to_string(),
            token_a: "ETH".to_string(),
            token_b: "USDC".to_string(),
            liquidity: 1250000.75,
            volume_24h: 45000.30,
            apr: 12.5,
        },
        PoolData {
            id: "2".to_string(),
            token_a: "BTC".to_string(),
            token_b: "USDC".to_string(),
            liquidity: 2500000.00,
            volume_24h: 87000.45,
            apr: 8.75,
        },
        PoolData {
            id: "3".to_string(),
            token_a: "ETH".to_string(),
            token_b: "DAI".to_string(),
            liquidity: 980000.25,
            volume_24h: 32000.15,
            apr: 15.2,
        },
    ];

    html! {
        <div>
            <div class="flex justify-between items-center">
                <h2 class="text-2xl font-bold text-gray-900">{"Liquidity Pools"}</h2>
                <button class="bg-indigo-600 hover:bg-indigo-700 text-white font-medium py-2 px-4 rounded-md">
                    {"Create Pool"}
                </button>
            </div>
            
            <div class="mt-6 grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
                {pools.iter().map(|pool| {
                    html! {
                        <PoolCard pool={pool.clone()} />
                    }
                }).collect::<Html>()}
            </div>
            
            <div class="mt-8">
                <h3 class="text-lg font-medium text-gray-900">{"How to Provide Liquidity"}</h3>
                <div class="mt-4 bg-white shadow overflow-hidden sm:rounded-lg">
                    <div class="px-4 py-5 sm:p-6">
                        <ol class="list-decimal list-inside space-y-2">
                            <li class="text-gray-600">{"Select a pool or create a new one"}</li>
                            <li class="text-gray-600">{"Deposit an equivalent value of both tokens"}</li>
                            <li class="text-gray-600">{"Receive LP tokens representing your share"}</li>
                            <li class="text-gray-600">{"Earn trading fees proportional to your share"}</li>
                        </ol>
                    </div>
                </div>
            </div>
        </div>
    }
}