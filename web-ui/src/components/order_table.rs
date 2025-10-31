//! Order table component
//!
//! This component displays a table of orders.

use yew::prelude::*;

/// Order data structure
#[derive(Properties, PartialEq, Clone)]
pub struct OrderData {
    pub id: String,
    pub pair: String,
    pub side: OrderSide,
    pub price: f64,
    pub amount: f64,
    pub filled: f64,
    pub status: OrderStatus,
}

/// Order side enum
#[derive(PartialEq, Clone)]
pub enum OrderSide {
    Buy,
    Sell,
}

/// Order status enum
#[derive(PartialEq, Clone)]
pub enum OrderStatus {
    Open,
    Filled,
    Cancelled,
}

/// Order table properties
#[derive(Properties, PartialEq)]
pub struct OrderTableProps {
    pub orders: Vec<OrderData>,
}

/// Order table component
#[function_component(OrderTable)]
pub fn order_table(props: &OrderTableProps) -> Html {
    let OrderTableProps { orders } = props;

    html! {
        <div class="flex flex-col">
            <div class="-my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
                <div class="py-2 align-middle inline-block min-w-full sm:px-6 lg:px-8">
                    <div class="shadow overflow-hidden border-b border-gray-200 sm:rounded-lg">
                        <table class="min-w-full divide-y divide-gray-200">
                            <thead class="bg-gray-50">
                                <tr>
                                    <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                        {"Pair"}
                                    </th>
                                    <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                        {"Side"}
                                    </th>
                                    <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                        {"Price"}
                                    </th>
                                    <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                        {"Amount"}
                                    </th>
                                    <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                        {"Filled"}
                                    </th>
                                    <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                        {"Status"}
                                    </th>
                                </tr>
                            </thead>
                            <tbody class="bg-white divide-y divide-gray-200">
                                {orders.iter().map(|order| {
                                    html! {
                                        <tr>
                                            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                                                {&order.pair}
                                            </td>
                                            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                <span class={order_side_class(&order.side)}>
                                                    {order_side_text(&order.side)}
                                                </span>
                                            </td>
                                            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                {format!("{:.4}", order.price)}
                                            </td>
                                            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                {format!("{:.4}", order.amount)}
                                            </td>
                                            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                {format!("{:.2}%", (order.filled / order.amount) * 100.0)}
                                            </td>
                                            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                <span class={order_status_class(&order.status)}>
                                                    {order_status_text(&order.status)}
                                                </span>
                                            </td>
                                        </tr>
                                    }
                                }).collect::<Html>()}
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Get CSS class for order side
fn order_side_class(side: &OrderSide) -> &'static str {
    match side {
        OrderSide::Buy => "px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-green-100 text-green-800",
        OrderSide::Sell => "px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-red-100 text-red-800",
    }
}

/// Get text for order side
fn order_side_text(side: &OrderSide) -> &'static str {
    match side {
        OrderSide::Buy => "Buy",
        OrderSide::Sell => "Sell",
    }
}

/// Get CSS class for order status
fn order_status_class(status: &OrderStatus) -> &'static str {
    match status {
        OrderStatus::Open => "px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-yellow-100 text-yellow-800",
        OrderStatus::Filled => "px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-green-100 text-green-800",
        OrderStatus::Cancelled => "px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-red-100 text-red-800",
    }
}

/// Get text for order status
fn order_status_text(status: &OrderStatus) -> &'static str {
    match status {
        OrderStatus::Open => "Open",
        OrderStatus::Filled => "Filled",
        OrderStatus::Cancelled => "Cancelled",
    }
}