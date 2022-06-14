use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    
     id: i32,   
     first_name: String,    
     last_name: String, 
     telephon_number: String,    
     address: String,
     country: String,   
     zip_code: String,   
     age: i32
}

#[derive(Debug, Serialize, Deserialize)]
struct Res {

    users:Vec<User>
}

fn main() {
    
    let mut users: Vec<User> = Vec::new();

   
  
    for n in 1 .. 5000001{
        //for n in 1 .. 11{
        users.push(User{
            id: 20,
            first_name: "José Luis".to_string(),
            last_name: "Juste Martín".to_string(),
            telephon_number: "627206369".to_string(),    
            address: "Calle Pizarro 18 Beti bloque B 5D".to_string(),
            country: "Spain".to_string(),   
            zip_code: "38109".to_string(),   
            age: 40
        });
    }
    let r = Res {
        
        users:users
    };
    let s = serde_json::to_string(&r).unwrap();
    let literal: &str = s.as_str();
    let mut file = File::create("C:\\ghz\\datos.json").unwrap();
    writeln!(&mut file, "{}", literal).unwrap();
    
}
