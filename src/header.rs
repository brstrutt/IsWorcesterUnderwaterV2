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
            <RiverLevelCounter
                link={"https://check-for-flooding.service.gov.uk/station/2092"}
                name={"Barbourne"}
                reading={*barbourne_last_reading}
            />
            <RiverLevelCounter
                link={"https://check-for-flooding.service.gov.uk/station/2039"}
                name={"Diglis"}
                reading={*diglis_last_reading}
            />
            <div class="float_right">
                <a href="https://check-for-flooding.service.gov.uk/location?q=worcester">
                    { "Live Flood Alerts" }
                </a>
            </div>
        </header>
    }
}

#[derive(Properties, PartialEq)]
pub struct RiverLevelCounterProps {
    pub link: &'static str,
    pub name: &'static str,
    pub reading: Promise<flood_monitoring_api::LatestReading>,
}

#[function_component(RiverLevelCounter)]
fn river_level_counter(RiverLevelCounterProps {link, name, reading}: &RiverLevelCounterProps) -> Html {
    html! {
        <div>
            <a href={*link}>
                {*name}
            </a>
            {
                match reading {
                    Promise::Loading => format!(": ?.???M"),
                    Promise::Error => format!(": ERROR!"),
                    Promise::Data(river_level) => format!(": {:?}M", river_level.value),
                }
            }
        </div>
    }
}
