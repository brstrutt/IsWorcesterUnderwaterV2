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
                match barbourne_last_reading {
                    Some(river_level) => format!("Level: {:?}", river_level.value),
                    _ => format!("Loading")
                }
            }
        </div>
    }
}
