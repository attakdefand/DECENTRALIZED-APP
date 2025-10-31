//! Order form component with validation and security
//!
//! This component provides a secure order creation form with:
//! - Input validation
//! - Rate limiting
//! - Error handling
//! - Loading states

use yew::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use serde::{Serialize, Deserialize};

use crate::services::throttle::ThrottleService;

/// Order type enum
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum OrderType {
    Buy,
    Sell,
}

/// Order form data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderFormData {
    pub order_type: OrderType,
    pub token_pair: String,
    pub amount: f64,
    pub price: f64,
}

/// Order form props
#[derive(Properties, PartialEq)]
pub struct OrderFormProps {
    pub on_submit: Callback<OrderFormData>,
}

/// Order form component
#[function_component(OrderForm)]
pub fn order_form(props: &OrderFormProps) -> Html {
    let order_type = use_state(|| OrderType::Buy);
    let token_pair = use_state(|| String::from("ETH/USDC"));
    let amount = use_state(|| String::new());
    let price = use_state(|| String::new());
    let error = use_state(|| Option::<String>::None);
    let loading = use_state(|| false);
    
    // Security: Input validation
    let validate_amount = |input: &str| -> Result<f64, String> {
        if input.is_empty() {
            return Err("Amount is required".to_string());
        }
        
        match input.parse::<f64>() {
            Ok(val) if val > 0.0 && val <= 1_000_000.0 => Ok(val),
            Ok(val) if val <= 0.0 => Err("Amount must be greater than 0".to_string()),
            Ok(_) => Err("Amount exceeds maximum allowed".to_string()),
            Err(_) => Err("Invalid amount format".to_string()),
        }
    };
    
    let validate_price = |input: &str| -> Result<f64, String> {
        if input.is_empty() {
            return Err("Price is required".to_string());
        }
        
        match input.parse::<f64>() {
            Ok(val) if val > 0.0 && val <= 10_000_000.0 => Ok(val),
            Ok(val) if val <= 0.0 => Err("Price must be greater than 0".to_string()),
            Ok(_) => Err("Price exceeds maximum allowed".to_string()),
            Err(_) => Err("Invalid price format".to_string()),
        }
    };
    
    // Security: Sanitize numeric input (allow only digits and decimal point)
    let sanitize_numeric = |input: &str| -> String {
        input.chars()
            .filter(|c| c.is_numeric() || *c == '.')
            .take(20) // Prevent excessive input
            .collect()
    };
    
    let on_order_type_change = {
        let order_type = order_type.clone();
        Callback::from(move |e: Event| {
            let target: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
            let new_type = if target.value() == "buy" {
                OrderType::Buy
            } else {
                OrderType::Sell
            };
            order_type.set(new_type);
        })
    };
    
    let on_token_pair_change = {
        let token_pair = token_pair.clone();
        Callback::from(move |e: Event| {
            let target: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
            token_pair.set(target.value());
        })
    };
    
    let on_amount_change = {
        let amount = amount.clone();
        Callback::from(move |e: Event| {
            let target: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
            let sanitized = sanitize_numeric(&target.value());
            amount.set(sanitized);
        })
    };
    
    let on_price_change = {
        let price = price.clone();
        Callback::from(move |e: Event| {
            let target: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
            let sanitized = sanitize_numeric(&target.value());
            price.set(sanitized);
        })
    };
    
    let on_submit = {
        let order_type = order_type.clone();
        let token_pair = token_pair.clone();
        let amount = amount.clone();
        let price = price.clone();
        let error = error.clone();
        let loading = loading.clone();
        let on_submit_callback = props.on_submit.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            // Security: Rate limiting
            let mut throttle = ThrottleService::new();
            throttle.configure_limit("order_submission", 10, 60000.0); // 10 orders per minute
            
            if !throttle.is_allowed("order_submission") {
                error.set(Some("Too many order submissions. Please wait.".to_string()));
                return;
            }
            
            loading.set(true);
            error.set(None);
            
            // Validate inputs
            let amount_val = match validate_amount(&amount) {
                Ok(val) => val,
                Err(e) => {
                    error.set(Some(e));
                    loading.set(false);
                    return;
                }
            };
            
            let price_val = match validate_price(&price) {
                Ok(val) => val,
                Err(e) => {
                    error.set(Some(e));
                    loading.set(false);
                    return;
                }
            };
            
            // Security: Additional validation
            if token_pair.is_empty() {
                error.set(Some("Token pair is required".to_string()));
                loading.set(false);
                return;
            }
            
            // Create order data
            let order_data = OrderFormData {
                order_type: (*order_type).clone(),
                token_pair: (*token_pair).clone(),
                amount: amount_val,
                price: price_val,
            };
            
            // Submit order
            on_submit_callback.emit(order_data);
            
            // Reset form
            amount.set(String::new());
            price.set(String::new());
            loading.set(false);
        })
    };
    
    html! {
        <div class="bg-white shadow sm:rounded-lg">
            <div class="px-4 py-5 sm:p-6">
                <h3 class="text-lg font-medium leading-6 text-gray-900">{"Create Order"}</h3>
                <form class="mt-5 space-y-4" onsubmit={on_submit}>
                    // Order Type Selection
                    <div>
                        <label class="text-sm font-medium text-gray-700">{"Order Type"}</label>
                        <select
                            class="mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm rounded-md"
                            onchange={on_order_type_change}
                        >
                            <option value="buy" selected={*order_type == OrderType::Buy}>{"Buy"}</option>
                            <option value="sell" selected={*order_type == OrderType::Sell}>{"Sell"}</option>
                        </select>
                    </div>
                    
                    // Token Pair Selection
                    <div>
                        <label class="text-sm font-medium text-gray-700">{"Token Pair"}</label>
                        <select
                            class="mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm rounded-md"
                            onchange={on_token_pair_change}
                        >
                            <option value="ETH/USDC">{"ETH/USDC"}</option>
                            <option value="BTC/USDC">{"BTC/USDC"}</option>
                            <option value="ETH/DAI">{"ETH/DAI"}</option>
                        </select>
                    </div>
                    
                    // Amount Input
                    <div>
                        <label for="amount" class="block text-sm font-medium text-gray-700">
                            {"Amount"}
                        </label>
                        <input
                            type="text"
                            name="amount"
                            id="amount"
                            class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md"
                            placeholder="0.00"
                            value={(*amount).clone()}
                            onchange={on_amount_change}
                            required=true
                        />
                    </div>
                    
                    // Price Input
                    <div>
                        <label for="price" class="block text-sm font-medium text-gray-700">
                            {"Price"}
                        </label>
                        <input
                            type="text"
                            name="price"
                            id="price"
                            class="mt-1 focus:ring-indigo-500 focus:border-indigo-500 block w-full shadow-sm sm:text-sm border-gray-300 rounded-md"
                            placeholder="0.00"
                            value={(*price).clone()}
                            onchange={on_price_change}
                            required=true
                        />
                    </div>
                    
                    // Total Display
                    if !amount.is_empty() && !price.is_empty() {
                        if let (Ok(amt), Ok(prc)) = (amount.parse::<f64>(), price.parse::<f64>()) {
                            <div class="bg-gray-50 rounded-md p-3">
                                <div class="flex justify-between">
                                    <span class="text-sm text-gray-600">{"Total:"}</span>
                                    <span class="text-sm font-semibold text-gray-900">
                                        {format!("${:.2}", amt * prc)}
                                    </span>
                                </div>
                            </div>
                        }
                    }
                    
                    // Error Message
                    if let Some(err) = (*error).clone() {
                        <div class="rounded-md bg-red-50 p-4">
                            <div class="flex">
                                <div class="ml-3">
                                    <h3 class="text-sm font-medium text-red-800">{err}</h3>
                                </div>
                            </div>
                        </div>
                    }
                    
                    // Submit Button
                    <button
                        type="submit"
                        disabled={*loading}
                        class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
                    >
                        if *loading {
                            {"Processing..."}
                        } else if *order_type == OrderType::Buy {
                            {"Place Buy Order"}
                        } else {
                            {"Place Sell Order"}
                        }
                    </button>
                </form>
            </div>
        </div>
    }
}
