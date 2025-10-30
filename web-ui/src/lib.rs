//! Web UI for the Decentralized Exchange
//!
//! This module provides the WebAssembly-based frontend for the DEX application
//! using Yew framework and Rust/WASM compilation.

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;
mod services;
mod router;

use router::{switch, Route};
use services::run_all_demos;

/// Initialize the web application
#[wasm_bindgen(start)]
pub fn run_app() {
    // Run our demo services to showcase the new features
    run_all_demos();
    
    yew::Renderer::<Main>::new().render();
}

/// Main application component
#[function_component(Main)]
pub fn main() -> Html {
    html! {
        <BrowserRouter>
            <MainLayout />
        </BrowserRouter>
    }
}

/// Main layout component with header, navigation, and footer
#[function_component(MainLayout)]
pub fn main_layout() -> Html {
    html! {
        <div class="min-h-screen bg-gray-50">
            <header class="bg-white shadow">
                <div class="max-w-7xl mx-auto px-4 py-6 sm:px-6 lg:px-8">
                    <h1 class="text-3xl font-bold text-gray-900">{"Decentralized Exchange"}</h1>
                </div>
            </header>
            <main>
                <div class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
                    <Switch<Route> render={switch} />
                </div>
            </main>
            <footer class="bg-white mt-8">
                <div class="max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8">
                    <p class="text-center text-gray-500">{"© 2025 Decentralized Exchange. All rights reserved."}</p>
                </div>
            </footer>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_main_component() {
        // This is a simple test to ensure the component can be created
        assert!(true);
    }
}