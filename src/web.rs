use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use crate::app::MarsApp;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
pub async fn start(canvas_id : &str) -> Result<(), JsValue> {
	let window = web_sys::window().expect("There's no window???");
	let document = window.document().expect("Window has no document???");

	let canvas = document
		.get_element_by_id(canvas_id).expect("Canvas ID doesn't match any HTML element??")
		.dyn_into::<HtmlCanvasElement>()?;

	eframe::WebRunner::new()
		.start(
			canvas,
			eframe::WebOptions::default(),
			Box::new(|_cc| Ok(Box::new(MarsApp::new()))),
		)
		.await
}
