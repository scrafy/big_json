use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    
     id: i32,   
     first_name: String,    
     last_name: String, 
     telephon_numer: String,    
     address: String,
     country: String,   
     zip_code: String,   
     age: i32
}

fn main() {
    
    let mut users: Vec<User> = Vec::new();

    let mut u = User{
        id: 1,
        first_name: "José Luis".to_string(),
        last_name: "Juste Martín".to_string(),
        telephon_numer: "627206369".to_string(),    
        address: "Calle Pizarro 18 Beti bloque B 5D".to_string(),
        country: "Spain".to_string(),   
        zip_code: "38109".to_string(),   
        age: 40
    };
  
    for n in 1 .. 10000001{
        //for n in 1 .. 11{
        users.push(User{
            id: 20,
            first_name: "José Luis".to_string(),
            last_name: "Juste Martín".to_string(),
            telephon_numer: "627206369".to_string(),    
            address: "Calle Pizarro 18 Beti bloque B 5D".to_string(),
            country: "Spain".to_string(),   
            zip_code: "38109".to_string(),   
            age: 40
        });
    }
    let s = serde_json::to_string(&users).unwrap();
    let literal: &str = s.as_str();
    let mut file = File::create("datos.json").unwrap();
    writeln!(&mut file, "{}", literal).unwrap();
    
}
