use clap::Parser;
use reqwest::StatusCode;

///Cli url checker
#[derive(Parser, Clone)]
struct Cli{
    ///Url(s)
    #[clap(short, num_args = 1..)]
    urls: Vec<String>,
    ///Send request as POST method (default is GET)
    #[clap(short, long)]
    post: Option<bool>
}

pub struct UrlChecker{
    urls: Vec<String>,
    post: bool,
}

impl UrlChecker {
    pub fn new() -> Self{
        let cli = Cli::parse();
        if let Some(p) = cli.post {
            let post = p;
            Self { urls: cli.urls, post}
        }else{
            Self { urls: cli.urls, post: false }
        }
    }

    //todo: make this function to send request as multithreading
    pub async fn run(&mut self){
        let mut t_vec: Vec<tokio::task::JoinHandle<()>> = Vec::new();
        //cannot use iterator due to error: borrowed data escapes outside of method
        loop{
            let post = self.post;
            if let Some(url) = self.urls.pop(){
                let handler = tokio::spawn(async move {
                    request(&url, post).await;
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
    async fn send_get_request(url: &String) -> anyhow::Result<StatusCode>{
        let response = reqwest::get(url).await?;

        Ok(response.status())
    }
    async fn send_post_request(url: &String) -> anyhow::Result<StatusCode>{
        let response = reqwest::Client::new()
        .post(url)
        .send()
        .await?;
        
        Ok(response.status())
    }
}

async fn request(url: &String, post: bool){
    if post == true {
        match UrlChecker::send_post_request(url).await{
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
