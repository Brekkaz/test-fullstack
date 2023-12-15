use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Serialize;

mod api;
mod models;
mod repository;
mod utils;

#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}

async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
#[cfg(not(tarpaulin_include))]
async fn main() -> std::io::Result<()> {
    let todo_db = repository::database::Database::new();
    let app_data = web::Data::new(todo_db);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(api::config::config)
            .service(healthcheck)
            .default_service(web::route().to(not_found))
            .wrap(actix_web::middleware::Logger::default())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::{healthcheck, not_found};
    use actix_web::http::StatusCode;
    use actix_web::{test, web, App};

    #[actix_rt::test]
    async fn test_should_get_health_check_correctly() {
        let app = App::new().service(healthcheck);
        let mut app = test::init_service(app).await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_should_get_not_found_correctly() {
        let mut app =
            test::init_service(App::new().default_service(web::route().to(not_found))).await;
        let request = test::TestRequest::get().uri("/lorem").to_request();
        let response = test::call_service(&mut app, request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
