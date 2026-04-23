
mod checker;

use checker::UrlChecker;

//worker threads are two for now. I might fix this later
#[tokio::main(worker_threads = 2)]
async fn main(){
    UrlChecker::new().run().await;
}