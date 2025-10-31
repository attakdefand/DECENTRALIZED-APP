//! Liquidity form component with validation and security
//!
//! This component provides a secure liquidity addition form with:
//! - Input validation for both token amounts
//! - Price impact calculation
//! - Slippage protection
//! - Rate limiting
//! - Error handling

use yew::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use serde::{Serialize, Deserialize};

use crate::services::throttle::ThrottleService;

/// Liquidity form data
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LiquidityFormData {
    pub pool_id: String,
    pub token_a_amount: f64,
    pub token_b_amount: f64,
    pub slippage_tolerance: f64,
}

/// Liquidity form props
#[derive(Properties, PartialEq)]
pub struct LiquidityFormProps {
    pub on_submit: Callback<LiquidityFormData>,
}

/// Liquidity form component
#[function_component(LiquidityForm)]
pub fn liquidity_form(props: &LiquidityFormProps) -> Html {
    let pool_id = use_state(|| String::from("ETH/USDC"));
    let token_a_amount = use_state(|| String::new());
    let token_b_amount = use_state(|| String::new());
    let slippage = use_state(|| String::from("0.5"));
    let error = use_state(|| Option::<String>::None);
    let loading = use_state(|| false);
    let price_impact = use_state(|| 0.0);
    
    // Security: Input validation
    let validate_amount = |input: &str| -> Result<f64, String> {
        if input.is_empty() {
            return Err("Amount is required".to_string());
        }
        
        match input.parse::<f64>() {
            Ok(val) if val > 0.0 && val <= 10_000_000.0 => Ok(val),
            Ok(val) if val <= 0.0 => Err("Amount must be greater than 0".to_string()),
            Ok(_) => Err("Amount exceeds maximum allowed".to_string()),
            Err(_) => Err("Invalid amount format".to_string()),
        }
    };
    
    let validate_slippage = |input: &str| -> Result<f64, String> {
        match input.parse::<f64>() {
            Ok(val) if val >= 0.0 && val <= 50.0 => Ok(val),
            Ok(_) => Err("Slippage must be between 0% and 50%".to_string()),
            Err(_) => Err("Invalid slippage format".to_string()),
        }
    };
    
    // Security: Sanitize numeric input
    let sanitize_numeric = |input: &str| -> String {
        input.chars()
            .filter(|c| c.is_numeric() || *c == '.')
            .take(20)
            .collect()
    };
    
    // Calculate price impact when amounts change
    let calculate_price_impact = {
        let token_a_amount = token_a_amount.clone();
        let token_b_amount = token_b_amount.clone();
        let price_impact = price_impact.clone();
        
        move || {
            if let (Ok(a), Ok(b)) = (
                token_a_amount.parse::<f64>(),
                token_b_amount.parse::<f64>()
            ) {
                // Simplified price impact calculation
                // In production, this would call the backend
                let impact = ((a / (a + 1000000.0)) * 100.0).min(50.0);
                price_impact.set(impact);
            }
        }
    };
    
    let on_pool_change = {
        let pool_id = pool_id.clone();
        Callback::from(move |e: Event| {
            let target: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
            pool_id.set(target.value());
        })
    };
    
    let on_token_a_change = {
        let token_a_amount = token_a_amount.clone();
        let calc = calculate_price_impact.clone();
        Callback::from(move |e: Event| {
            let target: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
            let sanitized = sanitize_numeric(&target.value());
            token_a_amount.set(sanitized);
            calc();
        })
    };
    
    let on_token_b_change = {
        let token_b_amount = token_b_amount.clone();
        let calc = calculate_price_impact.clone();
        Callback::from(move |e: Event| {
            let target: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
            let sanitized = sanitize_numeric(&target.value());
            token_b_amount.set(sanitized);
            calc();
        })
    };
    
    let on_slippage_change = {
        let slippage = slippage.clone();
        Callback::from(move |e: Event| {
            let target: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
            let sanitized = sanitize_numeric(&target.value());
            slippage.set(sanitized);
        })
    };
    
    let on_submit = {
        let pool_id = pool_id.clone();
        let token_a_amount = token_a_amount.clone();
        let token_b_amount = token_b_amount.clone();
        let slippage = slippage.clone();
        let error = error.clone();
        let loading = loading.clone();
        let on_submit_callback = props.on_submit.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            // Security: Rate limiting
            let mut throttle = ThrottleService::new();
            throttle.configure_limit("liquidity_add", 5, 60000.0); // 5 additions per minute
            
            if !throttle.is_allowed("liquidity_add") {
                error.set(Some("Too many liquidity additions. Please wait.".to_string()));
                return;
            }
            
            loading.set(true);
            error.set(None);
            
            // Validate inputs
            let token_a_val = match validate_amount(&token_a_amount) {
                Ok(val) => val,
                Err(e) => {
                    error.set(Some(format!("Token A: {}", e)));
                    loading.set(false);
                    return;
                }
            };
            
            let token_b_val = match validate_amount(&token_b_amount) {
                Ok(val) => val,
                Err(e) => {
                    error.set(Some(format!("Token B: {}", e)));
                    loading.set(false);
                    return;
                }
            };
            
            let slippage_val = match validate_slippage(&slippage) {
                Ok(val) => val,
                Err(e) => {
                    error.set(Some(e));
                    loading.set(false);
                    return;
                }
            };
            
            // Security: Pool ID validation
            if pool_id.is_empty() {
                error.set(Some("Pool selection is required".to_string()));
                loading.set(false);
                return;
            }
            
            // Create liquidity data
            let liquidity_data = LiquidityFormData {
                pool_id: (*pool_id).clone(),
                token_a_amount: token_a_val,
                token_b_amount: token_b_val,
                slippage_tolerance: slippage_val,
            };
            
            // Submit liquidity
            on_submit_callback.emit(liquidity_data);
            
            // Reset form
            token_a_amount.set(String::new());
            token_b_amount.set(String::new());
            loading.set(false);
        })
    };
    
    html! {
        <div class="bg-white shadow sm:rounded-lg">
            <div class="px-4 py-5 sm:p-6">
                <h3 class="text-lg font-medium leading-6 text-gray-900">{"Add Liquidity"}</h3>
                <form class="mt-5 space-y-4" onsubmit={on_submit}>
                    // Pool Selection
                    <div>
                        <label for="pool" class="block text-sm font-medium text-gray-700">
                            {"Pool"}
                        </label>
                        <select
                            id="pool"
                            name="pool"
                            class="mt-1 block w-full pl-3 pr-10 py-2 text-base border-gray-300 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm rounded-md"
                            onchange={on_pool_change}
                            aria-label="Select liquidity pool"
                        >
                            <option value="ETH/USDC">{"ETH/USDC"}</option>
                            <option value="BTC/USDC">{"BTC/USDC"}</option>
                            <option value="ETH/DAI">{"ETH/DAI"}</option>
                        </select>
                    </div>
                    
                    // Token A Amount
                    <div>
                        <label for="token-a-amount" class="block text-sm font-medium text-gray-700">
                            {"Token A Amount"}
                        </label>
                        <input
                            type="text"
                            id="token-a-amount"
                            name="token_a_amount"
                            class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
                            placeholder="0.0"
                            value={(*token_a_amount).clone()}
                            onchange={on_token_a_change}
                            aria-label="Enter token A amount"
                            aria-describedby="token-a-help"
                        />
                        <p id="token-a-help" class="mt-1 text-sm text-gray-500">
                            {"Enter the amount of the first token to add"}
                        </p>
                    </div>
                    
                    // Token B Amount
                    <div>
                        <label for="token-b-amount" class="block text-sm font-medium text-gray-700">
                            {"Token B Amount"}
                        </label>
                        <input
                            type="text"
                            id="token-b-amount"
                            name="token_b_amount"
                            class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
                            placeholder="0.0"
                            value={(*token_b_amount).clone()}
                            onchange={on_token_b_change}
                            aria-label="Enter token B amount"
                            aria-describedby="token-b-help"
                        />
                        <p id="token-b-help" class="mt-1 text-sm text-gray-500">
                            {"Enter the amount of the second token to add"}
                        </p>
                    </div>
                    
                    // Slippage Tolerance
                    <div>
                        <label for="slippage" class="block text-sm font-medium text-gray-700">
                            {"Slippage Tolerance (%)"}
                        </label>
                        <input
                            type="text"
                            id="slippage"
                            name="slippage"
                            class="mt-1 block w-full border border-gray-300 rounded-md shadow-sm py-2 px-3 focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 sm:text-sm"
                            placeholder="0.5"
                            value={(*slippage).clone()}
                            onchange={on_slippage_change}
                            aria-label="Enter slippage tolerance percentage"
                            aria-describedby="slippage-help"
                        />
                        <p id="slippage-help" class="mt-1 text-sm text-gray-500">
                            {"Maximum acceptable price movement (0-50%)"}
                        </p>
                    </div>
                    
                    // Price Impact Warning
                    if *price_impact > 5.0 {
                        <div class="rounded-md bg-yellow-50 p-4" role="alert" aria-live="polite">
                            <div class="flex">
                                <div class="flex-shrink-0">
                                    <svg class="h-5 w-5 text-yellow-400" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                        <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
                                    </svg>
                                </div>
                                <div class="ml-3">
                                    <h3 class="text-sm font-medium text-yellow-800">
                                        {format!("High Price Impact: {:.2}%", *price_impact)}
                                    </h3>
                                    <div class="mt-2 text-sm text-yellow-700">
                                        <p>{"This transaction may result in significant price movement."}</p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                    
                    // Error Display
                    if let Some(err) = (*error).clone() {
                        <div class="rounded-md bg-red-50 p-4" role="alert" aria-live="assertive">
                            <div class="flex">
                                <div class="ml-3">
                                    <h3 class="text-sm font-medium text-red-800">{err}</h3>
                                </div>
                            </div>
                        </div>
                    }
                    
                    // Submit Button
                    <div>
                        <button
                            type="submit"
                            disabled={*loading}
                            class="w-full flex justify-center py-2 px-4 border border-transparent rounded-md shadow-sm text-sm font-medium text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
                            aria-label="Add liquidity to pool"
                        >
                            if *loading {
                                <svg class="animate-spin h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" aria-hidden="true">
                                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                </svg>
                                <span class="ml-2">{"Adding Liquidity..."}</span>
                            } else {
                                {"Add Liquidity"}
                            }
                        </button>
                    </div>
                </form>
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
    fn test_liquidity_form_data_serialization() {
        let data = LiquidityFormData {
            pool_id: "ETH/USDC".to_string(),
            token_a_amount: 10.0,
            token_b_amount: 25000.0,
            slippage_tolerance: 0.5,
        };
        
        let serialized = serde_json::to_string(&data).unwrap();
        let deserialized: LiquidityFormData = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(data, deserialized);
    }
}
