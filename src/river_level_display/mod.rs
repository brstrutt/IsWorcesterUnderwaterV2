mod css_keyframe_animation;
mod wave_animation;

use yew::prelude::*;

use self::wave_animation::WaveAnimation;


#[function_component(Background)]
pub fn river_level_display_background() -> Html {

    let wave_animation = WaveAnimation::new();

    html! {
        <>
            <div class="full_canvas">
                <div 
                    class="full_canvas background_water"
                    color="aqua"
                    style={wave_animation.to_string()}
                />
            </div>
            <div 
                class="full_canvas result_wrapper"
                style="color: black; --Keyframe0 animation: waves 10s linear infinite;"
            >
                <h1
                    class="result_content"
                    style="color: black;"
                >
                    {"No"}
                </h1>
            </div>
            <div class="full_canvas result_wrapper ShowBelowWaterLevel">
                <h1 class="result_content" style="color: white;">{"Yes"}</h1>
            </div>
        </>
    }
}
