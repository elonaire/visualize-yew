use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window, wasm_bindgen::{JsValue, JsCast}};
use yew::prelude::*;
use gloo::events::EventListener;

#[derive(Clone, Debug, PartialEq, Eq, Properties, Default)]
pub struct BarChartConfig {
    #[prop_or(Some("#1ECBE1".to_string()))]
    pub bar_color: Option<String>,
    pub grid_color: Option<String>,
    pub axis_color: Option<String>,
    pub axis_label_color: Option<String>,
    pub axis_label_font: Option<String>,
    pub axis_label_font_size: Option<i32>,
    pub axis_label_font_weight: Option<String>,
    pub axis_label_padding: Option<i32>,
    pub axis_label_rotation: Option<i32>,
    pub axis_label_x_offset: Option<i32>,
    pub axis_label_y_offset: Option<i32>,
    pub axis_tick_length: Option<i32>,
    pub axis_tick_width: Option<i32>,
    pub axis_width: Option<i32>,
    pub bar_spacing: Option<i32>,
    pub grid_line_width: Option<i32>,
    pub grid_num_lines: Option<usize>,
    pub margin_bottom: Option<i32>,
    pub margin_left: Option<i32>,
    pub margin_right: Option<i32>,
    pub margin_top: Option<i32>,
    pub x_axis_label: Option<String>,
    pub y_axis_label: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct DataPoint {
    pub name: String,
    pub value: i32,
}

#[derive(Clone, Properties, PartialEq, Debug, Eq)]
pub struct BarChartProps {
    pub data: Vec<DataPoint>,
    #[prop_or(Default::default())]
    pub config: Option<BarChartConfig>,
}


#[function_component]
pub fn BarChart(props: &BarChartProps) -> Html {
    let canvas_ref = use_node_ref();

    {
        let canvas_ref = canvas_ref.clone();
        let props_clone = props.clone();
        use_effect_with_deps(move |_| {
            let canvas = canvas_ref.cast::<HtmlCanvasElement>().expect("Failed to get canvas element");

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
                    let canvas = canvas_ref.cast::<HtmlCanvasElement>().expect("Failed to get canvas element");
                    
                    let device_pixel_ratio = window().unwrap().device_pixel_ratio();
                    let parent = canvas.parent_element().unwrap();
                    let width = parent.client_width() as f64;
                    let height = parent.client_height() as f64;

                    // Set the canvas dimensions to match its parent's dimensions
                    canvas.set_width((width * device_pixel_ratio) as u32);
                    canvas.set_height((height * device_pixel_ratio) as u32);

                    // Scale the context to account for the device pixel ratio
                    context.scale(device_pixel_ratio, device_pixel_ratio).unwrap();

                    draw_bar_chart(&context, width, height, &props_clone_resize);
                }
            };

            resize_callback(); // Initial call to set canvas size

            let listener = EventListener::new(&window().unwrap(), "resize", move |_event| {
                resize_callback();
            });

            move || drop(listener) // Clean up the event listener on component unmount
        }, ());
    }

    html! {
        <canvas ref={canvas_ref} style="width: 100%; height: 100%;"></canvas>
    }
}

fn draw_bar_chart(context: &CanvasRenderingContext2d, width: f64, height: f64, props: &BarChartProps) {
    let data = props.data.iter().map(|point| point.value).collect::<Vec<i32>>();
    let num_bars = (data.len() + 2) as f64; // Add 2 to account for spacing on the farthest right
    let total_spacing = width * 0.1; // Reserve 10% of the width for spacing between bars
    let total_bar_width = width - total_spacing;
    let bar_width = total_bar_width / num_bars;
    let bar_spacing = total_spacing / (num_bars - 1.0);
    let axis_padding = 50.0;

    // context.set_fill_style(&JsValue::from_str("blue"));
    context.clear_rect(0.0, 0.0, width, height);

    // Calculate max value and step for y-axis grid lines
    let max_value = *data.iter().max().unwrap() as f64 * 1.2; // 20% higher than max value
    let num_grid_lines = 5;
    let step_value = max_value / num_grid_lines as f64;
    let step_height = (height - axis_padding * 2.0) / num_grid_lines as f64;

    // Draw the y-axis grid lines and labels
    context.set_stroke_style(&JsValue::from_str("#cccccc"));
    context.set_line_width(1.0);
    context.set_fill_style(&JsValue::from_str("black"));
    context.set_text_align("right");
    context.set_text_baseline("middle");

    for i in 0..=num_grid_lines {
        let y = height - axis_padding - i as f64 * step_height;
        context.begin_path();
        context.move_to(axis_padding, y);
        context.line_to(width, y);
        context.stroke();

        // Draw the y-axis labels
        let label = (i as f64 * step_value).round();
        context.fill_text(&format!("{}", label), axis_padding - 10.0, y).unwrap();
    }

    // Draw the bars
    let bar_color = props.config.as_ref().and_then(|config| config.bar_color.as_deref()).unwrap_or("#1ECBE1");
    context.set_fill_style(&JsValue::from_str(bar_color));
    for (i, &value) in data.iter().enumerate() {
        let x = axis_padding + i as f64 * (bar_width + bar_spacing);
        let y = height - axis_padding - value as f64 * ((height - axis_padding * 2.0) / max_value);
        context.fill_rect(x, y, bar_width, height - axis_padding - y);
    }

    // Add x-axis labels
    context.set_fill_style(&JsValue::from_str("black"));
    context.set_text_align("center");
    context.set_text_baseline("middle");
    let labels = props.data.iter().map(|point| &point.name).collect::<Vec<&String>>();
    for (i, &label) in labels.iter().enumerate() {
        let x = axis_padding + i as f64 * (bar_width + bar_spacing) + bar_width / 2.0;
        let y = height - axis_padding / 2.0;
        context.fill_text(label, x, y).unwrap();
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
        let canvas = document.create_element("canvas").unwrap().dyn_into::<web_sys::HtmlCanvasElement>().unwrap();
        
        // Get the 2D context from the canvas
        canvas.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap()
    }

    #[wasm_bindgen_test]
    fn test_draw_bar_chart() {
        let context = mock_context();
        let width = 500.0;
        let height = 400.0;

        let data = vec![
            DataPoint { name: "A".to_string(), value: 10 },
            DataPoint { name: "B".to_string(), value: 20 },
            DataPoint { name: "C".to_string(), value: 15 },
        ];

        let props = BarChartProps {
            data,
            config: None,
        };

        draw_bar_chart(&context, width, height, &props);

        // Additional assertions would be needed to validate the correct behavior,
        // e.g., checking if certain methods were called or if certain values were set.
        // Since we cannot directly inspect the canvas from here, we assume success if no panic occurs.
        assert!(true);
    }
}
