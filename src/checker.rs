use std::{io::{BufRead, BufReader}};
use anyhow::{Result};
use clap::Parser;
use reqwest::StatusCode;

///Cli url checker
#[derive(Parser)]
#[command(about, long_about = None)]
pub struct Cli{
    ///Url(s)
    #[clap(short, num_args = 1..)]
    urls: Vec<String>,
    ///Send request as POST method (default is GET)
    #[arg(short, long)]
    post: Option<bool>,

    ///Body option for POST request if POST method is not used
    ///it will be replaced with default value. (Default is None)
    #[arg(short, long)]
    body: Option<String>,

    ///File option to iterate through file and making request for
    ///each url(s)
    ///All urls should be aligned line by line
    #[arg(short, long)]
    file: Option<String>
}

pub struct UrlChecker{
    urls: Vec<String>,
    post: bool,
    body: Option<String>
}

impl UrlChecker {
    pub fn new(cli: Option<Cli>) -> Result<Self>{
        // let cli = Cli::parse();
        let cl = cli.unwrap_or(Cli::parse());
        if let Some(path) = cl.file{
            let file = std::fs::File::open(path)?;
            let reader = BufReader::new(file);
            let mut urls = Vec::new();

            for line in reader.lines() {
                urls.push(line?);
            }
            Ok(Self {urls: urls, post: cl.post.unwrap_or(false), body: cl.body})
        }else{
            Ok(Self { urls: cl.urls, post: cl.post.unwrap_or(false), body: cl.body })
        }
    }

    pub async fn run(&mut self){
        let mut t_vec: Vec<tokio::task::JoinHandle<()>> = Vec::new();
        //cannot use iterator due to error: borrowed data escapes outside of method
        //note to my self: using arc, mutex made code lot messier. Simply using loop is btter
        //don't change it unless you want to see a chaos.
        loop{
            let post = self.post;
            let body = self.body.clone();
            if let Some(url) = self.urls.pop(){
                let handler = tokio::spawn(async move {
                    request(&url, post, body).await;
                });
                t_vec.push(handler);
            }else{
                break;
            }
        }
        for t in t_vec {
            match tokio::try_join!(t) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("Error while joining thread: {e}")
                }
            }
        }
    }
    async fn send_get_request(url: &String) -> anyhow::Result<StatusCode>{
        let response = reqwest::get(url).await?;

        Ok(response.status())
    }

    async fn send_post_request(url: &String, body: Option<String>) -> anyhow::Result<StatusCode>{
        if let Some(body) = body{
            let response = reqwest::Client::new()
            .post(url)
            .body(body)
            .send()
            .await?;
            Ok(response.status())
        }else{
            let response = reqwest::Client::new()
            .post(url)
            .send()
            .await?;

            Ok(response.status())
        }
    }

    #[allow(dead_code)]
    fn get_urls(&self) -> &Vec<String>{
        &self.urls
    }
}

async fn request(url: &String, post: bool, body: Option<String>){
    if post == true {
        match UrlChecker::send_post_request(url, body).await{
            Ok(code) => println!("url: {url} {code}"),
            Err(e) => eprintln!("url: {url}, request failed with {e}")
        }
    }else{
        match UrlChecker::send_get_request(url).await{
                Ok(code) => println!("url: {url} {code}"),
                Err(e) => eprintln!("url: {url}, request failed with: {e}")
        }
    }
}

#[cfg(test)]
mod test{
    use clap::Parser;

use super::{StatusCode, UrlChecker, Cli};

    #[tokio::test]
    async fn make_test_request(){
        let code = UrlChecker::send_get_request(&String::from("https://example.com")).await.unwrap();
        assert_eq!(code, StatusCode::OK);
    }

    #[tokio::test]
    async fn make_invalid_post_request(){
        //post request test purpose
        let code = UrlChecker::send_post_request(&String::from("https://example.com"), None)
            .await
            .unwrap();

        //example.com should return code 405
        assert_eq!(code, StatusCode::METHOD_NOT_ALLOWED);
    }

    #[tokio::test]
    //TODO: fix this test, why is stat_vec empty?
    //INFO: by the some reason urls vector is empty
    //+ it's not because of #[allow(dead_code)]
    async fn file_feature(){
        let mut stat_vec: Vec<StatusCode> = Vec::new();
        let checker = UrlChecker::new(
            Some(
                Cli::parse_from([String::from("-f ./src/urls.txt")])
            )
        ).unwrap();
        let urls = checker.get_urls();
        for url in urls{
            stat_vec.push(UrlChecker::send_get_request(&url).await.unwrap());
        }
        assert_eq!(stat_vec, [StatusCode::OK, StatusCode::OK])
    }
}