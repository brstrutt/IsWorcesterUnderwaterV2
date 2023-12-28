use yew::prelude::*;

mod flood_monitoring_api;
mod header;
mod footer;

#[function_component(App)]
fn app() -> Html {
    let river_level = use_state(|| Option::<flood_monitoring_api::Response>::None);
    {
        let river_level = river_level.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let result = flood_monitoring_api::get_river_levels(2642).await;
    
                if result.is_ok() {
                    river_level.set(Option::Some(result.unwrap()));
                }
                else {
                    log::error!("Failed to retreive river level data: {:?}", result);
                }
            });
            || ()
        });
    }

    html! {
        <div>
            <header::Header/>
            <div>{
                if river_level.is_some() {
                    format!("Level: {:?}", river_level.as_ref().unwrap().items[0].latestReading.value)
                } else {
                    format!("Loading")
                }
            }</div>
            <footer::Footer/>
        </div>
    }
}

fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    log::info!("App is starting");

    yew::Renderer::<App>::new().render();
}
