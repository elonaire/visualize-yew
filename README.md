# Visualize Yew
![Visualize Yew CI](https://github.com/elonaire/visualize-yew/actions/workflows/main.yml/badge.svg)

This is a simple crate to help you visualize your data in the browser using Yew. It is a wrapper around the yew crate that provides a simple API to create charts.

**Note**: This crate is **NOW** available for use, all charts are customizable to your liking.

**New/Upcoming Features:**
- [x] Customizable colors for all charts
- [x] Customizable labels for all charts
- [x] Customizable legend for all charts
- [x] Customizable stroke width for line chart
- [ ] Customizable tooltip for all charts

This crate is built using the [Yew](https://yew.rs/docs/0.20/getting-started/introduction) framework and uses HTML5 canvas to render the charts.

## Features
- [x] PieChart

    <img src="https://imagedelivery.net/fa3SWf5GIAHiTnHQyqU8IQ/2fd337b3-4c77-45b8-8195-0ac85496e700/public" width="200">
- [x] LineChart

    <img src="https://imagedelivery.net/fa3SWf5GIAHiTnHQyqU8IQ/509533f0-8ab1-4333-2b22-408c2b8d1e00/public" width="200">
- [x] BarChart

    <img src="https://imagedelivery.net/fa3SWf5GIAHiTnHQyqU8IQ/ff0d07c0-681e-43a8-349c-571c1d389b00/public" width="200">
- [x] DoughnutChart

    <img src="https://imagedelivery.net/fa3SWf5GIAHiTnHQyqU8IQ/f30b8b58-668c-45ce-3923-5e9840abe400/public" width="200">

## Usage
Add the following to your `Cargo.toml`:
```toml
[dependencies]
visualize-yew = { version = "0.20.0-alpha.1", features = ["PieChart"] }
```

## Example
```rust
use visualize_yew::pie_chart::{DataPoint as PieChartData, PieChart};

#[function_component]
fn Home() -> Html {
    let mut pie_chart_config = PieChartConfig::default();
    pie_chart_config.show_legend = true;

    let pie_data = vec![
        PieDataPoint {
            name: "A".to_string(),
            value: 10,
            color: "#F47489".to_string(),
        },
        PieDataPoint {
            name: "B".to_string(),
            value: 20,
            color: "#43bc7e".to_string(),
        },
        PieDataPoint {
            name: "C".to_string(),
            value: 30,
            color: "#1ECBE1".to_string(),
        },
        PieDataPoint {
            name: "D".to_string(),
            value: 40,
            color: "#8900ef".to_string(),
        },
    ];

    html! {
        // Chart will take the full width of the parent container
        <div>
            <PieChart data={pie_chart_data} config={pie_chart_config} />
        </div>
    }
}
```

## License
This project is licensed under both the MIT license and the Apache License (Version 2.0).

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, shall be dual licensed as above, without any additional terms or conditions.

## Acknowledgements
This project is inspired by the [Chart.js](https://www.chartjs.org/docs/latest/) library for JavaScript.
