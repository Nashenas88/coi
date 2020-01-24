use crate::{dtos::data::DataDto, services::service::IService};
use actix_web::{
    web::{self, HttpResponse, ServiceConfig},
    Responder,
};
use coi_actix_web::{ContainerKey, Injected};
use std::sync::Arc;

struct ServiceKey;
impl ContainerKey<dyn IService> for ServiceKey {
    const KEY: &'static str = "service";
}

async fn get(
    id: web::Path<i64>,
    Injected(service, ..): Injected<Arc<dyn IService>, ServiceKey>,
) -> Result<impl Responder, ()> {
    let name = service.get(*id).await.map_err(|e| log::error!("{}", e))?;
    Ok(HttpResponse::Ok().json(DataDto::from(name)))
}

async fn get_all(
    Injected(service, ..): Injected<Arc<dyn IService>, ServiceKey>,
) -> Result<impl Responder, ()> {
    let data = service.get_all().await.map_err(|e| log::error!("{}", e))?;
    Ok(HttpResponse::Ok().json(data.into_iter().map(DataDto::from).collect::<Vec<_>>()))
}

pub fn route_config(config: &mut ServiceConfig) {
    config.service(
        web::scope("/data")
            .route("", web::get().to(get_all))
            .route("/", web::get().to(get_all))
            .route("/{id}", web::get().to(get)),
    );
}
