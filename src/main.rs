use autocomplete::constant::values::CONFIG;
use autocomplete::server::server::AppServer;
use log::info;

#[tokio::main]
async fn main() {
    env_logger::init();
    let config = CONFIG.clone();
    info!("Create a new server.");
    let server = AppServer::new(config).await.unwrap();
    info!("Run the server.");
    server.run().await.unwrap();
}
