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
            <div class="float_right">
                { "Live Flood Alerts" }
            </div>
        </header>
    }
}
