//! Markets page
//!
//! This page displays market data with real-time WebSocket updates.

use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebSocket, MessageEvent};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;

use crate::components::market_chart::{MarketChart, ChartDataPoint};
use crate::services::throttle::ThrottleService;

/// Market price update from WebSocket
#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct PriceUpdate {
    symbol: String,
    price: f64,
    change_24h: f64,
    volume_24h: f64,
}

/// Market data state
#[derive(Clone, PartialEq)]
struct MarketData {
    symbol: String,
    price: f64,
    change_24h: f64,
    volume_24h: f64,
}

impl Default for MarketData {
    fn default() -> Self {
        Self {
            symbol: String::new(),
            price: 0.0,
            change_24h: 0.0,
            volume_24h: 0.0,
        }
    }
}

/// Markets page component
#[function_component(Markets)]
pub fn markets() -> Html {
    let eth_price = use_state(|| MarketData {
        symbol: "ETH/USDC".to_string(),
        price: 2530.0,
        change_24h: 2.5,
        volume_24h: 45000000.0,
    });
    
    let btc_price = use_state(|| MarketData {
        symbol: "BTC/USDC".to_string(),
        price: 45300.0,
        change_24h: -1.2,
        volume_24h: 87000000.0,
    });
    
    let ws_connected = use_state(|| false);
    let ws_error = use_state(|| Option::<String>::None);
    
    // WebSocket connection with throttling for updates
    {
        let eth_price = eth_price.clone();
        let btc_price = btc_price.clone();
        let ws_connected = ws_connected.clone();
        let ws_error = ws_error.clone();
        
        use_effect(move || {
            // Security: Rate limit WebSocket message processing
            let mut throttle = ThrottleService::new();
            throttle.configure_limit("ws_updates", 60, 1000.0); // Max 60 updates per second
            
            // Simulate WebSocket connection (in production, use real WebSocket endpoint)
            let ws_url = "wss://api.example.com/markets";
            
            // For demo purposes, we'll simulate updates with setTimeout
            // In production, replace this with actual WebSocket connection:
            // let ws = WebSocket::new(ws_url).ok();
            
            web_sys::console::log_1(&format!("Would connect to WebSocket: {}", ws_url).into());
            ws_connected.set(true);
            
            // Simulate real-time price updates
            let interval_closure = Closure::wrap(Box::new(move || {
                // Security: Check throttle before processing update
                if !throttle.is_allowed("ws_updates") {
                    web_sys::console::warn_1(&"WebSocket updates throttled".into());
                    return;
                }
                
                // Simulate price fluctuation
                let eth_change = (js_sys::Math::random() - 0.5) * 10.0;
                eth_price.set(MarketData {
                    symbol: "ETH/USDC".to_string(),
                    price: 2530.0 + eth_change,
                    change_24h: 2.5 + (js_sys::Math::random() - 0.5),
                    volume_24h: 45000000.0,
                });
                
                let btc_change = (js_sys::Math::random() - 0.5) * 100.0;
                btc_price.set(MarketData {
                    symbol: "BTC/USDC".to_string(),
                    price: 45300.0 + btc_change,
                    change_24h: -1.2 + (js_sys::Math::random() - 0.5),
                    volume_24h: 87000000.0,
                });
            }) as Box<dyn FnMut()>);
            
            // Update every 2 seconds (in production, this would be WebSocket messages)
            let interval_id = web_sys::window()
                .unwrap()
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    interval_closure.as_ref().unchecked_ref(),
                    2000,
                );
            
            interval_closure.forget();
            
            move || {
                // Cleanup: close WebSocket on unmount
                if let Ok(id) = interval_id {
                    web_sys::window().unwrap().clear_interval_with_handle(id);
                }
                ws_connected.set(false);
            }
        });
    }
    // Sample chart data - in a real app this would come from an API
    let eth_chart_data = vec![
        ChartDataPoint { timestamp: 1620000000, price: 2400.0 },
        ChartDataPoint { timestamp: 1620086400, price: 2450.5 },
        ChartDataPoint { timestamp: 1620172800, price: 2500.0 },
        ChartDataPoint { timestamp: 1620259200, price: 2480.5 },
        ChartDataPoint { timestamp: 1620345600, price: 2520.0 },
        ChartDataPoint { timestamp: 1620432000, price: 2550.5 },
        ChartDataPoint { timestamp: 1620518400, price: 2530.0 },
    ];

    let btc_chart_data = vec![
        ChartDataPoint { timestamp: 1620000000, price: 44000.0 },
        ChartDataPoint { timestamp: 1620086400, price: 44500.5 },
        ChartDataPoint { timestamp: 1620172800, price: 45000.0 },
        ChartDataPoint { timestamp: 1620259200, price: 44800.5 },
        ChartDataPoint { timestamp: 1620345600, price: 45200.0 },
        ChartDataPoint { timestamp: 1620432000, price: 45500.5 },
        ChartDataPoint { timestamp: 1620518400, price: 45300.0 },
    ];

    html! {
        <div>
            <div class="flex justify-between items-center">
                <h2 class="text-2xl font-bold text-gray-900">{"Markets"}</h2>
                <div class="flex items-center space-x-2">
                    if *ws_connected {
                        <div class="flex items-center text-green-600">
                            <div class="w-2 h-2 bg-green-600 rounded-full animate-pulse mr-2"></div>
                            <span class="text-sm">{"Live"}</span>
                        </div>
                    } else {
                        <div class="flex items-center text-gray-400">
                            <div class="w-2 h-2 bg-gray-400 rounded-full mr-2"></div>
                            <span class="text-sm">{"Disconnected"}</span>
                        </div>
                    }
                </div>
            </div>
            
            if let Some(err) = (*ws_error).clone() {
                <div class="mt-4 rounded-md bg-yellow-50 p-4">
                    <div class="flex">
                        <div class="ml-3">
                            <h3 class="text-sm font-medium text-yellow-800">
                                {"WebSocket connection issue: "}{err}
                            </h3>
                        </div>
                    </div>
                </div>
            }
            
            <div class="mt-6 grid grid-cols-1 gap-6 lg:grid-cols-2">
                <MarketChart data={eth_chart_data} symbol={"ETH/USDC".to_string()} />
                <MarketChart data={btc_chart_data} symbol={"BTC/USDC".to_string()} />
            </div>
            
            <div class="mt-8">
                <h3 class="text-lg font-medium text-gray-900">{"Market Overview"}</h3>
                <div class="mt-4 bg-white shadow overflow-hidden sm:rounded-lg">
                    <div class="px-4 py-5 sm:p-6">
                        <div class="flex flex-col">
                            <div class="-my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
                                <div class="py-2 align-middle inline-block min-w-full sm:px-6 lg:px-8">
                                    <div class="shadow overflow-hidden border-b border-gray-200 sm:rounded-lg">
                                        <table class="min-w-full divide-y divide-gray-200">
                                            <thead class="bg-gray-50">
                                                <tr>
                                                    <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                                        {"Pair"}
                                                    </th>
                                                    <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                                        {"Price"}
                                                    </th>
                                                    <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                                        {"24h Change"}
                                                    </th>
                                                    <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                                        {"24h Volume"}
                                                    </th>
                                                </tr>
                                            </thead>
                                            <tbody class="bg-white divide-y divide-gray-200">
                                                <tr>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                                                        {&eth_price.symbol}
                                                    </td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                        {format!("${:.2}", eth_price.price)}
                                                    </td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm">
                                                        if eth_price.change_24h >= 0.0 {
                                                            <span class="text-green-600">{format!("+{:.2}%", eth_price.change_24h)}</span>
                                                        } else {
                                                            <span class="text-red-600">{format!("{:.2}%", eth_price.change_24h)}</span>
                                                        }
                                                    </td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                        {format!("${:.0}", eth_price.volume_24h)}
                                                    </td>
                                                </tr>
                                                <tr>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                                                        {&btc_price.symbol}
                                                    </td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                        {format!("${:.2}", btc_price.price)}
                                                    </td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm">
                                                        if btc_price.change_24h >= 0.0 {
                                                            <span class="text-green-600">{format!("+{:.2}%", btc_price.change_24h)}</span>
                                                        } else {
                                                            <span class="text-red-600">{format!("{:.2}%", btc_price.change_24h)}</span>
                                                        }
                                                    </td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                        {format!("${:.0}", btc_price.volume_24h)}
                                                    </td>
                                                </tr>
                                            </tbody>
                                        </table>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}