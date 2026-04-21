use clap::Parser;

///Cli url checker
#[derive(Parser)]
struct Cli{
    ///Url(s)
    #[clap(short, num_args = 1..)]
    urls: Vec<String>
}

pub struct UrlChecker{
    urls: Vec<String>
}

impl UrlChecker {
    pub fn new() -> Self{
        let cli = Cli::parse();
        Self { urls: cli.urls }
    }

    pub fn run(&self){
        for url in &self.urls {
            println!("{url}")
        }
    }
}