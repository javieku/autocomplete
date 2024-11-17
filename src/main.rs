use autocomplete::constant::values::CONFIG;
use autocomplete::server::server::AppServer;
#[tokio::main]
async fn main() {
    let config = CONFIG.clone();
    println!("Create a new server.");
    let server = AppServer::new(config).await.unwrap();
    println!("Run the server.");
    server.run().await.unwrap();
}
