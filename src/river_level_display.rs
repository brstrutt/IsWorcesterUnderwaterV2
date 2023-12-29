use yew::prelude::*;
use crate::flood_monitoring_api::LatestReading;

#[function_component(Background)]
pub fn river_level_display_background() -> Html {
    html! {
        <>
            <div class="full_canvas">
                <div class="full_canvas background_water" color="aqua"></div>
                <div class="full_canvas background_water_2" color="aqua"></div>
                <div class="full_canvas clipping_water ShowBelowWaterLevel" color="blue"></div>
            </div>
            <div class="full_canvas result_wrapper ShowAboveWaterLevel">
                <h1 class="result_content" style="color: black;">{"No"}</h1>
            </div>
            <div class="full_canvas result_wrapper ShowBelowWaterLevel">
                <h1 class="result_content" style="color: white;">{"Yes"}</h1>
            </div>
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct ForegroundProps {
    pub barbourne_last_reading: Option<LatestReading>
}

#[function_component(Foreground)]
pub fn foreground(ForegroundProps { barbourne_last_reading }: &ForegroundProps) -> Html {

    html! {
        <div>
            {
                match barbourne_last_reading {
                    Some(river_level) => format!("Level: {:?}", river_level.value),
                    _ => format!("Loading")
                }
            }
        </div>
    }
}
