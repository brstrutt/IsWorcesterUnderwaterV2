use yew::prelude::*;
use crate::{flood_monitoring_api, promise::Promise};

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub barbourne_last_reading: Promise<flood_monitoring_api::LatestReading>,
    pub diglis_last_reading: Promise<flood_monitoring_api::LatestReading>
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
                        Promise::Loading => format!(": ?.???M"),
                        Promise::Error => format!(": ERROR!"),
                        Promise::Data(river_level) => format!(": {:?}M", river_level.value),
                    }
                }
            </div>
            <div>
                <a href="https://check-for-flooding.service.gov.uk/station/2039">
                    { "Diglis" }
                </a>
                {
                    match diglis_last_reading {
                        Promise::Loading => format!(": ?.???M"),
                        Promise::Error => format!(": ERROR!"),
                        Promise::Data(river_level) => format!(": {:?}M", river_level.value),
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
