mod css_keyframe_animation;
mod wave_animation;
mod waves;
mod depth_meter;
mod height_to_screenspace;

use yew::prelude::*;

use crate::flood_monitoring_api::LatestReading;

use self::wave_animation::WaveAnimation;

#[derive(Properties, PartialEq)]
pub struct BackgroundProps {
    pub barbourne_last_reading: LatestReading,
    pub diglis_last_reading: LatestReading
}

#[function_component(Background)]
pub fn river_level_display_background(BackgroundProps {barbourne_last_reading, diglis_last_reading}: &BackgroundProps) -> Html {
    let barbourne_range = height_to_screenspace::HeightRange{normal_level: 2.0, risky_level: 4.0};
    let diglis_range = height_to_screenspace::HeightRange{normal_level: 1.5, risky_level: 3.3};

    let wave_animation = use_memo(
        (*barbourne_last_reading, *diglis_last_reading), 
        |(barbourne_reading, diglis_reading)| -> WaveAnimation {
            let barbourne_flood_percentage = height_to_screenspace::get_flood_percentage(barbourne_reading.value as f64, barbourne_range);
            let diglis_flood_percentage = height_to_screenspace::get_flood_percentage(diglis_reading.value as f64, diglis_range);
            WaveAnimation::new(1.0 - barbourne_flood_percentage.min(1.0).max(0.0), 1.0 - diglis_flood_percentage.min(1.0).max(0.0))
        }
    );

    html! {
        <>
            <div
                class="full_canvas"
                style={wave_animation.to_string()}
            >
                <waves::Waves/>
                <depth_meter::RiverLevelMarkers/>
                <waves::SolubleText
                    shown_above_water="No"
                    shown_below_water="Yes"
                />
            </div>
        </>
    }
}
