
mod checker;

use checker::UrlChecker;

#[tokio::main]
async fn main(){
    UrlChecker::new().run();
}