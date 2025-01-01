use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            <div>{ "This uses Environment Agency flood and river level data from the real-time data API (Beta)" }</div>
            <div>
                <a href="https://github.com/brstrutt/IsWorcesterUnderwaterV2" target="_blank" rel="noopener noreferrer">
                    <img src="/assets/GitHub-Mark-Light-64px.png" title="Github" alt="Github Logo" width="12px" height="12px"/>
                </a>
            </div>
            <div class="float_right">
                <div>
                    { "Icons made by "}
                    <a href="https://www.flaticon.com/authors/catkuro" title="catkuro">
                        { "catkuro" }
                    </a>
                        { " from " }
                    <a href="https://www.flaticon.com/" title="Flaticon">
                        { "www.flaticon.com" }
                    </a>
                </div>
            </div>
        </footer>
    }
}
