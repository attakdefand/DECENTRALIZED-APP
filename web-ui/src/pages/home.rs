//! Home page
//!
//! This is the main landing page of the application.

use yew::prelude::*;

/// Home page component
#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <div class="bg-white overflow-hidden shadow rounded-lg">
                <div class="px-4 py-5 sm:p-6">
                    <h2 class="text-2xl font-bold text-gray-900">{"Welcome to the Decentralized Exchange"}</h2>
                    <p class="mt-2 text-gray-600">
                        {"Trade tokens, provide liquidity, and earn rewards in a fully decentralized exchange."}
                    </p>
                    <div class="mt-6 grid grid-cols-1 gap-5 sm:grid-cols-3">
                        <div class="bg-indigo-50 rounded-lg p-4">
                            <h3 class="text-lg font-medium text-indigo-800">{"Trade"}</h3>
                            <p class="mt-2 text-indigo-700">
                                {"Swap tokens instantly with low fees and high liquidity."}
                            </p>
                        </div>
                        <div class="bg-green-50 rounded-lg p-4">
                            <h3 class="text-lg font-medium text-green-800">{"Earn"}</h3>
                            <p class="mt-2 text-green-700">
                                {"Provide liquidity to pools and earn trading fees."}
                            </p>
                        </div>
                        <div class="bg-purple-50 rounded-lg p-4">
                            <h3 class="text-lg font-medium text-purple-800">{"Stake"}</h3>
                            <p class="mt-2 text-purple-700">
                                {"Stake your tokens to participate in governance and earn rewards."}
                            </p>
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="mt-8">
                <h3 class="text-lg font-medium text-gray-900">{"Recent Activity"}</h3>
                <div class="mt-4 bg-white shadow overflow-hidden sm:rounded-md">
                    <ul class="divide-y divide-gray-200">
                        <li>
                            <a href="#" class="block hover:bg-gray-50">
                                <div class="px-4 py-4 sm:px-6">
                                    <div class="flex items-center justify-between">
                                        <p class="text-sm font-medium text-indigo-600 truncate">{"Swap 10 ETH for 25,000 USDC"}</p>
                                        <div class="ml-2 flex-shrink-0 flex">
                                            <p class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-green-100 text-green-800">
                                {"Completed"}
                              </p>
                                        </div>
                                    </div>
                                    <div class="mt-2 sm:flex sm:justify-between">
                                        <div class="sm:flex">
                                            <p class="flex items-center text-sm text-gray-500">
                                                <svg class="flex-shrink-0 mr-1.5 h-5 w-5 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4l3 3m6-3a9 9 0 11-18 0 9 9 0 0118 0z" />
                                                </svg>
                                                {"2 hours ago"}
                                            </p>
                                        </div>
                                    </div>
                                </div>
                            </a>
                        </li>
                    </ul>
                </div>
            </div>
        </div>
    }
}