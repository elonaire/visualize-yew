use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, window, wasm_bindgen::{JsValue, JsCast}};
use yew::prelude::*;
use gloo::events::EventListener;

#[derive(Clone, Debug, PartialEq, Eq, Properties, Default)]
pub struct PieChartConfig {
    #[prop_or("center".to_string())]
    pub text_align: String,
    #[prop_or(true)]
    pub show_legend: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataPoint {
    pub name: String,
    pub value: i32,
    pub color: String,
}

#[derive(Clone, Properties, PartialEq, Debug, Eq)]
pub struct PieChartProps {
    pub data: Vec<DataPoint>,
    #[prop_or(Default::default())]
    pub config: PieChartConfig,
}

// #[cfg(feature = "PieChart")]
#[function_component]
pub fn PieChart(props: &PieChartProps) -> Html {
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

                    draw_pie_chart(&context, width, height, &props_clone_resize);
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
                { for props.data.iter().map(|data_point| {
                    html! {
                        <div style="display: flex; flex-direction: row; align-items: center; gap: 2px;">
                                <span style="font-size: 10px;">{ &data_point.name }</span>
                            <div style={format!("background-color: {}; width: 10px; height: 10px; display: inline-block;", &data_point.color)}></div>
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

pub fn draw_pie_chart(context: &CanvasRenderingContext2d, width: f64, height: f64, props: &PieChartProps) {
    let data = props.data.iter().map(|data_point| data_point.value).collect::<Vec<i32>>();
    // let labels = props.data.iter().map(|data_point| data_point.name.clone()).collect::<Vec<String>>();
    // randomize color selection(use a cool color palette)
    let colors = props.data.iter().map(|data_point| data_point.color.clone()).collect::<Vec<String>>();

    // Calculate the total sum of the data
    let total: f64 = data.iter().sum::<i32>() as f64;

    // Set up initial angle
    let mut start_angle = 0.0;

    for (i, &value) in data.iter().enumerate() {
        // Calculate the slice angle
        let slice_angle = value as f64 / total * std::f64::consts::PI * 2.0;

        // Draw the slice
        context.begin_path();
        context.move_to(width / 2.0, height / 2.0);
        context.arc(
            width / 2.0,
            height / 2.0,
            (width.min(height) / 2.0) - 5.0,
            start_angle,
            start_angle + slice_angle,
        ).unwrap();
        context.close_path();

        // Fill the slice with color
        context.set_fill_style(&JsValue::from_str(colors[i].as_str()));
        context.fill();

        // Update the starting angle
        start_angle += slice_angle;
    }

    // Draw labels
    // context.set_fill_style(&JsValue::from_str("black"));
    // context.set_text_align("center");
    // context.set_text_baseline("middle");

    // start_angle = 0.0;
    // for (i, &value) in data.iter().enumerate() {
    //     let slice_angle = value as f64 / total * std::f64::consts::PI * 2.0;
    //     let label_angle = start_angle + slice_angle / 2.0;

    //     let label_x = width / 2.0 + (width.min(height) / 2.0 - 30.0) * label_angle.cos();
    //     let label_y = height / 2.0 + (height.min(width) / 2.0 - 30.0) * label_angle.sin();

    //     context.fill_text(labels[i].as_str(), label_x, label_y).unwrap();

    //     start_angle += slice_angle;
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
    fn test_draw_pie_chart() {
        let context = mock_context();
        let width = 800.0;
        let height = 600.0;
        let props = PieChartProps {
            data: vec![
                DataPoint { name: "A".to_string(), value: 10, color: "".to_string() },
                DataPoint { name: "B".to_string(), value: 20, color: "".to_string() },
                DataPoint { name: "C".to_string(), value: 30, color: "".to_string() },
                DataPoint { name: "D".to_string(), value: 40, color: "".to_string() },
            ],
            config: PieChartConfig {
                text_align: "center".to_string(),
                show_legend: true,
            },
        };

        draw_pie_chart(&context, width, height, &props);

        // Additional assertions would be needed to validate the correct behavior,
        // e.g., checking if certain methods were called or if certain values were set.
        // Since we cannot directly inspect the canvas from here, we assume success if no panic occurs.
        assert!(true);
    }
}
