#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod cache;
mod extensions;
mod views;

use crate::app::App;

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    use image::GenericImageView;

    let icon_data = {
        let image = image::load_from_memory(include_bytes!("../assets/icon.ico"))
            .expect("Failed to load icon image");
        let rgba = image.to_rgba8().into_vec();
        let (width, height) = image.dimensions();
        eframe::egui::IconData {
            rgba,
            width,
            height,
        }
    };

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_min_inner_size([510.0, 430.0])
            .with_inner_size([510.0, 430.0])
            .with_icon(icon_data),
        ..Default::default()
    };

    eframe::run_native(
        "Kallel's Utilities",
        options,
        Box::new(|cc| Ok(Box::new(App::default(cc)))),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(App::default(cc)))),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}
