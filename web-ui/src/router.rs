//! Router module for the web UI
//!
//! This module defines the application routes and navigation.

use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    home::Home, markets::Markets, not_found::NotFound, orders::Orders, pools::Pools,
};

/// Application routes
#[derive(Clone, PartialEq, Routable, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/pools")]
    Pools,
    #[at("/orders")]
    Orders,
    #[at("/markets")]
    Markets,
    #[not_found]
    #[at("/404")]
    NotFound,
}

/// Route switch function
pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Pools => html! { <Pools /> },
        Route::Orders => html! { <Orders /> },
        Route::Markets => html! { <Markets /> },
        Route::NotFound => html! { <NotFound /> },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use yew_router::Routable;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_route_paths() {
        assert_eq!(Route::Home.to_path(), "/");
        assert_eq!(Route::Pools.to_path(), "/pools");
        assert_eq!(Route::Orders.to_path(), "/orders");
        assert_eq!(Route::Markets.to_path(), "/markets");
        assert_eq!(Route::NotFound.to_path(), "/404");
    }
}
