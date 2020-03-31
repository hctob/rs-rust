extern crate serde;
extern crate serde_json;
extern crate reqwest;

use std::io::{stdout, Write};
use std::rc::Rc;
use std::sync::Arc;
use std::io;

use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;
use serde_json::{Result, Value, Error};

/*#[derive(Debug, Deserialize, Serialize)]
struct ItemInfo {
    item: String,
    icon: String,
    icon_large: String,
    _type: String,
    typeIcon: String,
    name: String,
    description: String,
    members: bool,
    trend: String,
    price: String,
    change: String,
    current: Vec<String>,
    today: Vec<String>,
    day30: Vec<String>,
    day90: Vec<String>,
    day180: Vec<String>
}*/

#[derive(Debug, Deserialize, Serialize)]
struct ItemInfo {
    //item: String,
    icon: String,
    icon_large: String,
    id: i32,
    _type: String,
    typeIcon: String,
    name: String,
    description: String,
    members: bool,
    trend: String,
    price: String,
    change: String,
    current: Vec<String>,
    today: Vec<String>,
    day30: Vec<String>,
    day90: Vec<String>,
    day180: Vec<String>
}
#[derive(Debug, Deserialize, Serialize)]
struct Item {
    item: HashMap<String, String>,
}

/*impl ItemInfo {
    fn new() -> ItemInfo {
        ItemInfo {
            item: String::new(),
            icon: String::new(),
            icon_large: String::new(),
            _type: String::new(),
            typeIcon: String::new(),
            name: String::new(),
            description: String::new(),
            members: false,
            trend: String::new(),
            price: String::new(),
            change: String::new(),
            current: Vec::new(),
            today: Vec::new(),
            day30: Vec::new(),
            day90: Vec::new(),
            day180: Vec::new(),
        }
    }
}*/

const BASE_URL: &str = "http://services.runescape.com/m=itemdb_oldschool/";

// Write the contents of rust-lang.org to stdout
fn main() -> Result<()> {
    //let mut easy = Easy::new();

    //easy.url("http://services.runescape.com/m=itemdb_oldschool/api/catalogue/detail.json?item=4151").unwrap();
    /*let fut = async {
        let url = "http://services.runescape.com/m=itemdb_oldschool/api/catalogue/detail.json?item=4151";
        let body = reqwest::get(url).await?
        .text()
        .await?;
        Ok::<String, io::Error>
    };*/
    let url = "http://services.runescape.com/m=itemdb_oldschool/api/catalogue/detail.json?item=4151";
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();//.text().unwrap();

    println!("Assembling whip");
    //println!("JSON: {}", body);

    let whip: Value = serde_json::from_str::<Value>(body.as_str()).unwrap_or_else(|e| {
        eprintln!("JSON: '{}'\nError: {}", body, e);
        ::std::process::exit(1);
    });
    println!("Item name: {}", whip["item"]["name"]);
    println!("Item price: {}", whip["item"]["current"]["price"]);
    println!("Item Description: {}", whip["item"]["description"]);
    //println!("Item trend: {:?}", whip["item"]["current"]);
    /*easy.write_function(|data| {
        *whip_clone = serde_json::from_slice(data).unwrap();
        let ret: usize = 0;
        Ok(ret)
    }).unwrap();
    easy.perform().unwrap();*/
    Ok(())
}
