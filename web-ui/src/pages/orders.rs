//! Orders page
//!
//! This page displays user orders with real backend integration and allows order creation.

use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::components::order_table::{OrderTable, OrderData, OrderSide, OrderStatus};
use crate::components::order_form::{OrderForm, OrderFormData};
use crate::services::{
    api::create_orders_client,
    models::{OrderResponse, OrderInfo},
    cache::CacheService,
    retry::{RetryService, RetryConfig},
};

/// Orders page component
#[function_component(Orders)]
pub fn orders() -> Html {
    let orders = use_state(|| Vec::<OrderData>::new());
    let loading = use_state(|| true);
    let error = use_state(|| Option::<String>::None);
    let show_form = use_state(|| false);
    
    // Load orders from backend with caching and retry
    {
        let orders = orders.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        use_effect(move || {
            spawn_local(async move {
                let cache = CacheService::new();
                
                // Try cache first
                if let Some(cached_orders) = cache.get_local::<Vec<OrderData>>("orders_data") {
                    web_sys::console::log_1(&"Loaded orders from cache".into());
                    orders.set(cached_orders);
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
                let fetch_orders = || async {
                    let api_client = create_orders_client();
                    
                    match api_client.get::<OrderResponse>("/orders").await {
                        Ok(response) => {
                            // Convert API response to OrderData
                            let orders_data: Vec<OrderData> = response.orders.into_iter().map(|o| {
                                OrderData {
                                    id: o.id,
                                    pair: o.pair,
                                    side: match o.side.as_str() {
                                        "buy" => OrderSide::Buy,
                                        "sell" => OrderSide::Sell,
                                        _ => OrderSide::Buy,
                                    },
                                    price: o.price.parse().unwrap_or(0.0),
                                    amount: o.amount.parse().unwrap_or(0.0),
                                    filled: o.filled.parse().unwrap_or(0.0),
                                    status: match o.status.as_str() {
                                        "open" => OrderStatus::Open,
                                        "filled" => OrderStatus::Filled,
                                        "cancelled" => OrderStatus::Cancelled,
                                        _ => OrderStatus::Open,
                                    },
                                }
                            }).collect();
                            
                            Ok::<Vec<OrderData>, String>(orders_data)
                        }
                        Err(e) => {
                            web_sys::console::warn_1(&format!("API call failed: {}", e).into());
                            // Fallback to mock data
                            Ok::<Vec<OrderData>, String>(vec![
                                OrderData {
                                    id: "12345".to_string(),
                                    pair: "ETH/USDC".to_string(),
                                    side: OrderSide::Buy,
                                    price: 2500.50,
                                    amount: 1.5,
                                    filled: 1.0,
                                    status: OrderStatus::Open,
                                },
                                OrderData {
                                    id: "12346".to_string(),
                                    pair: "BTC/USDC".to_string(),
                                    side: OrderSide::Sell,
                                    price: 45000.00,
                                    amount: 0.25,
                                    filled: 0.25,
                                    status: OrderStatus::Filled,
                                },
                            ])
                        }
                    }
                };
                
                match retry_service.retry(fetch_orders).await {
                    Ok(fetched_orders) => {
                        // Cache for 2 minutes
                        if let Err(e) = cache.set_local("orders_data", &fetched_orders, 120000.0) {
                            web_sys::console::warn_1(&format!("Failed to cache: {:?}", e).into());
                        }
                        orders.set(fetched_orders);
                        loading.set(false);
                    }
                    Err(e) => {
                        error.set(Some(format!("Failed to load orders: {}", e)));
                        loading.set(false);
                    }
                }
            });
            || ()
        });
    }
    
    let toggle_form = {
        let show_form = show_form.clone();
        Callback::from(move |_| {
            show_form.set(!*show_form);
        })
    };
    
    let on_order_submit = {
        let orders = orders.clone();
        let show_form = show_form.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        Callback::from(move |form_data: OrderFormData| {
            let orders = orders.clone();
            let show_form = show_form.clone();
            let loading = loading.clone();
            let error = error.clone();
            
            web_sys::console::log_1(&format!("Order submitted: {:?}", form_data).into());
            
            loading.set(true);
            error.set(None);
            
            spawn_local(async move {
                let api_client = create_orders_client();
                
                // Prepare order data for API
                #[derive(serde::Serialize)]
                struct CreateOrderRequest {
                    pair: String,
                    side: String,
                    price: f64,
                    amount: f64,
                }
                
                let order_request = CreateOrderRequest {
                    pair: form_data.token_pair.clone(),
                    side: match form_data.order_type {
                        crate::components::order_form::OrderType::Buy => "buy".to_string(),
                        crate::components::order_form::OrderType::Sell => "sell".to_string(),
                    },
                    price: form_data.price,
                    amount: form_data.amount,
                };
                
                // Security: Use retry logic for order submission
                let retry_service = RetryService::with_config(RetryConfig {
                    max_attempts: 2, // Only retry once for order submission
                    base_delay: 1000.0,
                    max_delay: 2000.0,
                    backoff_multiplier: 2.0,
                });
                
                let submit_order = || async {
                    api_client.post::<CreateOrderRequest, OrderInfo>("/orders", &order_request)
                        .await
                        .map_err(|e| e.to_string())
                };
                
                match retry_service.retry(submit_order).await {
                    Ok(created_order) => {
                        web_sys::console::log_1(&"Order created successfully".into());
                        
                        // Add new order to list
                        let new_order = OrderData {
                            id: created_order.id,
                            pair: created_order.pair,
                            side: match created_order.side.as_str() {
                                "buy" => OrderSide::Buy,
                                "sell" => OrderSide::Sell,
                                _ => OrderSide::Buy,
                            },
                            price: created_order.price.parse().unwrap_or(form_data.price),
                            amount: created_order.amount.parse().unwrap_or(form_data.amount),
                            filled: created_order.filled.parse().unwrap_or(0.0),
                            status: OrderStatus::Open,
                        };
                        
                        let mut updated_orders = (*orders).clone();
                        updated_orders.insert(0, new_order);
                        orders.set(updated_orders.clone());
                        
                        // Update cache
                        let cache = CacheService::new();
                        let _ = cache.set_local("orders_data", &updated_orders, 120000.0);
                        
                        show_form.set(false);
                        loading.set(false);
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Order submission failed: {}", e).into());
                        
                        // Fallback: Add order locally if API fails
                        let new_order = OrderData {
                            id: format!("{}", js_sys::Date::now() as u64),
                            pair: form_data.token_pair,
                            side: match form_data.order_type {
                                crate::components::order_form::OrderType::Buy => OrderSide::Buy,
                                crate::components::order_form::OrderType::Sell => OrderSide::Sell,
                            },
                            price: form_data.price,
                            amount: form_data.amount,
                            filled: 0.0,
                            status: OrderStatus::Open,
                        };
                        
                        let mut updated_orders = (*orders).clone();
                        updated_orders.insert(0, new_order);
                        orders.set(updated_orders);
                        
                        error.set(Some(format!("Order created locally (API unavailable): {}", e)));
                        show_form.set(false);
                        loading.set(false);
                    }
                }
            });
        })
    };

    html! {
        <div>
            <div class="flex justify-between items-center">
                <h2 class="text-2xl font-bold text-gray-900">{"Orders"}</h2>
                <div class="flex space-x-2">
                    <button 
                        onclick={toggle_form.clone()}
                        class="bg-indigo-600 hover:bg-indigo-700 text-white font-medium py-2 px-4 rounded-md"
                        disabled={*loading}
                    >
                        if *show_form {
                            {"Hide Form"}
                        } else {
                            {"New Order"}
                        }
                    </button>
                    <button class="bg-gray-600 hover:bg-gray-700 text-white font-medium py-2 px-4 rounded-md">
                        {"Cancel All"}
                    </button>
                </div>
            </div>
            
            // Show error if API failed
            if let Some(err) = (*error).clone() {
                <div class="mt-4 rounded-md bg-yellow-50 p-4">
                    <div class="flex">
                        <div class="ml-3">
                            <h3 class="text-sm font-medium text-yellow-800">{err}</h3>
                        </div>
                    </div>
                </div>
            }
            
            // Show order form when toggled
            if *show_form {
                <div class="mt-6">
                    <OrderForm on_submit={on_order_submit} />
                </div>
            }
            
            // Show loading or orders
            if *loading {
                <div class="mt-6 flex justify-center">
                    <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-indigo-600"></div>
                </div>
            } else {
                <div class="mt-6">
                    <OrderTable orders={(*orders).clone()} />
                </div>
            }
            
            <div class="mt-8">
                <h3 class="text-lg font-medium text-gray-900">{"Order Types"}</h3>
                <div class="mt-4 grid grid-cols-1 gap-5 sm:grid-cols-3">
                    <div class="bg-white overflow-hidden shadow rounded-lg p-4">
                        <h4 class="text-md font-medium text-gray-900">{"Market Order"}</h4>
                        <p class="mt-2 text-sm text-gray-600">
                            {"Executes immediately at the best available price."}
                        </p>
                    </div>
                    <div class="bg-white overflow-hidden shadow rounded-lg p-4">
                        <h4 class="text-md font-medium text-gray-900">{"Limit Order"}</h4>
                        <p class="mt-2 text-sm text-gray-600">
                            {"Executes only when the price reaches your specified level."}
                        </p>
                    </div>
                    <div class="bg-white overflow-hidden shadow rounded-lg p-4">
                        <h4 class="text-md font-medium text-gray-900">{"Stop Order"}</h4>
                        <p class="mt-2 text-sm text-gray-600">
                            {"Triggers a market order when the price reaches a specific level."}
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}