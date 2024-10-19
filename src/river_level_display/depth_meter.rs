use yew::prelude::*;

#[function_component(RiverLevelMarkers)]
pub fn river_level_markers() -> Html {
    html! {
        <div class="depth_meter_container">
            <div class="left marker">
                <div class="depth_row">
                    <div class="block large"/>
                    <div class="height_label">{"5m"}</div>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block large"/>
                    <div class="height_label">{"4m"}</div>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block large"/>
                    <div class="height_label">{"3m"}</div>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block large"/>
                    <div class="height_label">{"2m"}</div>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block large"/>
                    <div class="height_label">{"1m"}</div>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block large"/>
                    <div class="height_label">{"0m"}</div>
                </div>
            </div>
            <div class="right marker">
                <div class="depth_row">
                    <div class="height_label">{"5m"}</div>
                    <div class="block large"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="height_label">{"4m"}</div>
                    <div class="block large"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="height_label">{"3m"}</div>
                    <div class="block large"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="height_label">{"2m"}</div>
                    <div class="block large"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="height_label">{"1m"}</div>
                    <div class="block large"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="block small"/>
                </div>
                <div class="depth_row">
                    <div class="height_label">{"0m"}</div>
                    <div class="block large"/>
                </div>
            </div>
        </div>
    }
}
