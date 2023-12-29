use yew::prelude::*;
use crate::flood_monitoring_api::LatestReading;

#[derive(Properties, PartialEq)]
pub struct RiverLevelDisplayProps {
    pub barbourne_last_reading: Option<LatestReading>
}

#[function_component(RiverLevelDisplay)]
pub fn river_level_display(RiverLevelDisplayProps { barbourne_last_reading }: &RiverLevelDisplayProps) -> Html {

    html! {
        <div>
            {
                if barbourne_last_reading.is_some() {
                    format!("Level: {:?}", barbourne_last_reading.as_ref().unwrap().value)
                } else {
                    format!("Loading")
                }
            }
        </div>
    }
}
