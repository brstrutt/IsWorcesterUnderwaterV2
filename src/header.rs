use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header>
            <div>
                { "Barbourne: ???M" }
            </div>
            <div>
                { "Diglis: ???M" }
            </div>
            <div>
                { "Live Flood Alerts" }
            </div>
        </header>
    }
}
