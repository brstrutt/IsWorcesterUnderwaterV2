mod screen_point;

use yew::prelude::*;

use self::screen_point::{Keyframe, ScreenPoint, get_keyframes_as_string};

#[function_component(Background)]
pub fn river_level_display_background() -> Html {

    let wave_keyframes = default_wave_keyframes();

    html! {
        <>
            <div class="full_canvas">
                <div 
                    class="full_canvas background_water"
                    color="aqua"
                    style={wave_keyframes}
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

fn default_wave_keyframes() -> String {
    let num_of_frames = 4;
    let num_of_points = 20;
    let left_height = 0.67;
    let right_height = 0.50;


    let base_keyframe = generate_wave(left_height, right_height, 0.0, num_of_points);

    let mut keyframes: Vec<Keyframe> = vec!();
    for i in 0..num_of_frames {
        keyframes.push(
            base_keyframe.clone()
            .iter()
            .map(|point| *point * (0.02 * i as f64 + 1.0))
            .collect());
    }

    "--Keyframe0:0% 75%, 100% 75%; --Keyframe1:0% 70%, 100% 70%;";
    get_keyframes_as_string(keyframes)
}

fn generate_wave(left_height: f64, right_height: f64, wave_height: f64, num_of_points: u32) -> Vec<ScreenPoint> {
    let base_line = interpolate_line(left_height, right_height, num_of_points);
    base_line
}

fn interpolate_line(left_height: f64, right_height: f64, num_of_points: u32) -> Vec<ScreenPoint> {
    let num_of_gaps = num_of_points as f64 - 1.0;
    let height_diff_between_sides = left_height - right_height;

    let height_diff_between_points = height_diff_between_sides / num_of_gaps;
    let width_diff_between_points = 1.0 / num_of_gaps;

    let mut points = vec!();
    for i in 0..num_of_points {
        let new_point_x = width_diff_between_points * i as f64;
        let new_point_y = (height_diff_between_points * i as f64) + left_height;
        points.push(ScreenPoint::new(new_point_x,new_point_y));
    }

    points
}
