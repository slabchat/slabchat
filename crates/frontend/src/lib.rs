use sycamore::builder::component;
use sycamore::component;
use sycamore::prelude::{Scope, View};
use sycamore::web::html::*;
use sycamore::web::Html;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
	div().class("content").c(h1().t("SlabChat")).view(cx)
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
	// print pretty errors in wasm https://github.com/rustwasm/console_error_panic_hook
	console_error_panic_hook::set_once();

	tracing_wasm::set_as_global_default();

	tracing::info!("Hello world!");

	sycamore::render(|cx| component(|| App(cx, ())));

	Ok(())
}
