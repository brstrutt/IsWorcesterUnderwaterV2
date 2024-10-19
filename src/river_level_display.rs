mod css_keyframe_animation;
mod wave_animation;

use std::string;

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

    let wave_animation = use_memo(
        (*barbourne_last_reading, *diglis_last_reading), 
        |(barbourne_reading, diglis_reading)| -> WaveAnimation {
            let barbourne_flood_percentage = get_flood_percentage(barbourne_reading.value as f64, 2.0, 4.0);
            let diglis_flood_percentage = get_flood_percentage(diglis_reading.value as f64, 1.5, 3.3);
            WaveAnimation::new(1.0 - barbourne_flood_percentage, 1.0 - diglis_flood_percentage)
        }
    );

    html! {
        <>
            <div
                class="full_canvas"
                style={wave_animation.to_string()}
            >
                <Waves/>
                <SolubleText
                    shown_above_water="No"
                    shown_below_water="Yes"
                />
            </div>
        </>
    }
}

#[function_component(Waves)]
fn waves() -> Html {
    html! {
        <>
            <div
                class="full_canvas background_water"
                color="aqua"
            />
            <div 
                class="full_canvas background_water_2"
                color="aqua"
            />
            <div 
                class="full_canvas clipping_water ShowBelowWaterLevel"
                color="blue"
            />
        </>
    }
}


#[derive(Properties, PartialEq)]
pub struct SolubleTextProps {
    pub shown_above_water: String,
    pub shown_below_water: String
}

#[function_component(SolubleText)]
fn soluble_text(SolubleTextProps {shown_above_water, shown_below_water}: &SolubleTextProps) -> Html {
    html! {
        <>
            <div
                class="full_canvas result_wrapper ShowAboveWaterLevel"
            >
                <h1
                    class="result_content"
                    style="color: black;"
                >
                    {shown_above_water}
                </h1>
            </div>
            <div
                class="full_canvas result_wrapper ShowBelowWaterLevel"
            >
                <h1
                    class="result_content" 
                    style="color: white;"
                >
                    {shown_below_water}
                </h1>
            </div>
        </>
    }
}

fn get_flood_percentage(current_level: f64, normal_level: f64, risky_level: f64) -> f64 {
	let quarter = risky_level - normal_level; // Calculate what 25% of the range is
	let base = normal_level - quarter; // Calculate what value 0% would be, aka the base of the range

	let scaled_level = current_level - base;
	let percentage_multiplier = 1.0 / (4.0 * quarter);
	let percentage = scaled_level * percentage_multiplier;
	percentage.min(1.0).max(0.0)
}
