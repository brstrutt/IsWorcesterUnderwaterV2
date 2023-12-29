use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header>
            <div>
                <a href="https://check-for-flooding.service.gov.uk/station/2092">
                    { "Barbourne" }
                </a>
                { ": ?.???M" }
            </div>
            <div>
                <a href="https://check-for-flooding.service.gov.uk/station/2039">
                    { "Diglis" }
                </a>
                { ": ?.???M" }
            </div>
            <div class="float_right">
                <a href="https://check-for-flooding.service.gov.uk/location?q=worcester">
                    { "Live Flood Alerts" }
                </a>
            </div>
        </header>
    }
}
