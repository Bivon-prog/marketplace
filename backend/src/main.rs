mod models;
mod handlers;
mod db;
mod auth;

use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_cors::Cors;
use mongodb::Client;
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let mongodb_uri = env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    let database_name = env::var("DATABASE_NAME").unwrap_or_else(|_| "marketplace_db".to_string());

    let client = Client::with_uri_str(&mongodb_uri)
        .await
        .expect("Failed to connect to MongoDB");

    let database = client.database(&database_name);

    // Initialize collections with indexes
    db::init_db(&database).await.expect("Failed to initialize database");

    let host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("SERVER_PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("{}:{}", host, port);

    println!("🚀 Server starting on http://{}", bind_address);
    println!("📦 Connected to MongoDB: {}", database_name);

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .app_data(web::Data::new(database.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .service(handlers::auth::login)
                    .service(handlers::auth::signup)
                    .service(handlers::services::get_services)
                    .service(handlers::services::create_service)
                    .service(handlers::services::get_service_by_id)
                    .service(handlers::products::get_products)
                    .service(handlers::products::create_product)
                    .service(handlers::products::get_product_by_id)
                    .service(handlers::niche::get_niche_products)
                    .service(handlers::bookings::create_booking)
                    .service(handlers::bookings::get_user_bookings)
                    .service(handlers::purchases::create_purchase)
                    .service(handlers::purchases::get_user_purchases)
                    .service(handlers::reviews::create_review)
                    .service(handlers::reviews::get_reviews)
            )
    })
    .bind(&bind_address)?
    .run()
    .await
}
