//! # Visualize Yew
//!
//! `Visualize Yew` is a modular charting library that supports multiple chart types behind feature flags.
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
//! rusty_charts = { version = "0.1", features = ["BarChart", "PieChart"] }
//! ```
//!
//! ## Example
//!
//! ```rust
//! #[cfg(feature = "BarChart")]
//! use rusty_charts::bar_chart;
//!
//! fn main() {
//!     #[cfg(feature = "BarChart")]
//!     bar_chart();
//! }
//! ```

pub mod charts;

#[cfg(feature = "BarChart")]
/// Renders a bar chart.
///
/// Enable this via the `BarChart` feature in Cargo.toml.
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
