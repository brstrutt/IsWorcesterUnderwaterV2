use euclid::{Point2D, UnknownUnit};

pub type ScreenPoint = Point2D<f64, UnknownUnit>;
pub type Keyframe = Vec<ScreenPoint>;

pub fn get_keyframes_as_string(key_frames: Vec<Keyframe>) -> String {
    let mut keyframes_string = String::new();

    for (i, keyframe) in key_frames.iter().enumerate() {
        let keyframe_string = get_keyframe_string(i, keyframe.clone());
        keyframes_string += keyframe_string.as_str();

        if keyframe != key_frames.last().unwrap() {
            keyframes_string += " ";
        }
    }

    keyframes_string
}

fn get_keyframe_string(index: usize, path: Keyframe) -> String {
    let mut keyframe_string = format!("--Keyframe{:?}:", index);

    for point in &path {
        let point_string = get_screen_point_string(*point);
        keyframe_string += point_string.as_str();

        if point != path.last().unwrap() {
            keyframe_string += ", ";
        }
    }

    keyframe_string += ";";

    keyframe_string
}

fn get_screen_point_string(point: ScreenPoint) -> String {
    format!("{:?}% {:?}%", point.x * 100.0, point.y * 100.0)
}
