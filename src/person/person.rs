use serde_derive::{Deserialize, Serialize};
use crate::location::station::Station;
use crate::person::person_basic_information::PersonBasicInformation;

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    name: String,
    station: Station
}

impl Person {
    pub fn get() -> Vec<Person> {
        let pbi_vec: Vec<PersonBasicInformation> = PersonBasicInformation::get();
        pbi_vec
            .into_iter()
            .map(|pbi| Person {})
    }
}