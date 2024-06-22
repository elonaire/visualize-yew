pub mod charts;

#[cfg(feature = "BarChart")]
pub use charts::bar_chart::bar_chart;

#[cfg(feature = "PieChart")]
pub use charts::pie_chart::pie_chart;

#[cfg(feature = "LineChart")]
pub use charts::line_chart::line_chart;

#[cfg(feature = "MultilineChart")]
pub use charts::multi_line_chart::multi_line_chart;

#[cfg(feature = "DoughnutChart")]
pub use charts::doughnut_chart::doughnut_chart;
