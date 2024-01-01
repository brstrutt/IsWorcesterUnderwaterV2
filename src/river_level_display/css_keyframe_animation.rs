use std::{ops::{Add, Mul, Sub, AddAssign, Div}, slice::IterMut, iter::Map};

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

    pub fn map<B, F: FnMut(&ScreenPoint) -> B>(&self, f: F) -> Map<std::slice::Iter<ScreenPoint>, F> {
        self.0.iter().map(f)
    }

    pub fn iter_mut(&mut self) -> IterMut<ScreenPoint>{
        self.0.iter_mut()
    }

    pub fn move_offscreen_left_keyframes_onscreen_on_the_right(&mut self, translation: ScreenPoint) {
        let num_of_keyframes = self.0.len();
        if num_of_keyframes < 1 { return }
        while self.0.first_mut().unwrap().x() < &mut 0.0 {
            self.0.rotate_left(1);
            *(self.0.last_mut().unwrap()) += translation.clone() * (num_of_keyframes as f64 + 1.0);
        }
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

#[derive(PartialEq, Clone, Debug)]
pub struct ScreenPoint(Point2D<f64, UnknownUnit>);

impl ScreenPoint {
    pub fn new(x: f64, y: f64) -> ScreenPoint {
        ScreenPoint {0: Point2D::new(x, y)}
    }

    pub fn x(&mut self) -> &mut f64 {
        &mut self.0.x
    }

    pub fn y(&mut self) -> &mut f64 {
        &mut self.0.y
    }

    fn to_string(&self) -> String {
        format!("{:?}% {:?}%", self.0.x * 100.0, self.0.y * 100.0)
    }
}

impl Add<ScreenPoint> for ScreenPoint {
    type Output = Self;

    fn add(self, other: ScreenPoint) -> Self {
        Self::new(self.0.x + other.0.x, self.0.y + other.0.y)
    }
}

impl Add<f64> for ScreenPoint {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Self::new(self.0.x + other, self.0.y + other)
    }
}

impl AddAssign<ScreenPoint> for ScreenPoint {

    fn add_assign(&mut self, other: ScreenPoint) {
        self.0.x += other.0.x;
        self.0.y += other.0.y;
    }
}

impl Sub<ScreenPoint> for ScreenPoint {
    type Output = Self;

    fn sub(self, other: ScreenPoint) -> Self {
        Self::new(self.0.x - other.0.x, self.0.y - other.0.y)
    }
}

impl Sub<f64> for ScreenPoint {
    type Output = Self;

    fn sub(self, other: f64) -> Self {
        Self::new(self.0.x - other, self.0.y - other)
    }
}

impl Mul<f64> for ScreenPoint {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self::new(self.0.x * other, self.0.y * other)
    }
}

impl Div<f64> for ScreenPoint {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self::new(self.0.x / other, self.0.y / other)
    }
}
