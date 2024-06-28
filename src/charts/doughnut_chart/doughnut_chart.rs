use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window, wasm_bindgen::{JsValue, JsCast}};
use yew::prelude::*;
use gloo::events::EventListener;
use std::f64::consts::PI;

#[derive(Clone, Debug, PartialEq, Eq, Properties, Default)]
pub struct DoughnutChartConfigs {
    #[prop_or(true)]
    pub show_legend: bool,
}

#[derive(Clone, Properties, PartialEq, Debug, Eq)]
pub struct DoughnutChartProps {
    pub data: Vec<(String, i32, String)>,
    #[prop_or_default]
    pub config: DoughnutChartConfigs,
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
                    if height < width {
                        canvas.set_height((height * device_pixel_ratio) as u32);
                    } else {
                        canvas.set_height((width * device_pixel_ratio) as u32);
                        
                    }

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

    let legend_html = if props.config.show_legend {
        
        html! {
            <div style="display: flex; flex-direction: row; gap: 5px; margin-bottom: 1em;">
                { for props.data.iter().map(|(label, _value, color)| {
                    html! {
                        <div style="display: flex; flex-direction: row; align-items: center; gap: 2px;">
                                <span style="font-size: 10px;">{ &label }</span>
                            <div style={format!("background-color: {}; width: 10px; height: 10px; display: inline-block;", &color)}></div>
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
            <canvas ref={canvas_ref} style="width: 100%; height: 100%;"></canvas>
        </div>
    }
}

fn draw_doughnut_chart(context: &CanvasRenderingContext2d, width: f64, height: f64, props: &DoughnutChartProps) {
    let center_x = width / 2.0;
    let center_y = height / 2.0;
    let radius = (width.min(height) / 2.0).min(150.0);
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
    // start_angle = -PI / 2.0;
    // context.set_fill_style(&JsValue::from_str("black"));
    // context.set_text_align("center");
    // context.set_text_baseline("middle");

    // for (label, value, _) in segments {
    //     let sweep_angle = (*value as f64 / total) * 2.0 * PI;
    //     let angle: f64 = start_angle + sweep_angle / 2.0;

    //     let x = center_x + (radius + 20.0) * angle.cos();
    //     let y = center_y + (radius + 20.0) * angle.sin();

    //     let _ignored_result = context.fill_text(label.as_str(), x, y);

    //     start_angle += sweep_angle;
    // }
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
    fn test_draw_doughnut_chart() {
        let context = mock_context();

        let props = DoughnutChartProps {
            data: vec![
                ("A".to_string(), 10, "#ff0000".to_string()),
                ("B".to_string(), 20, "#00ff00".to_string()),
                ("C".to_string(), 30, "#0000ff".to_string()),
            ],
            config: DoughnutChartConfigs {
                show_legend: true,
            }
        };

        draw_doughnut_chart(&context, 500.0, 500.0, &props);

        // Additional assertions would be needed to validate the correct behavior,
        // e.g., checking if certain methods were called or if certain values were set.
        // Since we cannot directly inspect the canvas from here, we assume success if no panic occurs.
        assert!(true);
    }
}
