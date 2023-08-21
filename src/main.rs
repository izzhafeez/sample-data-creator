// src/main.rs

mod storage;

use storage::person::Person;
use crate::storage::storable::Storable;

fn main() {

  // Deserialize the JSON string into a Vec<Person>
  let persons: Vec<Person> = Person::get().unwrap();

  // Print the deserialized people
  for person in persons {
    println!("Name: {}", person.name);
  }
}
