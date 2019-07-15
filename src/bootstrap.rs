use crate::config::ApplicationConfig;
use crate::routes::routes;
use crate::logging::init_logging;
use crate::database::{init_database, run_migrations};

use actix_web::{HttpServer, App, middleware};

pub struct ApplicationBootstrap {}

impl ApplicationBootstrap {
    pub fn start(config: ApplicationConfig) -> std::io::Result<()> {
        init_logging().expect("Failed to initialize logging!");

        info!("Initializing database connection...");
        let database = init_database(config.database);
        info!("Running database migrations...");
        run_migrations(&database);

        info!("Starting up actix server...");
        HttpServer::new(move || {
            App::new()
                .wrap(middleware::Logger::default())
                .configure(|cfg| routes(cfg, database.clone()))
        })
            .bind(config.server.get_listen_address())?
            .run()
    }
}