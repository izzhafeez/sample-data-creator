use std::collections::HashMap;
use std::sync::Arc;
use serde_derive::{Deserialize, Serialize};
use crate::location::station::Station;
use crate::person::person_basic_information::PersonBasicInformation;

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    name: String,
    station: Station
}

impl Person {
    pub fn get(station_map: HashMap<String, Station>) -> Vec<Person> {
        let pbi_vec: Vec<PersonBasicInformation> = PersonBasicInformation::get();
        for pbi in pbi_vec.into_iter() {
            let name: String = pbi.name;
            let station_name: String = pbi.station_name;
            let station: Station = *station_map.get(&station_name)?.clone();

        }
    }
}