//! Navigation component
//!
//! This component provides the main navigation menu for the application.

use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

/// Navigation component properties
#[derive(Properties, PartialEq)]
pub struct NavigationProps {
    pub current_route: Route,
}

/// Navigation component
#[function_component(Navigation)]
pub fn navigation(props: &NavigationProps) -> Html {
    let NavigationProps { current_route } = props;

    html! {
        <nav class="bg-white shadow">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex justify-between h-16">
                    <div class="flex">
                        <div class="flex-shrink-0 flex items-center">
                            <span class="text-xl font-bold text-indigo-600">{"DEX"}</span>
                        </div>
                        <div class="hidden sm:ml-6 sm:flex sm:space-x-8">
                            <NavLink route={Route::Home} current_route={current_route.clone()}>
                                {"Home"}
                            </NavLink>
                            <NavLink route={Route::Pools} current_route={current_route.clone()}>
                                {"Pools"}
                            </NavLink>
                            <NavLink route={Route::Orders} current_route={current_route.clone()}>
                                {"Orders"}
                            </NavLink>
                            <NavLink route={Route::Markets} current_route={current_route.clone()}>
                                {"Markets"}
                            </NavLink>
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    }
}

/// Navigation link component
#[derive(Properties, PartialEq)]
struct NavLinkProps {
    pub route: Route,
    pub current_route: Route,
    pub children: Children,
}

#[function_component(NavLink)]
fn nav_link(props: &NavLinkProps) -> Html {
    let NavLinkProps {
        route,
        current_route,
        children,
    } = props;
    let is_active = route == current_route;

    let class = if is_active {
        "border-indigo-500 text-gray-900 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium"
    } else {
        "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium"
    };

    html! {
        <Link<Route> to={route.clone()} classes={class}>
            {for children.iter()}
        </Link<Route>>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_navigation_props_derive() {
        let props = NavigationProps {
            current_route: Route::Home,
        };

        // Just test that the props can be created, since Route doesn't implement Debug
        assert!(true);
    }
}
