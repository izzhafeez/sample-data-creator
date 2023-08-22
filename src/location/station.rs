use std::collections::HashMap;
use std::fs;
use serde_derive::{Deserialize, Serialize};
use crate::location::coord::Coord;
use crate::location::location::Location;

#[derive(Debug, Serialize, Deserialize)]
pub struct Station {
    name: String,
    #[serde(flatten)]
    coord: Coord
}

impl Location for Station {
    fn get_lat(&self) -> f32 {
        self.coord.get_lat()
    }

    fn get_lng(&self) -> f32 {
        self.coord.get_lng()
    }
}

impl Station {
    pub fn get() -> HashMap<String, Station> {
        let file: String = fs::read_to_string("./data/stations.json")
            .expect("Cannot read file.");
        Station::parse(&file)
    }

    fn parse(string: &str) -> HashMap<String, Station> {
        let station_vec: Vec<Station> = serde_json::from_str(string)
            .expect("Station vec could not be parsed.");
        station_vec
            .into_iter()
            .map(|station| (station.name.clone(), station))
            .collect()
    }
}