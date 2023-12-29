use yew::prelude::*;

mod flood_monitoring_api;
mod header;
mod footer;
mod river_level_display;

#[hook]
fn use_river_level(monitoring_station_id: i32) -> UseStateHandle<Option<flood_monitoring_api::LatestReading>> {
    let river_level = use_state(|| Option::<flood_monitoring_api::LatestReading>::None);
    {
        let river_level = river_level.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let result = flood_monitoring_api::get_river_levels(monitoring_station_id).await;
    
                if result.is_ok() {
                    match result.unwrap().items.first() {
                        Some(first_data) => river_level.set(Some(first_data.latestReading)),
                        _ => (),
                    }
                }
                else {
                    log::error!("Failed to retreive river level data: {:?}", result);
                }
            });
            || ()
        });
    }
    river_level
}

#[function_component(App)]
fn app() -> Html {
    let barbourne_river_level = use_river_level(2642);
    html! {
        <div class="content_overlay">
            <header::Header/>
            <main>
                <river_level_display::RiverLevelDisplay 
                    barbourne_last_reading={*barbourne_river_level}
                />
            </main>
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
