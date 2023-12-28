use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer>
            <div>{ "This uses Environment Agency flood and river level data from the real-time data API (Beta)" }</div>
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
