use yew::prelude::*;

#[function_component(Background)]
pub fn river_level_display_background() -> Html {
    html! {
        <>
            <div class="full_canvas">
                <div class="full_canvas background_water" color="aqua"></div>
                <div class="full_canvas background_water_2" color="aqua"></div>
                <div class="full_canvas clipping_water ShowBelowWaterLevel" color="blue"></div>
            </div>
            <div class="full_canvas result_wrapper ShowAboveWaterLevel">
                <h1 class="result_content" style="color: black;">{"No"}</h1>
            </div>
            <div class="full_canvas result_wrapper ShowBelowWaterLevel">
                <h1 class="result_content" style="color: white;">{"Yes"}</h1>
            </div>
        </>
    }
}
