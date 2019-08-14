use crate::network;
use json;
use rustc_serialize::json::Json;


#[derive(Clone, Eq, PartialEq)]
pub struct RefInfo{
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

impl RefInfo{
    pub fn new() -> RefInfo{
        RefInfo{
            title:String::new(),
            url:String::new(),
            read_data:String::new(),
            author:String::new(),
            write_data:String::new(),
            web_type:WebType::Html,
            publisher:String::new(),
            language:Language::ChineseHans,
            archive_url:String::new(),
        }
    }
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
