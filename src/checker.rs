use clap::Parser;
use reqwest::{Error, StatusCode};

///Cli url checker
#[derive(Parser, Clone)]
struct Cli{
    ///Url(s)
    #[clap(short, num_args = 1..)]
    urls: Vec<String>,
}

pub struct UrlChecker{
    urls: Vec<String>,
}

impl UrlChecker {
    pub fn new() -> Self{
        let cli = Cli::parse();
        Self { urls: cli.urls}
    }

    //todo: make this function to send request as multithreading
    pub async fn run(&self){
        for url in &self.urls {
            match self.send_get_request(url).await{
                Ok(stat) => println!("url: {url} code: {stat}"),
                Err(e) => eprintln!("{e}"),
            }
        }
    }
    
    async fn send_get_request(&self, url: &String) -> Result<StatusCode, Error>{
        let response = reqwest::get(url).await?;

        Ok(response.status())
    }
}