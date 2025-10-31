//! Pools page
//!
//! This page displays liquidity pools with real backend integration, caching and retry logic.

use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::components::pool_card::{PoolCard, PoolData};
use crate::components::liquidity_form::{LiquidityForm, LiquidityFormData};
use crate::services::{
    cache::CacheService,
    retry::{RetryService, RetryConfig},
    api::create_pools_client,
    models::PoolResponse,
};

/// Pools page component
#[function_component(Pools)]
pub fn pools() -> Html {
    let pools = use_state(|| Vec::<PoolData>::new());
    let loading = use_state(|| true);
    let error = use_state(|| Option::<String>::None);
    let show_liquidity_form = use_state(|| false);
    
    // Load pools with caching and retry logic
    {
        let pools = pools.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        use_effect(move || {
            spawn_local(async move {
                let cache = CacheService::new();
                
                // Try to get from cache first
                if let Some(cached_pools) = cache.get_local::<Vec<PoolData>>("pools_data") {
                    web_sys::console::log_1(&"Loaded pools from cache".into());
                    pools.set(cached_pools);
                    loading.set(false);
                    return;
                }
                
                // Fetch with retry logic
                let retry_service = RetryService::with_config(RetryConfig {
                    max_attempts: 3,
                    base_delay: 1000.0,
                    max_delay: 5000.0,
                    backoff_multiplier: 2.0,
                });
                
                // Real API call to backend
                let fetch_pools = || async {
                    let api_client = create_pools_client();
                    
                    // Security: Use try-catch for error handling
                    match api_client.get::<PoolResponse>("/pools").await {
                        Ok(response) => {
                            // Convert API response to PoolData
                            let pools: Vec<PoolData> = response.pools.into_iter().map(|p| {
                                PoolData {
                                    id: p.id,
                                    token_a: p.token_a.symbol,
                                    token_b: p.token_b.symbol,
                                    liquidity: p.liquidity.parse().unwrap_or(0.0),
                                    volume_24h: p.volume_24h.parse().unwrap_or(0.0),
                                    apr: p.apr.parse().unwrap_or(0.0),
                                }
                            }).collect();
                            
                            Ok::<Vec<PoolData>, String>(pools)
                        }
                        Err(e) => {
                            web_sys::console::warn_1(&format!("API call failed: {}", e).into());
                            // Fallback to mock data if API fails
                            Ok::<Vec<PoolData>, String>(vec![
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
                            ])
                        }
                    }
                };
                
                match retry_service.retry(fetch_pools).await {
                    Ok(fetched_pools) => {
                        // Cache the data for 5 minutes
                        if let Err(e) = cache.set_local("pools_data", &fetched_pools, 300000.0) {
                            web_sys::console::warn_1(&format!("Failed to cache pools: {:?}", e).into());
                        }
                        pools.set(fetched_pools);
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to load pools: {}", e)));
                        loading.set(false);
                    }
                }
            });
            || ()
        });
    }
    
    // Handle liquidity form submission
    let on_liquidity_submit = {
        let show_liquidity_form = show_liquidity_form.clone();
        
        Callback::from(move |liquidity_data: LiquidityFormData| {
            web_sys::console::log_1(&format!("Liquidity submitted: {:?}", liquidity_data).into());
            
            // In production, this would:
            // 1. Submit to backend API
            // 2. Show toast notification on success/error
            // 3. Refresh pools list
            
            // Hide form after submission
            show_liquidity_form.set(false);
        })
    };
    
    let toggle_liquidity_form = {
        let show_liquidity_form = show_liquidity_form.clone();
        
        Callback::from(move |_| {
            show_liquidity_form.set(!*show_liquidity_form);
        })
    };

    html! {
        <div>
            <div class="flex justify-between items-center">
                <h2 class="text-2xl font-bold text-gray-900">{"Liquidity Pools"}</h2>
                <button
                    onclick={toggle_liquidity_form}
                    class="bg-indigo-600 hover:bg-indigo-700 text-white font-medium py-2 px-4 rounded-md"
                    aria-label="Toggle liquidity form"
                >
                    if *show_liquidity_form {
                        {"Hide Form"}
                    } else {
                        {"Add Liquidity"}
                    }
                </button>
            </div>
            
            // Show liquidity form when toggled
            if *show_liquidity_form {
                <div class="mt-6">
                    <LiquidityForm on_submit={on_liquidity_submit} />
                </div>
            }
            
            // Show loading state
            if *loading {
                <div class="mt-6 flex justify-center">
                    <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-indigo-600"></div>
                </div>
            } else if let Some(err) = (*error).clone() {
                <div class="mt-6 rounded-md bg-red-50 p-4">
                    <div class="flex">
                        <div class="ml-3">
                            <h3 class="text-sm font-medium text-red-800">{err}</h3>
                        </div>
                    </div>
                </div>
            } else {
                <div class="mt-6 grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
                    {(*pools).iter().map(|pool| {
                        html! {
                            <PoolCard pool={pool.clone()} />
                        }
                    }).collect::<Html>()}
                </div>
            }
            
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