use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            <div>{ "This uses Environment Agency flood and river level data from the real-time data API (Beta)" }</div>
            <div>
                <a href="https://github.com/brstrutt/IsWorcesterUnderwaterV2" target="_blank" rel="noopener noreferrer">
                    { "github source" }
                </a>
            </div>
        </footer>
    }
}
