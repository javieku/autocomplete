use super::state::AppState;
use crate::configuration::app_config::AppConfig;
use crate::handler::autocomplete::create_router_app;
use anyhow::Result;
use log::info;
use std::fmt::Error;

pub struct AppServer {
    pub state: AppState,
    tcp: tokio::net::TcpListener,
}
impl AppServer {
    pub async fn new(mut config: AppConfig) -> Result<Self, Error> {
        let tcp = tokio::net::TcpListener::bind(config.server.get_socket_addr().unwrap())
            .await
            .unwrap();
        let addr = tcp.local_addr().unwrap();
        info!("The server is listening on: {addr}");
        config.server.port = addr.port();
        let state = AppState::new(config).await?;
        Ok(Self { state, tcp })
    }

    pub async fn run(self) -> Result<(), Error> {
        let router = create_router_app(self.state);
        axum::serve(self.tcp, router).await.unwrap();
        Ok(())
    }
}
