use std::collections::HashMap;
use postcard::{from_bytes, to_allocvec};
use serde::{Serialize, Deserialize};
use std::fs::{read, write};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub balance: f64,
    pub experience: f64,
    pub cluck_level: u32,
    pub rluck_level: u32,
    pub inventory: Vec<Item>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    rarity: u64,
    class: u64,
    quality: f64,
}

impl Default for User {
    fn default() -> Self {
        User {
            balance: 0.0,
            experience: 0.0,
            cluck_level: 0,
            rluck_level: 0,
            inventory: vec![],
        }
    }
}

pub fn retrieve_database(db_path: &str) -> HashMap<u64, User> {
    let database = read(db_path).expect("Issue reading the file into a vector");
    from_bytes(&database).expect("Issue deserializing the database")
}

pub fn write_database(db_path: &str, new_data: HashMap<u64, User>) {
    let db_vec = to_allocvec(&new_data).expect("Issue serializing the database");
    write(db_path, db_vec).expect("There was an issue writing the database to the file");
}