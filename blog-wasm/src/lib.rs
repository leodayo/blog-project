mod api;
mod app;
mod components;
mod dto;
mod error;
mod pages;
mod state;
mod storage;

use app::App;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App /> });
}
