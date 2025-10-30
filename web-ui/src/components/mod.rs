//! Components module
//!
//! This module contains reusable UI components.

pub mod navigation;
pub mod pool_card;
pub mod order_table;
pub mod market_chart;

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_components_module_structure() {
        // This is a simple test to ensure the module structure is correct
        assert!(true);
    }
}