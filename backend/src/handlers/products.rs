use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use mongodb::{Database, bson::{doc, oid::ObjectId}};
use chrono::Utc;
use futures::stream::TryStreamExt;
use crate::models::{Product, CreateProductRequest};
use crate::auth::verify_jwt;

#[get("/products")]
pub async fn get_products(
    db: web::Data<Database>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    let collection = db.collection::<Product>("products");
    
    let mut filter = doc! {};
    
    if let Some(category) = query.get("category") {
        filter.insert("category", category);
    }
    
    if let Some(search) = query.get("search") {
        filter.insert("$or", vec![
            doc! { "title": { "$regex": search, "$options": "i" } },
            doc! { "description": { "$regex": search, "$options": "i" } },
        ]);
    }

    match collection.find(filter, None).await {
        Ok(cursor) => {
            match cursor.try_collect::<Vec<Product>>().await {
                Ok(products) => HttpResponse::Ok().json(products),
                Err(_) => HttpResponse::InternalServerError().json("Failed to fetch products"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().json("Failed to fetch products"),
    }
}

#[get("/products/{id}")]
pub async fn get_product_by_id(
    db: web::Data<Database>,
    id: web::Path<String>,
) -> impl Responder {
    let collection = db.collection::<Product>("products");
    
    let object_id = match ObjectId::parse_str(id.as_str()) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().json("Invalid product ID"),
    };

    match collection.find_one(doc! { "_id": object_id }, None).await {
        Ok(Some(product)) => HttpResponse::Ok().json(product),
        Ok(None) => HttpResponse::NotFound().json("Product not found"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to fetch product"),
    }
}

#[post("/products")]
pub async fn create_product(
    db: web::Data<Database>,
    req: HttpRequest,
    product_req: web::Json<CreateProductRequest>,
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

    let seller_id = claims.sub;
    let collection = db.collection::<Product>("products");

    let new_product = Product {
        id: None,
        seller_id,
        title: product_req.title.clone(),
        description: product_req.description.clone(),
        category: product_req.category.clone(),
        price: product_req.price,
        file_type: product_req.file_type.clone(),
        file_url: product_req.file_url.clone(),
        icon: product_req.icon.clone(),
        rating: None,
        downloads: 0,
        created_at: Utc::now(),
    };

    match collection.insert_one(new_product, None).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "Product created successfully"
        })),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create product"),
    }
}
