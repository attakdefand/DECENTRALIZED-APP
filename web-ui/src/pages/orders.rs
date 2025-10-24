//! Orders page
//!
//! This page displays user orders and allows management.

use yew::prelude::*;

use crate::components::order_table::{OrderData, OrderSide, OrderStatus, OrderTable};

/// Orders page component
#[function_component(Orders)]
pub fn orders() -> Html {
    // Sample order data - in a real app this would come from an API
    let orders = vec![
        OrderData {
            id: "12345".to_string(),
            pair: "ETH/USDC".to_string(),
            side: OrderSide::Buy,
            price: 2500.50,
            amount: 1.5,
            filled: 1.0,
            status: OrderStatus::Open,
        },
        OrderData {
            id: "12346".to_string(),
            pair: "BTC/USDC".to_string(),
            side: OrderSide::Sell,
            price: 45000.00,
            amount: 0.25,
            filled: 0.25,
            status: OrderStatus::Filled,
        },
        OrderData {
            id: "12347".to_string(),
            pair: "ETH/DAI".to_string(),
            side: OrderSide::Buy,
            price: 2450.75,
            amount: 2.0,
            filled: 0.0,
            status: OrderStatus::Open,
        },
    ];

    html! {
        <div>
            <div class="flex justify-between items-center">
                <h2 class="text-2xl font-bold text-gray-900">{"Orders"}</h2>
                <div class="flex space-x-2">
                    <button class="bg-indigo-600 hover:bg-indigo-700 text-white font-medium py-2 px-4 rounded-md">
                        {"New Order"}
                    </button>
                    <button class="bg-gray-600 hover:bg-gray-700 text-white font-medium py-2 px-4 rounded-md">
                        {"Cancel All"}
                    </button>
                </div>
            </div>

            <div class="mt-6">
                <OrderTable orders={orders} />
            </div>

            <div class="mt-8">
                <h3 class="text-lg font-medium text-gray-900">{"Order Types"}</h3>
                <div class="mt-4 grid grid-cols-1 gap-5 sm:grid-cols-3">
                    <div class="bg-white overflow-hidden shadow rounded-lg p-4">
                        <h4 class="text-md font-medium text-gray-900">{"Market Order"}</h4>
                        <p class="mt-2 text-sm text-gray-600">
                            {"Executes immediately at the best available price."}
                        </p>
                    </div>
                    <div class="bg-white overflow-hidden shadow rounded-lg p-4">
                        <h4 class="text-md font-medium text-gray-900">{"Limit Order"}</h4>
                        <p class="mt-2 text-sm text-gray-600">
                            {"Executes only when the price reaches your specified level."}
                        </p>
                    </div>
                    <div class="bg-white overflow-hidden shadow rounded-lg p-4">
                        <h4 class="text-md font-medium text-gray-900">{"Stop Order"}</h4>
                        <p class="mt-2 text-sm text-gray-600">
                            {"Triggers a market order when the price reaches a specific level."}
                        </p>
                    </div>
                </div>
            </div>
        </div>
    }
}
