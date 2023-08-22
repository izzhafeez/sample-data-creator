pub trait Location {
    fn get_lat(&self) -> f32;
    fn get_lng(&self) -> f32;
}