use clap::Parser;
use reqwest::StatusCode;
use anyhow::Result;

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
    pub async fn run(&mut self){
        let mut t_vec: Vec<tokio::task::JoinHandle<()>> = Vec::new();
        loop{
            if let Some(url) = self.urls.pop(){
                let handler = tokio::spawn(async move {
                    request(&url).await;
                });
                t_vec.push(handler);
            }else{
                break;
            }
        }
        for t in t_vec {
            let _ = tokio::join!(t);
        }
    }
    async fn send_get_request(url: &String) -> Result<StatusCode>{
        let response = reqwest::get(url).await?;

        Ok(response.status())
    }
}

async fn request(url: &String){
    match UrlChecker::send_get_request(url).await{
            Ok(code) => println!("url: {url} {code}"),
            Err(e) => eprintln!("{e}")
    }
}