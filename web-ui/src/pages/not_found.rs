//! Not found page
//!
//! This page is displayed when a route is not found.

use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

/// Not found page component
#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="min-h-full pt-16 pb-12 flex flex-col">
            <main class="flex-grow flex flex-col justify-center max-w-7xl w-full mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex-shrink-0 flex justify-center">
                    <Link<Route> to={Route::Home} classes="inline-flex">
                        <span class="sr-only">{"Decentralized Exchange"}</span>
                        <span class="text-2xl font-bold text-indigo-600">{"DEX"}</span>
                    </Link<Route>>
                </div>
                <div class="py-16">
                    <div class="text-center">
                        <p class="text-sm font-semibold text-indigo-600 uppercase tracking-wide">{"404 error"}</p>
                        <h1 class="mt-2 text-4xl font-extrabold text-gray-900 tracking-tight sm:text-5xl">{"Page not found."}</h1>
                        <p class="mt-2 text-base text-gray-500">{"Sorry, we couldn’t find the page you’re looking for."}</p>
                        <div class="mt-6">
                            <Link<Route> to={Route::Home} classes="text-base font-medium text-indigo-600 hover:text-indigo-500">
                                {"Go back home"}
                                <span aria-hidden="true">{" →"}</span>
                            </Link<Route>>
                        </div>
                    </div>
                </div>
            </main>
        </div>
    }
}