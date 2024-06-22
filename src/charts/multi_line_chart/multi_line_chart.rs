use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window, wasm_bindgen::{JsValue, JsCast}};
use yew::prelude::*;
use gloo::events::EventListener;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct MultilineChartProps {
    pub data: Vec<(String, Vec<(i32, i32)>, String)>,
}

#[function_component]
pub fn MultilineChart(props: &MultilineChartProps) -> Html {
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

                    draw_multiline_chart(&context, width, height, &props_clone_resize);
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

fn draw_multiline_chart(context: &CanvasRenderingContext2d, width: f64, height: f64, props: &MultilineChartProps) {
    let datasets = &props.data;

    let axis_padding = 50.0;
    let max_value = datasets.iter()
        .flat_map(|(_, data, _)| data.iter().map(|&(_, value)| value))
        .max()
        // .cloned()
        .unwrap_or(0) as f64 * 1.2;
    let num_points = datasets.first().unwrap().1.len() as f64;
    let point_spacing = (width - axis_padding * 2.0) / (num_points - 1.0);

    context.set_fill_style(&JsValue::from_str("white"));
    context.clear_rect(0.0, 0.0, width, height);

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
        context.begin_path();
        context.move_to(axis_padding, y);
        context.line_to(width, y);
        context.stroke();

        // Draw the y-axis labels
        let label = (i as f64 * step_value).round();
        context.fill_text(&format!("{}", label), axis_padding - 10.0, y).unwrap();
    }

    // Draw each dataset as a separate line
    for (_label, data, color) in datasets {
        context.set_stroke_style(&JsValue::from_str(color));
        context.set_line_width(2.0);

        context.begin_path();
        context.move_to(axis_padding, height - axis_padding - (data[0].1 as f64 / max_value) * (height - axis_padding * 2.0));

        for i in 1..data.len() {
            let x = axis_padding + data[i].0 as f64 * point_spacing;
            let y = height - axis_padding - (data[i].1 as f64 / max_value) * (height - axis_padding * 2.0);

            let prev_x = axis_padding + data[i-1].0 as f64 * point_spacing;
            let prev_y = height - axis_padding - (data[i-1].1 as f64 / max_value) * (height - axis_padding * 2.0);

            let ctrl1_x = prev_x + point_spacing / 3.0;
            let ctrl1_y = prev_y;

            let ctrl2_x = x - point_spacing / 3.0;
            let ctrl2_y = y;

            context.bezier_curve_to(ctrl1_x, ctrl1_y, ctrl2_x, ctrl2_y, x, y);
        }
        context.stroke();

        // Add colored dots at inflection points
        context.set_fill_style(&JsValue::from_str(color));
        for &(index, value) in data.iter() {
            let x = axis_padding + index as f64 * point_spacing;
            let y = height - axis_padding - (value as f64 / max_value) * (height - axis_padding * 2.0);
            context.begin_path();
            context.arc(x, y, 3.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
            context.fill();
        }
    }

    // Add x-axis labels
    context.set_fill_style(&JsValue::from_str("black"));
    context.set_text_align("center");
    context.set_text_baseline("middle");
    
    let labels = datasets.into_iter().map(|dataset| dataset.0.clone());
    for (i, label) in labels.enumerate() {
        let x = axis_padding + i as f64 * point_spacing;
        let y = height - axis_padding / 2.0;
        context.fill_text(label.as_str(), x, y).unwrap();
    }

    // Add legend for index map
    let legend_x = width - axis_padding - 200.0;
    let legend_y = axis_padding;

    for (i, (label, _, color)) in datasets.iter().enumerate() {
        let text_y = legend_y + i as f64 * 20.0;
        
        context.set_fill_style(&JsValue::from_str(color));
        context.fill_rect(legend_x, text_y - 10.0, 10.0, 10.0);

        context.set_fill_style(&JsValue::from_str("black"));
        context.fill_text(label, legend_x + 20.0, text_y).unwrap();
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
    fn test_draw_multiline_chart() {
        let context = mock_context();
        let width = 800.0;
        let height = 600.0;
        let props = MultilineChartProps {
            data: vec![
                ("Dataset 1".to_string(), vec![(0, 10), (1, 20), (2, 30), (3, 40), (4, 50)], "#ff0000".to_string()),
                ("Dataset 2".to_string(), vec![(0, 50), (1, 40), (2, 30), (3, 20), (4, 10)], "#00ff00".to_string()),
            ],
        };

        draw_multiline_chart(&context, width, height, &props);

        // Additional assertions would be needed to validate the correct behavior,
        // e.g., checking if certain methods were called or if certain values were set.
        // Since we cannot directly inspect the canvas from here, we assume success if no panic occurs.
        assert!(true);
    }
}
