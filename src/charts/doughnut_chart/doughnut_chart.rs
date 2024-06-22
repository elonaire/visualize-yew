use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window, wasm_bindgen::{JsValue, JsCast}};
use yew::prelude::*;
use gloo::events::EventListener;
use std::f64::consts::PI;

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct DoughnutChartConfigs {

}

#[derive(Clone, Properties, PartialEq, Debug, Eq)]
pub struct DoughnutChartProps {
    pub data: Vec<(String, i32, String)>,
    pub config: Option<DoughnutChartConfigs>,
}
#[function_component]
pub fn DoughnutChart(props: &DoughnutChartProps) -> Html {
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

                    draw_doughnut_chart(&context, width, height, &props_clone_resize);
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

fn draw_doughnut_chart(context: &CanvasRenderingContext2d, width: f64, height: f64, props: &DoughnutChartProps) {
    let center_x = width / 2.0;
    let center_y = height / 2.0;
    let radius = (width.min(height) / 3.0).min(150.0);
    let inner_radius = radius * 0.5;

    // Define the segments of the doughnut chart
    let segments = &props.data;

    let total: f64 = segments.iter().map(|(_, value, _)| *value as f64).sum();

    // Draw each segment of the doughnut chart
    let mut start_angle = -PI / 2.0;

    for (_label, value, color) in segments {
        let sweep_angle = (*value as f64 / total) * 2.0 * PI;
        let end_angle = start_angle + sweep_angle;

        context.begin_path();
        context.set_fill_style(&JsValue::from_str(color.as_str()));
        context.move_to(center_x, center_y);
        let _ignored_result = context.arc(center_x, center_y, radius, start_angle, end_angle);
        context.line_to(center_x, center_y);
        context.fill();
        context.close_path();

        // Outline the segment
        context.begin_path();
        context.set_stroke_style(&JsValue::from_str("white"));
        context.set_line_width(2.0);
        context.move_to(center_x, center_y);
        let _ignored_result = context.arc(center_x, center_y, radius, start_angle, end_angle);
        context.line_to(center_x, center_y);
        context.stroke();
        context.close_path();

        // Draw inner hole
        context.begin_path();
        let _ignored_result = context.set_global_composite_operation("destination-out");
        let _ignored_result = context.arc(center_x, center_y, inner_radius, 0.0, 2.0 * PI);
        context.fill();
        context.close_path();
        let _ignored_result = context.set_global_composite_operation("source-over");

        start_angle = end_angle;
    }

    // Add labels
    start_angle = -PI / 2.0;
    context.set_fill_style(&JsValue::from_str("black"));
    context.set_text_align("center");
    context.set_text_baseline("middle");

    for (label, value, _) in segments {
        let sweep_angle = (*value as f64 / total) * 2.0 * PI;
        let angle: f64 = start_angle + sweep_angle / 2.0;

        let x = center_x + (radius + 20.0) * angle.cos();
        let y = center_y + (radius + 20.0) * angle.sin();

        let _ignored_result = context.fill_text(label.as_str(), x, y);

        start_angle += sweep_angle;
    }
}
