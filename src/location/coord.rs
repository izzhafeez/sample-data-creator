use serde_derive::{Deserialize, Serialize};
use crate::location::location::Location;

#[derive(Debug, Serialize, Deserialize)]
pub struct Coord {
    lat: f32,
    lng: f32
}

impl Location for Coord {
    fn get_lat(&self) -> f32 {
        self.lat
    }

    fn get_lng(&self) -> f32 {
        self.lng
    }
}