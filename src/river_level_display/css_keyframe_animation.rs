use std::ops::{Add, Mul};

use euclid::{Point2D, UnknownUnit};

pub struct Animation(Vec<Keyframe>);

impl Animation {
    pub fn new(frames: Vec<Keyframe>) -> Animation {
        Animation {0: frames}
    }

    pub fn to_string(&self) -> String {
        let mut keyframes_string = String::new();

        for (i, keyframe) in self.0.iter().enumerate() {
            let keyframe_string = format!("--Keyframe{}:{};", i, keyframe.to_string());
            keyframes_string += keyframe_string.as_str();
    
            if keyframe != self.0.last().unwrap() {
                keyframes_string += " ";
            }
        }
    
        keyframes_string
    }
}

#[derive(PartialEq, Clone)]
pub struct Keyframe(Vec<ScreenPoint>);

impl Keyframe {
    pub fn new(point: Vec<ScreenPoint>) -> Keyframe {
        Keyframe {0: point}
    }

    pub fn map<B, F: FnMut(&ScreenPoint) -> B>(&self, f: F) -> Vec<B> {
        self.0.iter().map(f).collect()
    }

    fn to_string(&self) -> String {
        let mut keyframe_string = String::new();
    
        for point in &self.0 {
            let point_string = point.to_string();
            keyframe_string += point_string.as_str();
    
            if point != self.0.last().unwrap() {
                keyframe_string += ", ";
            }
        }
    
        keyframe_string
    }
}

#[derive(PartialEq, Clone)]
pub struct ScreenPoint(Point2D<f64, UnknownUnit>);

impl ScreenPoint {
    pub fn new(x: f64, y: f64) -> ScreenPoint {
        ScreenPoint {0: Point2D::new(x, y)}
    }

    fn to_string(&self) -> String {
        format!("{:?}% {:?}%", self.0.x * 100.0, self.0.y * 100.0)
    }
}

impl Add<f64> for ScreenPoint {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Self::new(self.0.x + other, self.0.y + other)
    }
}

impl Mul<f64> for ScreenPoint {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self::new(self.0.x * other, self.0.y * other)
    }
}
