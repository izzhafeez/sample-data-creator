use std::error::Error;
use std::fs;
use serde_derive::{Deserialize, Serialize};
use crate::storage::storable::Storable;

#[derive(Deserialize, Serialize)]
pub struct Person {
  pub name: String,
}

impl Storable for Person {
    fn get() -> Result<Vec<Self>, Box<dyn Error>> {
        let persons: Vec<Person> = {
            let data = fs::read_to_string("./data/persons.json")
                .expect("Error");
            serde_json::from_str(&data).unwrap()
        };
        Ok(persons)
    }
}