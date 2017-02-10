extern crate json;
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io::prelude::*;

pub struct Pref{
    parsed: json::JsonValue
}

impl Pref{
    pub fn open() -> Pref{
        let mut file = File::open("res/data.json").unwrap();
        let mut file_data = String::new();
        file.read_to_string(&mut file_data).unwrap();
        Pref{
            parsed: json::parse(&file_data).unwrap()
        }
    }

    pub fn get_int(&self, k: &str) -> i32{
        self.parsed[k].as_i32().unwrap()
    }

    pub fn set_int(&mut self, k: &str, v: i32){
        self.parsed[k] = json::JsonValue::Number(json::number::Number::from(v));
    }

    pub fn get_str(&self, k: &str) -> &str{
        self.parsed[k].as_str().unwrap()
    }

    pub fn set_str(&mut self, k: &str, v: &str){
        self.parsed[k] = json::JsonValue::String(String::from(v));
    }

    pub fn save(&mut self){
        let mut options = OpenOptions::new();
        options.write(true);
        options.truncate(true);

        let mut file = options.open(Path::new("res/data.json")).unwrap();
        file.write_all(self.parsed.dump().as_bytes()).unwrap();

    }
}