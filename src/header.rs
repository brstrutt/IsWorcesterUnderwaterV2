use yew::prelude::*;
use crate::flood_monitoring_api::LatestReading;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub barbourne_last_reading: Option<LatestReading>,
    pub diglis_last_reading: Option<LatestReading>
}

#[function_component(Header)]
pub fn header(HeaderProps {barbourne_last_reading, diglis_last_reading}: &HeaderProps) -> Html {
    html! {
        <header>
            <div>
                <a href="https://check-for-flooding.service.gov.uk/station/2092">
                    { "Barbourne" }
                </a>
                {
                    match barbourne_last_reading {
                        Some(river_level) => format!(": {:?}M", river_level.value),
                        _ => format!(": ?.???M")
                    }
                }
            </div>
            <div>
                <a href="https://check-for-flooding.service.gov.uk/station/2039">
                    { "Diglis" }
                </a>
                {
                    match diglis_last_reading {
                        Some(river_level) => format!(": {:?}M", river_level.value),
                        _ => format!(": ?.???M")
                    }
                }
            </div>
            <div class="float_right">
                <a href="https://check-for-flooding.service.gov.uk/location?q=worcester">
                    { "Live Flood Alerts" }
                </a>
            </div>
        </header>
    }
}
