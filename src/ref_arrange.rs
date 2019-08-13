use regex::Regex;
use chrono;
use chrono::prelude::*;
use crate::network;
use std::str::FromStr;

const TITLE_RE :Regex = Regex::from_str("<title>(.*?)</title>").unwrap();
const WRITE_DATA_RE :Regex = Regex::from_str("\\d{4}(\\-|\\/)\\d{1,2}\\1\\d{1,2}").unwrap();

fn web_ref(url:&str) ->(){

    let item = network::get_web(url);
}


struct RefInfo{
    title:String,
    url:String,
    read_data:String,
    author:String,
    write_data:String,
    web_type:WebType,
    publisher:String,
    language:Language,
    archive_url:String,
}


#[derive(Copy, Clone, Eq, PartialEq)]
enum WebType{
    Html,
    Pdf,
    Mime,
    //todo add more.
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Language{
    English,
    ChineseHans,
    ChineseTw,
    Japanese,
    French,
    Russian,
    //todo add more
}
const AUTHOR_RE:Regex = Regex::from_str("(?:\\w*?\\.)*(\\w*?)\\.\\w*").unwrap();
fn get_author(domain:String) -> String{
    let author_name:String;
    for each in AUTHOR_RE.captures_iter(domain.as_str()){
        author_name = each.get(1).unwrap().as_str().to_string();
    }
    match domain.as_str() {
        "xinhuanet" => "新华网".to_string(),
        _ => domain
    }
}