use yew::prelude::*;
use super::flood_monitoring_api;
use super::river_level_display;
use super::header::Header;
use super::footer::Footer;

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

#[function_component(Page)]
pub fn page() -> Html {
    let barbourne_river_level = use_river_level(2642);
    html! {
        <>
            <river_level_display::Background/> 
            <div class="content_overlay">
                <Header/>
                <main>
                    <river_level_display::Foreground 
                        barbourne_last_reading={*barbourne_river_level}
                    />
                </main>
                <Footer/>
            </div>
        </>
    }
}
