//! # Visualize Yew
//!
//! `Visualize Yew` is a simple crate to help you visualize your data in the browser using Yew. It is a wrapper around the yew crate that provides a simple API to create charts.
//!
//! ## Features
//!
//! - `BarChart` — Renders a standard bar chart.
//! - `PieChart` — Renders a pie chart.
//! - `LineCurveChart` — Renders a line chart with optional curve smoothing.
//! - `DoughnutChart` — Renders a doughnut chart similar to a pie chart but with a hole in the center.
//!
//! Enable the desired chart(s) in your `Cargo.toml`:
//!
//! ```toml
//! visualize-yew = { version = "0.2x.x", features = ["BarChart", "PieChart"] }
//! ```
//!
//! ## Example
//!
//! ```rust
//! use visualize_yew::pie_chart::{DataPoint as PieChartData, PieChart};
//!
//! #[function_component]
//! fn Home() -> Html {
//!     let mut pie_chart_config = PieChartConfig::default();
//!     pie_chart_config.show_legend = true;
//!
//!     let pie_data = vec![
//!         PieChartData::new("A", 10, ""),
//!         PieChartData::new("B", 20, ""),
//!         PieChartData::new("C", 30, ""),
//!         PieChartData::new("D", 40, ""),
//!     ];
//!
//!     html! {
//!         // Chart will take the full width of the parent container
//!         <div>
//!             <PieChart data={pie_chart_data} config={pie_chart_config} />
//!         </div>
//!     }
//! }
//! ```

pub mod charts;

#[cfg(feature = "BarChart")]
/// Renders a bar chart.
///
/// This feature is enabled by default.
pub use charts::bar_chart::bar_chart;

#[cfg(feature = "PieChart")]
/// Renders a pie chart.
///
/// Enable this via the `PieChart` feature in Cargo.toml.
pub use charts::pie_chart::pie_chart;

#[cfg(feature = "LineCurveChart")]
/// Renders a line chart with optional curves.
///
/// Enable this via the `LineCurveChart` feature in Cargo.toml.
pub use charts::line_chart::line_chart;

#[cfg(feature = "DoughnutChart")]
/// Renders a doughnut chart.
///
/// Enable this via the `DoughnutChart` feature in Cargo.toml.
pub use charts::doughnut_chart::doughnut_chart;
