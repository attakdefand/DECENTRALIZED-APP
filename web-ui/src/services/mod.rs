//! Services module
//!
//! This module contains services for interacting with the backend API.

pub mod api;
pub mod markets;
pub mod orders;
pub mod pools;

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_services_module_structure() {
        // This is a simple test to ensure the module structure is correct
        assert!(true);
    }
}
