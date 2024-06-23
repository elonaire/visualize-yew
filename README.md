# Visualize Yew
This is a simple crate to help you visualize your data in the browser using Yew. It is a wrapper around the yew crate that provides a simple API to create charts.

This crate is still in development and is not yet ready for production use. The API is subject to change.

This crate is built using the [yew](https://yew.rs/docs/en/) framework and uses HTML5 canvas to render the charts.

## Features
- [ ] PieChart
- [ ] LineChart
- [x] BarChart
- [ ] DoughnutChart

## Usage
Add the following to your `Cargo.toml`:
```toml
[dependencies]
visualize-yew = { version = "0.20.0", features = ["PieChart"] }
```

## Example
```rust
use visualize_yew::pie_chart::{DataPoint as PieChartData, PieChart};

#[function_component]
fn Home() -> Html {
    let pie_chart_data: Vec<PieChartData> = vec![
        PieChartData {
            name: "A".to_string(),
            value: 10,
        },
        PieChartData {
            name: "B".to_string(),
            value: 20,
        },
        PieChartData {
            name: "C".to_string(),
            value: 30,
        },
        PieChartData {
            name: "D".to_string(),
            value: 40,
        },
        PieChartData {
            name: "E".to_string(),
            value: 50,
        },
    ];

    html! {
        // Chart will take the full width of the parent container
        <div>
            <PieChart data={pie_chart_data} />
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
