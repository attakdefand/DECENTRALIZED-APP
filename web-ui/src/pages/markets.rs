//! Markets page
//!
//! This page displays market data and charts.

use yew::prelude::*;

use crate::components::market_chart::{MarketChart, ChartDataPoint};

/// Markets page component
#[function_component(Markets)]
pub fn markets() -> Html {
    // Sample chart data - in a real app this would come from an API
    let eth_chart_data = vec![
        ChartDataPoint { timestamp: 1620000000, price: 2400.0 },
        ChartDataPoint { timestamp: 1620086400, price: 2450.5 },
        ChartDataPoint { timestamp: 1620172800, price: 2500.0 },
        ChartDataPoint { timestamp: 1620259200, price: 2480.5 },
        ChartDataPoint { timestamp: 1620345600, price: 2520.0 },
        ChartDataPoint { timestamp: 1620432000, price: 2550.5 },
        ChartDataPoint { timestamp: 1620518400, price: 2530.0 },
    ];

    let btc_chart_data = vec![
        ChartDataPoint { timestamp: 1620000000, price: 44000.0 },
        ChartDataPoint { timestamp: 1620086400, price: 44500.5 },
        ChartDataPoint { timestamp: 1620172800, price: 45000.0 },
        ChartDataPoint { timestamp: 1620259200, price: 44800.5 },
        ChartDataPoint { timestamp: 1620345600, price: 45200.0 },
        ChartDataPoint { timestamp: 1620432000, price: 45500.5 },
        ChartDataPoint { timestamp: 1620518400, price: 45300.0 },
    ];

    html! {
        <div>
            <h2 class="text-2xl font-bold text-gray-900">{"Markets"}</h2>
            
            <div class="mt-6 grid grid-cols-1 gap-6 lg:grid-cols-2">
                <MarketChart data={eth_chart_data} symbol={"ETH/USDC".to_string()} />
                <MarketChart data={btc_chart_data} symbol={"BTC/USDC".to_string()} />
            </div>
            
            <div class="mt-8">
                <h3 class="text-lg font-medium text-gray-900">{"Market Overview"}</h3>
                <div class="mt-4 bg-white shadow overflow-hidden sm:rounded-lg">
                    <div class="px-4 py-5 sm:p-6">
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
                                                        {"Price"}
                                                    </th>
                                                    <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                                        {"24h Change"}
                                                    </th>
                                                    <th scope="col" class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                                        {"24h Volume"}
                                                    </th>
                                                </tr>
                                            </thead>
                                            <tbody class="bg-white divide-y divide-gray-200">
                                                <tr>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                                                        {"ETH/USDC"}
                                                    </td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                        {"$2,530.00"}
                                                    </td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm text-green-600">
                                                        {"+2.5%"}
                                                    </td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                        {"$45,000,000"}
                                                    </td>
                                                </tr>
                                                <tr>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                                                        {"BTC/USDC"}
                                                    </td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                        {"$45,300.00"}
                                                    </td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm text-red-600">
                                                        {"-1.2%"}
                                                    </td>
                                                    <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                                                        {"$87,000,000"}
                                                    </td>
                                                </tr>
                                            </tbody>
                                        </table>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}