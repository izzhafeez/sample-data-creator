use std::fs;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PersonBasicInformation {
    name: String,
    station_name: String
}

impl PersonBasicInformation {
    pub fn get() -> Vec<Self> {
        let file: String = fs::read_to_string("./data/person_basic_information.json")
            .expect("Cannot read file.");
        PersonBasicInformation::parse(&file)
    }

    fn parse(string: &str) -> Vec<Self> {
        serde_json::from_str(string)
            .expect("Basic Information could not be parsed.")
    }
}