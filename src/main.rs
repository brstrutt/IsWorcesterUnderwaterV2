use yew::prelude::*;

mod flood_monitoring_api;
mod page;
mod header;
mod river_level_display;
mod footer;

#[function_component(App)]
fn app() -> Html {
    html! { <page::Page/> }
}

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    log::info!("App is starting");

    yew::Renderer::<App>::new().render();
}
