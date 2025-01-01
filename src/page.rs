use yew::prelude::*;
use super::flood_monitoring_api;
use super::river_level_display;
use super::header::Header;
use super::footer::Footer;
use super::promise::Promise;

#[hook]
fn use_river_level(monitoring_station_id: i32) -> UseStateHandle<Promise<flood_monitoring_api::LatestReading>> {
    let river_level = use_state(|| Promise::Loading);
    {
        let river_level = river_level.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match flood_monitoring_api::get_latest_river_level(monitoring_station_id).await {
                    Ok(data) => river_level.set(Promise::Data(data)),
                    Err(error) => {
                        log::error!("Failed to retreive river level data: {:?}", error);
                        river_level.set(Promise::Error);
                    }
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
    let diglis_river_level = use_river_level(2085);

    html! {
        <>
            if !barbourne_river_level.is_loading() && !diglis_river_level.is_loading() {
                <river_level_display::Background
                    barbourne_last_reading={barbourne_river_level.as_result()}
                    diglis_last_reading={diglis_river_level.as_result()}
                />
            }
            <div class="content_overlay">
                <Header
                    barbourne_last_reading={*barbourne_river_level}
                    diglis_last_reading={*diglis_river_level}
                />
                <main/>
                <Footer/>
            </div>
        </>
    }
}
