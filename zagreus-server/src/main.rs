#![deny(clippy::all)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use std::sync::Arc;

use crate::config::loader::ConfigurationManager;
use crate::config::ZagreusServerConfig;
use crate::controller::ServerController;
use crate::template::registry::TemplateRegistry;
use crate::websocket::server::WebsocketServer;

mod data;
mod config;
mod controller;
mod endpoint;
mod error;
mod fs;
mod logger;
mod template;
mod websocket;

const ZAGREUS_VERSION: &str = "0.0.1";

const APPLICATION_NAME: &str = "zagreus-server";
const CONFIG_FILE_NAME: &str = "config.json";

type ServerTemplateRegistry = Arc<tokio::sync::RwLock<TemplateRegistry>>;

#[tokio::main]
async fn main() {
    let application_folder = fs::get_application_folder(APPLICATION_NAME).unwrap_or_else(|err| {
        panic!("Could not get application folder: {}", err);
    });
    logger::init_logger();

    match ConfigurationManager::<ZagreusServerConfig>::load(&application_folder, CONFIG_FILE_NAME) {
        Ok(manager) => start_with_config(manager).await,
        Err(err) => error!("Could not load configuration: {}.", err),
    }
}

async fn start_with_config(configuration_manager: ConfigurationManager<ZagreusServerConfig>) {
    let configuration = configuration_manager.get_configuration();
    let ws_server = Arc::new(WebsocketServer::new());
    let (template_event_tx, template_event_rx) = tokio::sync::mpsc::unbounded_channel();
    let mut template_registry = TemplateRegistry::new(&configuration.data_folder, template_event_tx);
    template_registry.load_templates();
    let template_registry = Arc::new(tokio::sync::RwLock::new(template_registry));

    let server_controller = Arc::new(ServerController::new(template_event_rx,
                                                           ws_server.clone(), template_registry.clone()));

    match endpoint::routes::get_routes(server_controller, ws_server, template_registry, configuration) {
        Ok(routes) => {
            warp::serve(routes)
                .run(([0, 0, 0, 0], 58179))
                .await
        }
        Err(err) => {
            error!("Could not configure server routes: {}.", err);
        }
    }
}
