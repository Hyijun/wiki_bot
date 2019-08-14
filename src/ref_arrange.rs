///**
//*NOTICE 此代码已废弃！
//*已转用现成API
//*
//*
//**/
//
//如需启用，请删除这句话；
//
//
//
//use regex::Regex;
//use chrono;
//use chrono::prelude::*;
//use crate::network;
//use std::str::FromStr;
//
//
//pub fn web_ref(url:&str) ->RefInfo{
//    let title_re :Regex = Regex::from_str("<title>(.*?)</title>").unwrap();
//    let write_data_re :Regex = Regex::from_str("(\\d{4}(?:-|/)\\d{1,2}(?:-|/)\\d{1,2})").unwrap();
//    let mut ref_info = RefInfo::new();
//    let mut item :String= network::get_web(url);
//
//    ref_info.read_data = chrono::Utc::today().to_string().replace("UTC", "");
//    println!("{}", now_time);
//    item = item.replace("\n", "");
//
//    let title = get_re_result(title_re, &item);
//    let write_data = get_re_result(write_data_re, &item);
//
//    if &url[url.len()-4..] == ".pdf"{
//        ref_info.web_type = WebType::Pdf
//    }
//
//    println!("{}", title);
//    println!("{}", write_data);
//
//
//    let mut times = 0;
//
//    ref_info
//}
//
//#[derive(Clone, Eq, PartialEq)]
//pub struct RefInfo{
//    title:String,
//    url:String,
//    read_data:String,
//    author:String,
//    write_data:String,
//    web_type:WebType,
//    publisher:String,
//    language:Language,
//    archive_url:String,
//}
//
//impl RefInfo{
//    pub fn new() -> RefInfo{
//        RefInfo{
//            title:String::new(),
//            url:String::new(),
//            read_data:String::new(),
//            author:String::new(),
//            write_data:String::new(),
//            web_type:WebType::Html,
//            publisher:String::new(),
//            language:Language::ChineseHans,
//            archive_url:String::new(),
//        }
//    }
//}
//
//
//#[derive(Copy, Clone, Eq, PartialEq)]
//enum WebType{
//    Html,
//    Pdf,
//    Mime,
//    //todo add more.
//}
//
//#[derive(Copy, Clone, Eq, PartialEq)]
//enum Language{
//    English,
//    ChineseHans,
//    ChineseTw,
//    Japanese,
//    French,
//    Russian,
//    //todo add more
//}
//fn get_author(domain:String) -> String{
//    let author_re:Regex = Regex::from_str("(?:\\w*?\\.)*(\\w*?)\\.\\w*").unwrap();
//    let mut author_name:String;
//    for each in author_re.captures_iter(domain.as_str()){
//        author_name = each.get(1).unwrap().as_str().to_string();
//    }
//    let result =
//    match domain.as_str() {
//        "xinhuanet" => "新华网",
//        _ => "None"
//    };
//    result.to_string()
//}
//
//fn get_re_result(re: Regex, match_item: &str) -> String{
//    let mut resule = String::new();
//    for each in re.captures_iter(match_item) {
//       resule = each.get(1).unwrap().as_str().to_string()
//    }
//    resule
//}