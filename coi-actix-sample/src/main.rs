use crate::{
    postgres::PostgresPoolProvider, repositories::repo::RepositoryProvider,
    services::service::ServiceProvider,
};
use actix_web::{middleware, App, HttpServer};
use coi::container;
use mobc_postgres::{mobc::Pool, tokio_postgres::NoTls, PgConnectionManager};
use std::sync::{Arc, RwLock};

mod dtos;
mod models;
mod postgres;
mod repositories;
mod routes;
mod services;

#[actix_rt::main]
async fn main() -> Result<(), String> {
    std::env::set_var("RUST_LOG", "actix_server=debug,actix_web=debug");
    env_logger::init();

    let config = "host=127.0.0.1 dbname=docker port=45432 user=docker password=docker"
        .parse()
        .map_err(|e| format!("{}", e))?;
    let manager = PgConnectionManager::new(config, NoTls);
    let pool = Pool::builder().max_open(20).build(manager);
    let pool_provider = PostgresPoolProvider::new(pool);

    let container = Arc::new(RwLock::new(container! {
        pool => pool_provider.singleton,
        service => ServiceProvider.scoped,
        repository => RepositoryProvider.scoped,
    }));

    HttpServer::new(move || {
        App::new()
            .app_data(Arc::clone(&container))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .configure(routes::data::route_config)
    })
    .bind("127.0.0.1:8000")
    .map_err(|e| format!("{}", e))?
    .run()
    .await
    .map_err(|e| format!("{}", e))
}
