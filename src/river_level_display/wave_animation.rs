use super::css_keyframe_animation::{Animation, Keyframe, ScreenPoint};

pub struct WaveAnimation {
    animation: Animation
}

impl WaveAnimation {
    pub fn new() -> WaveAnimation {
        let num_of_frames = 4;
        let num_of_points = 20;
        let left_height = 0.67;
        let right_height = 0.50;

        let base_keyframe = generate_wave(left_height, right_height, 0.02, num_of_points);

        let mut animation = vec!();
        for i in 0..num_of_frames {
            animation.push(
                Keyframe::new(
                    base_keyframe
                    .map(|point| point.clone() * (0.02 * i as f64 + 1.0))
                )
            );
        }
        WaveAnimation {animation: Animation::new(animation)}
    }

    pub fn to_string(&self) -> String {
        self.animation.to_string()
    }
}


fn generate_wave(left_height: f64, right_height: f64, wave_height: f64, num_of_points: u32) -> Keyframe {
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