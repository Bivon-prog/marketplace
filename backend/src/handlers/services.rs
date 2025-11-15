use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use mongodb::{Database, bson::{doc, oid::ObjectId}};
use chrono::Utc;
use futures::stream::TryStreamExt;
use crate::models::{Service, CreateServiceRequest};
use crate::auth::verify_jwt;

#[get("/services")]
pub async fn get_services(
    db: web::Data<Database>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    let collection = db.collection::<Service>("services");
    
    let mut filter = doc! {};
    
    if let Some(category) = query.get("category") {
        filter.insert("category", category);
    }
    
    if let Some(location) = query.get("location") {
        filter.insert("location", doc! { "$regex": location, "$options": "i" });
    }
    
    if let Some(search) = query.get("search") {
        filter.insert("$or", vec![
            doc! { "title": { "$regex": search, "$options": "i" } },
            doc! { "description": { "$regex": search, "$options": "i" } },
        ]);
    }

    match collection.find(filter, None).await {
        Ok(cursor) => {
            match cursor.try_collect::<Vec<Service>>().await {
                Ok(services) => HttpResponse::Ok().json(services),
                Err(_) => HttpResponse::InternalServerError().json("Failed to fetch services"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().json("Failed to fetch services"),
    }
}

#[get("/services/{id}")]
pub async fn get_service_by_id(
    db: web::Data<Database>,
    id: web::Path<String>,
) -> impl Responder {
    let collection = db.collection::<Service>("services");
    
    let object_id = match ObjectId::parse_str(id.as_str()) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().json("Invalid service ID"),
    };

    match collection.find_one(doc! { "_id": object_id }, None).await {
        Ok(Some(service)) => HttpResponse::Ok().json(service),
        Ok(None) => HttpResponse::NotFound().json("Service not found"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to fetch service"),
    }
}

#[post("/services")]
pub async fn create_service(
    db: web::Data<Database>,
    req: HttpRequest,
    service_req: web::Json<CreateServiceRequest>,
) -> impl Responder {
    let auth_header = match req.headers().get("Authorization") {
        Some(h) => h.to_str().unwrap_or(""),
        None => return HttpResponse::Unauthorized().json("Missing authorization"),
    };

    let token = auth_header.strip_prefix("Bearer ").unwrap_or("");
    let claims = match verify_jwt(token) {
        Ok(c) => c,
        Err(_) => return HttpResponse::Unauthorized().json("Invalid token"),
    };

    let provider_id = claims.sub;
    let collection = db.collection::<Service>("services");

    let new_service = Service {
        id: None,
        provider_id,
        title: service_req.title.clone(),
        description: service_req.description.clone(),
        category: service_req.category.clone(),
        price: service_req.price,
        location: service_req.location.clone(),
        icon: service_req.icon.clone(),
        rating: None,
        created_at: Utc::now(),
    };

    match collection.insert_one(new_service, None).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "Service created successfully"
        })),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create service"),
    }
}
