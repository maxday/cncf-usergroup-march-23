use actix_web::middleware::Logger;
use actix_web::{web, App, HttpRequest, HttpServer};
use actix_web_opentelemetry::RequestTracing;
use provider::{setup_provider, shutdown_provider};

mod provider;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    setup_provider();
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(RequestTracing::new())
            .service(web::resource("/pizza/{id}").to(get_pizza))
    })
    .bind("0.0.0.0:7081")
    .unwrap()
    .run()
    .await?;

    shutdown_provider();

    Ok(())
}

async fn get_pizza(pizza_id: actix_web::web::Path<String>, req: HttpRequest) -> String {
    for i in req.headers().iter() {
        println!("debug header = {:?}", i);
    }
    get_pizza_details(pizza_id.as_ref())
}

#[tracing::instrument]
fn get_pizza_details(pizza_id: &str) -> String {
    tracing::info!("debug get_pizza_details");
    format!("this is the pizza details for {}", pizza_id)
}
