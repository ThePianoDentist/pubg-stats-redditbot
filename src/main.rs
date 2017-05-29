extern crate reqwest;
extern crate pretty_env_logger;
extern crate serde_json;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;
use serde_json::{Value};
use std::{thread, time};
use reqwest::header::UserAgent;

error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
        JSONError(serde_json::Error);
    }
}

fn check_for_identifier(comment: &str){
    lazy_static! {
        //static ref RE: Regex = Regex::new(r".*?stats!\s?([^\s]+)").unwrap();
        static ref RE: Regex = Regex::new(r"[\w]+\s([^\s]+)").unwrap();
    }
    if let Some(re_match) = RE.captures(comment).and_then(|x| x.get(1)){ 
        println!("{}", re_match.as_str());
    }
}

fn spider_comments(children: &Value){
    for child in children.as_array().unwrap(){
        let data = &child["data"];
        let body = &data["body"];
        match body.as_str(){
            Some(str_) => check_for_identifier(str_),
            None => {}
        };
        let replies = &data["replies"];
        if replies != "" && !replies.is_null() {
            let reply_data = &data["replies"]["data"];
            spider_comments(&reply_data["children"]);
        }
    }
}

fn run() -> Result<()> {
    pretty_env_logger::init().unwrap();
    let subreddit_url: &'static str = "https://www.reddit.com/r/PUBATTLEGROUNDS/new/.json";
    let client = reqwest::Client::new()?;
    let mut res = client.get(subreddit_url).header(UserAgent("pubg stats bot /u/LePianoDentist".to_string())).send()?;
    let json = res.json::<Value>()?;
    let posts = &json["data"]["children"];
    for post in posts.as_array().unwrap(){  // TODO safer unwraps?
        let permalink = &post["data"]["permalink"];
        let comments_url = "https://www.reddit.com".to_string() + permalink.as_str().unwrap() + ".json";
        println!("{}", comments_url);
        let mut comments = client.get(&comments_url).header(UserAgent("pubg stats bot /u/LePianoDentist".to_string())).send()?;
        let comment_json = comments.json::<Value>()?;
        spider_comments(&comment_json[1]["data"]["children"]);
        let two_secs = time::Duration::from_secs(2);

        thread::sleep(two_secs);
    }
    Ok(())
}

quick_main!(run);

