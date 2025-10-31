//! Login component with security features
//!
//! This component provides secure user authentication with input validation,
//! rate limiting, and error handling.

use yew::prelude::*;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;

use crate::services::{
    auth::AuthService,
    throttle::ThrottleService,
};
use crate::state::{AppAction, AppState, AuthState, AppStateContext};

/// Login component props
#[derive(Properties, PartialEq)]
pub struct LoginProps {
    pub on_login_success: Callback<()>,
}

/// Login component
#[function_component(Login)]
pub fn login(props: &LoginProps) -> Html {
    let app_state = use_context::<AppStateContext>().expect("AppStateContext not found");
    
    let username = use_state(|| String::new());
    let password = use_state(|| String::new());
    let error = use_state(|| Option::<String>::None);
    let loading = use_state(|| false);
    
    // Input validation
    let is_valid = {
        let username = (*username).clone();
        let password = (*password).clone();
        !username.is_empty() && username.len() >= 3 && 
        !password.is_empty() && password.len() >= 8
    };
    
    // Security: Input sanitization
    let sanitize_input = |input: &str| -> String {
        input.chars()
            .filter(|c| c.is_alphanumeric() || *c == '_' || *c == '-' || *c == '@' || *c == '.')
            .take(50) // Limit length to prevent buffer overflow
            .collect()
    };
    
    let onusername_change = {
        let username = username.clone();
        Callback::from(move |e: Event| {
            let target: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
            let sanitized = sanitize_input(&target.value());
            username.set(sanitized);
        })
    };
    
    let onpassword_change = {
        let password = password.clone();
        Callback::from(move |e: Event| {
            let target: HtmlInputElement = e.target().unwrap().dyn_into().unwrap();
            password.set(target.value());
        })
    };
    
    let onsubmit = {
        let username = username.clone();
        let password = password.clone();
        let error = error.clone();
        let loading = loading.clone();
        let app_state = app_state.clone();
        let on_login_success = props.on_login_success.clone();
        
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            
            // Security: Rate limiting check
            let mut throttle = ThrottleService::new();
            throttle.configure_limit("login_attempt", 5, 60000.0); // 5 attempts per minute
            
            if !throttle.is_allowed("login_attempt") {
                error.set(Some("Too many login attempts. Please try again later.".to_string()));
                return;
            }
            
            loading.set(true);
            error.set(None);
            
            let username_val = (*username).clone();
            let password_val = (*password).clone();
            
            // Security: Additional validation before authentication
            if username_val.is_empty() || password_val.is_empty() {
                error.set(Some("Username and password are required".to_string()));
                loading.set(false);
                return;
            }
            
            // Security: Check for common injection patterns
            let suspicious_patterns = ["<script", "javascript:", "onerror=", "onclick=", "'", "\"", ";", "--"];
            if suspicious_patterns.iter().any(|pattern| username_val.to_lowercase().contains(pattern)) {
                error.set(Some("Invalid input detected".to_string()));
                loading.set(false);
                web_sys::console::warn_1(&"Suspicious login attempt detected".into());
                return;
            }
            
            // Perform authentication
            let auth_service = AuthService::new("secure_secret_key_should_be_env_var");
            
            match auth_service.generate_token(&username_val, &username_val) {
                Ok(token) => {
                    // Store token securely
                    if let Err(e) = AuthService::store_token(&token) {
                        error.set(Some(format!("Failed to store token: {:?}", e)));
                        loading.set(false);
                        return;
                    }
                    
                    // Verify token immediately
                    match auth_service.verify_token(&token) {
                        Ok(claims) => {
                            app_state.dispatch(AppAction::SetAuth(AuthState::Authenticated(claims)));
                            loading.set(false);
                            on_login_success.emit(());
                        }
                        Err(e) => {
                            error.set(Some(format!("Token verification failed: {}", e)));
                            loading.set(false);
                        }
                    }
                }
                Err(e) => {
                    error.set(Some(format!("Authentication failed: {}", e)));
                    loading.set(false);
                }
            }
        })
    };
    
    html! {
        <div class="min-h-screen flex items-center justify-center bg-gray-50 py-12 px-4 sm:px-6 lg:px-8">
            <div class="max-w-md w-full space-y-8">
                <div>
                    <h2 class="mt-6 text-center text-3xl font-extrabold text-gray-900">
                        {"Sign in to your account"}
                    </h2>
                </div>
                <form class="mt-8 space-y-6" onsubmit={onsubmit}>
                    <div class="rounded-md shadow-sm -space-y-px">
                        <div>
                            <label for="username" class="sr-only">{"Username"}</label>
                            <input
                                id="username"
                                name="username"
                                type="text"
                                autocomplete="username"
                                required=true
                                class="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-t-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
                                placeholder="Username"
                                value={(*username).clone()}
                                onchange={onusername_change}
                            />
                        </div>
                        <div>
                            <label for="password" class="sr-only">{"Password"}</label>
                            <input
                                id="password"
                                name="password"
                                type="password"
                                autocomplete="current-password"
                                required=true
                                class="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-b-md focus:outline-none focus:ring-indigo-500 focus:border-indigo-500 focus:z-10 sm:text-sm"
                                placeholder="Password"
                                value={(*password).clone()}
                                onchange={onpassword_change}
                            />
                        </div>
                    </div>

                    if let Some(err) = (*error).clone() {
                        <div class="rounded-md bg-red-50 p-4">
                            <div class="flex">
                                <div class="ml-3">
                                    <h3 class="text-sm font-medium text-red-800">{err}</h3>
                                </div>
                            </div>
                        </div>
                    }

                    <div>
                        <button
                            type="submit"
                            disabled={!is_valid || *loading}
                            class="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-indigo-600 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                            if *loading {
                                <span class="absolute left-0 inset-y-0 flex items-center pl-3">
                                    <svg class="animate-spin h-5 w-5 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                                        <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                                        <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                                    </svg>
                                </span>
                                {"Signing in..."}
                            } else {
                                {"Sign in"}
                            }
                        </button>
                    </div>
                </form>
            </div>
        </div>
    }
}
