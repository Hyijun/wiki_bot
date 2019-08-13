use reqwest;
use std::io::Read;

pub fn get_web(url:&str) -> String{
    let mut html = reqwest::get(url).unwrap();
    let mut string = String::new();
    html.read_to_string(& mut string);
    string
}