use rand::Rng;

use super::css_keyframe_animation::{Animation, Keyframe, ScreenPoint};

pub struct WaveAnimation {
    animation: Animation
}

impl WaveAnimation {
    pub fn new(left_height: f64, right_height: f64) -> WaveAnimation {
        let num_of_points = 20;

        let base_keyframe = generate_wave_shape(left_height, right_height, 0.005, num_of_points);

        let num_of_gaps = num_of_points as f64 - 1.0;
        let translation_between_frames = ScreenPoint::new(1.0, left_height - right_height) / num_of_gaps;

        let num_of_frames = 4;
        let mut animation = vec!();
        let mut random_number_generator = rand::thread_rng();
        for i in 0..num_of_frames {
            let translation_between_frames = translation_between_frames.clone();
            
            let new_keyframe = base_keyframe.clone()
            .map(|point| point.clone() - translation_between_frames.clone() * i as f64)
            .map(|point| {
                let random_y_variation = (random_number_generator.gen::<f64>() - 0.5) / 500.0;
                point.clone() + ScreenPoint::new(0.0, random_y_variation)
            })
            .collect();

            let mut new_keyframe = Keyframe::new(new_keyframe);
            new_keyframe.move_offscreen_left_keyframes_onscreen_on_the_right(translation_between_frames);
            
            animation.push(new_keyframe);
        }

        WaveAnimation {animation: Animation::new(animation)}
    }

    pub fn to_string(&self) -> String {
        self.animation.to_string()
    }
}

fn generate_wave_shape(left_height: f64, right_height: f64, wave_height: f64, num_of_points: u32) -> Keyframe {
    let mut base_line = interpolate_line(left_height, right_height, num_of_points);

    for (i, point) in base_line.iter_mut().enumerate() {
        if i % 4 == 0 {
            *point.y() += wave_height;
        }
        else if i % 2 == 0 {
            *point.y() -= wave_height;
        }
    }

    base_line
}

fn interpolate_line(left_height: f64, right_height: f64, num_of_points: u32) -> Keyframe {
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

    Keyframe::new(points)
}