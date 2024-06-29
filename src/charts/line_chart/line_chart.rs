use gloo::events::EventListener;
use web_sys::{
    wasm_bindgen::{JsCast, JsValue},
    window, CanvasRenderingContext2d, HtmlCanvasElement,
};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Properties, Default)]
pub struct LineCurveChartConfig {
    // Add configuration properties here
    #[prop_or(true)]
    pub show_grid: bool,
    #[prop_or(true)]
    pub show_legend: bool,
    #[prop_or(true)]
    pub show_inflection_points: bool,
    #[prop_or(true)]
    pub show_x_labels: bool,
    #[prop_or(true)]
    pub show_y_labels: bool,
    #[prop_or(true)]
    pub show_x_axis: bool,
    #[prop_or(true)]
    pub show_y_axis: bool,
    #[prop_or(true)]
    pub show_x_axis_labels: bool,
    #[prop_or(true)]
    pub show_y_axis_labels: bool,
    #[prop_or_default]
    pub stroke_width: i32,
}

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct DataPoint {
    // pub x: String, // independent variable
    pub y: i32, // dependent variable
}

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct Series {
    pub name: String,
    pub color: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct LineCurveChartProps {
    pub data: Vec<(Series, Vec<DataPoint>)>,
    pub x: Vec<String>,
    #[prop_or_default]
    pub config: LineCurveChartConfig,
}

#[function_component]
pub fn LineCurveChart(props: &LineCurveChartProps) -> Html {
    let canvas_ref = use_node_ref();

    {
        let canvas_ref = canvas_ref.clone();
        let props_clone = props.clone();
        use_effect_with_deps(
            move |_| {
                let canvas = canvas_ref
                    .cast::<HtmlCanvasElement>()
                    .expect("Failed to get canvas element");

                let context = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();

                let props_clone_resize = props_clone.clone();
                let resize_callback = {
                    let canvas_ref = canvas_ref.clone();
                    move || {
                        let canvas = canvas_ref
                            .cast::<HtmlCanvasElement>()
                            .expect("Failed to get canvas element");

                        let device_pixel_ratio = window().unwrap().device_pixel_ratio();
                        let parent = canvas.parent_element().unwrap();
                        let width = parent.client_width() as f64;
                        // let height = parent.client_height() as f64;
                        let height = width * 0.6;

                        // Set the canvas dimensions to match its parent's dimensions
                        // Set the canvas dimensions to match its parent's dimensions
                        canvas.set_width((width * device_pixel_ratio) as u32);
                        canvas.set_height((height * device_pixel_ratio) as u32);

                        // Scale the context to account for the device pixel ratio
                        context
                            .scale(device_pixel_ratio, device_pixel_ratio)
                            .unwrap();

                        draw_multiline_chart(&context, width, height, &props_clone_resize);
                    }
                };

                resize_callback(); // Initial call to set canvas size

                let listener = EventListener::new(&window().unwrap(), "resize", move |_event| {
                    resize_callback();
                });

                move || drop(listener) // Clean up the event listener on component unmount
            },
            (),
        );
    }

    // Render the legend if enabled
    let legend_html = if props.config.show_legend {
        html! {
            <div style="display: flex; flex-direction: row; gap: 5px;  flex-wrap: wrap;">
                { for props.data.iter().map(|(series, _)| {
                    html! {
                        <div style="display: flex; flex-direction: row; align-items: center; gap: 2px;">
                                <span style="font-size: 10px;">{ &series.name }</span>
                            <div style={format!("background-color: {}; width: 10px; height: 10px; display: inline-block;", series.color)}></div>
                        </div>
                    }
                })}
            </div>
        }
    } else {
        html! {}
    };

    html! {
        <div>
            // legend
            { legend_html }
            <canvas ref={canvas_ref} style="width: 100%; height: 100%; box-sizing: border-box;"></canvas>
        </div>
    }
}

fn draw_multiline_chart(
    context: &CanvasRenderingContext2d,
    width: f64,
    height: f64,
    props: &LineCurveChartProps,
) {
    let datasets = &props.data;

    let axis_padding = 50.0;
    let max_value = datasets
        .iter()
        .flat_map(|(_, data)| data.iter().map(|datapoint| datapoint.y))
        .max()
        // .cloned()
        .unwrap_or(0) as f64
        * 1.2;
    let num_points = datasets.first().unwrap().1.len() as f64;
    let point_spacing = (width - axis_padding * 2.0) / (num_points - 1.0);

    context.set_fill_style(&JsValue::from_str("white"));
    context.clear_rect(0.0, 0.0, width, height);

    // Draw x-axis
    if props.config.show_x_axis {
        context.set_stroke_style(&JsValue::from_str("#cccccc"));
        context.set_line_width(1.0);
        context.begin_path();
        context.move_to(axis_padding, height - axis_padding);
        context.line_to(width, height - axis_padding);
        context.stroke();
    }

    // Draw y-axis
    if props.config.show_y_axis {
        context.set_stroke_style(&JsValue::from_str("#cccccc"));
        context.set_line_width(1.0);
        context.begin_path();
        context.move_to(axis_padding, 0.0);
        context.line_to(axis_padding, height - axis_padding);
        context.stroke();
    }

    // Draw the y-axis grid lines and labels
    context.set_stroke_style(&JsValue::from_str("#cccccc"));
    context.set_line_width(1.0);
    context.set_fill_style(&JsValue::from_str("black"));
    context.set_text_align("right");
    context.set_text_baseline("middle");

    let num_grid_lines = 5;
    let step_value = max_value / num_grid_lines as f64;
    let step_height = (height - axis_padding * 2.0) / num_grid_lines as f64;

    for i in 0..=num_grid_lines {
        let y = height - axis_padding - i as f64 * step_height;
        if props.config.show_grid {
            context.begin_path();
            context.move_to(axis_padding, y);
            context.line_to(width, y);
            context.stroke();
        }

        // Draw the y-axis labels
        if props.config.show_y_axis_labels {
            let label = (i as f64 * step_value).round();
            context
                .fill_text(&format!("{}", label), axis_padding - 10.0, y)
                .unwrap();
        }
    }

    // Draw each dataset as a separate line
    for (series, data) in datasets {
        context.set_stroke_style(&JsValue::from_str(series.color.as_str()));
        context.set_line_width(props.config.stroke_width as f64);

        context.begin_path();
        context.move_to(
            axis_padding,
            height - axis_padding - (data[0].y as f64 / max_value) * (height - axis_padding * 2.0),
        );

        for i in 1..data.len() {
            let x = axis_padding + i as f64 * point_spacing;
            let y = height
                - axis_padding
                - (data[i].y as f64 / max_value) * (height - axis_padding * 2.0);

            let prev_x = axis_padding + ((i - 1) as f64) * point_spacing;
            let prev_y = height
                - axis_padding
                - (data[i - 1].y as f64 / max_value) * (height - axis_padding * 2.0);

            let ctrl1_x = prev_x + point_spacing / 3.0;
            let ctrl1_y = prev_y;

            let ctrl2_x = x - point_spacing / 3.0;
            let ctrl2_y = y;

            context.bezier_curve_to(ctrl1_x, ctrl1_y, ctrl2_x, ctrl2_y, x, y);
        }
        context.stroke();

        // Add colored dots at inflection points
        if props.config.show_inflection_points {
            context.set_fill_style(&JsValue::from_str(series.color.as_str()));
            for (i, datapoint) in data.iter().enumerate() {
                let x = axis_padding + i as f64 * point_spacing;
                let y = height
                    - axis_padding
                    - (datapoint.y as f64 / max_value) * (height - axis_padding * 2.0);
                context.begin_path();
                context
                    .arc(x, y, 3.0, 0.0, std::f64::consts::PI * 2.0)
                    .unwrap();
                context.fill();
            }
        }
    }

    // Add x-axis labels
    if props.config.show_x_axis_labels {
        context.set_fill_style(&JsValue::from_str("black"));
        context.set_text_align("center");
        context.set_text_baseline("middle");

        let x_labels = props.x.clone().into_iter();
        for (i, x_label) in x_labels.enumerate() {
            let x = axis_padding + i as f64 * point_spacing;
            let y = height - axis_padding / 2.0;
            context.fill_text(x_label.as_str(), x, y).unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    // Function to create a mock CanvasRenderingContext2d
    fn mock_context() -> CanvasRenderingContext2d {
        // Create a canvas element
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document
            .create_element("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();

        // Get the 2D context from the canvas
        canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap()
    }

    #[wasm_bindgen_test]
    fn test_draw_multiline_chart() {
        let context = mock_context();
        let width = 800.0;
        let height = 600.0;

        let props = LineCurveChartProps {
            data: vec![
                (
                    Series {
                        name: "Dataset 1".to_string(),
                        color: "#ff0000".to_string(),
                    },
                    vec![
                        DataPoint { y: 10 },
                        DataPoint { y: 20 },
                        DataPoint { y: 15 },
                        DataPoint { y: 40 },
                        DataPoint { y: 30 },
                    ],
                ),
                (
                    Series {
                        name: "Dataset 2".to_string(),
                        color: "#00ff00".to_string(),
                    },
                    vec![
                        DataPoint { y: 50 },
                        DataPoint { y: 40 },
                        DataPoint { y: 30 },
                        DataPoint { y: 35 },
                        DataPoint { y: 20 },
                    ],
                ),
            ],
            x: vec![
                "0".to_string(),
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
                "4".to_string(),
            ],
            config: LineCurveChartConfig {
                show_grid: true,
                show_legend: true,
                show_inflection_points: true,
                show_x_labels: true,
                show_y_labels: true,
                show_x_axis: true,
                show_y_axis: true,
                show_x_axis_labels: true,
                show_y_axis_labels: true,
                stroke_width: 2,
            },
        };

        draw_multiline_chart(&context, width, height, &props);

        // Additional assertions would be needed to validate the correct behavior,
        // e.g., checking if certain methods were called or if certain values were set.
        // Since we cannot directly inspect the canvas from here, we assume success if no panic occurs.
        assert!(true);
    }
}
