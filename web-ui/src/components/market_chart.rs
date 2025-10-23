//! Market chart component
//!
//! This component displays a simple market chart using SVG.

use yew::prelude::*;

/// Chart data point
#[derive(Properties, PartialEq, Clone)]
pub struct ChartDataPoint {
    pub timestamp: u64,
    pub price: f64,
}

/// Market chart properties
#[derive(Properties, PartialEq)]
pub struct MarketChartProps {
    pub data: Vec<ChartDataPoint>,
    pub symbol: String,
}

/// Market chart component
#[function_component(MarketChart)]
pub fn market_chart(props: &MarketChartProps) -> Html {
    let MarketChartProps { data, symbol } = props;

    // If no data, show a message
    if data.is_empty() {
        return html! {
            <div class="bg-white overflow-hidden shadow rounded-lg p-6">
                <p class="text-center text-gray-500">{"No chart data available"}</p>
            </div>
        };
    }

    // Calculate min and max values for scaling
    let min_price = data.iter().map(|d| d.price).fold(f64::INFINITY, f64::min);
    let max_price = data.iter().map(|d| d.price).fold(f64::NEG_INFINITY, f64::max);
    let price_range = max_price - min_price;

    // Chart dimensions
    let width = 600.0;
    let height = 300.0;
    let padding = 40.0;

    // Convert data points to SVG coordinates
    let points: Vec<(f64, f64)> = data
        .iter()
        .enumerate()
        .map(|(i, point)| {
            let x = padding + (i as f64 / (data.len() - 1) as f64) * (width - 2.0 * padding);
            let y = if price_range > 0.0 {
                padding + (height - 2.0 * padding) * (max_price - point.price) / price_range
            } else {
                height / 2.0
            };
            (x, y)
        })
        .collect();

    // Create path data for the line
    let path_data = if !points.is_empty() {
        let mut path = format!("M {:.2} {:.2}", points[0].0, points[0].1);
        for (x, y) in points.iter().skip(1) {
            path.push_str(&format!(" L {:.2} {:.2}", x, y));
        }
        path
    } else {
        String::new()
    };

    html! {
        <div class="bg-white overflow-hidden shadow rounded-lg">
            <div class="px-4 py-5 sm:p-6">
                <h3 class="text-lg font-medium text-gray-900">
                    {format!("{} Price Chart", symbol)}
                </h3>
                <div class="mt-4">
                    <svg width={width.to_string()} height={height.to_string()} viewBox={format!("0 0 {} {}", width, height)}>
                        // X and Y axes
                        <line x1={padding.to_string()} y1={(height - padding).to_string()} x2={(width - padding).to_string()} y2={(height - padding).to_string()} stroke="black" stroke-width="1"/>
                        <line x1={padding.to_string()} y1={padding.to_string()} x2={padding.to_string()} y2={(height - padding).to_string()} stroke="black" stroke-width="1"/>
                        
                        // Chart line
                        <path d={path_data} fill="none" stroke="rgb(99, 102, 241)" stroke-width="2"/>
                        
                        // Min and max price labels
                        <text x={0.to_string()} y={(height - padding + 15.0).to_string()} font-size="12" fill="black">
                            {format!("{:.2}", min_price)}
                        </text>
                        <text x={0.to_string()} y={(padding + 15.0).to_string()} font-size="12" fill="black">
                            {format!("{:.2}", max_price)}
                        </text>
                    </svg>
                </div>
            </div>
        </div>
    }
}