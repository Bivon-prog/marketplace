use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use mongodb::{Database, bson::{doc, oid::ObjectId}};
use chrono::Utc;
use futures::stream::TryStreamExt;
use crate::models::{Purchase, CreatePurchaseRequest, Product};
use crate::auth::verify_jwt;

#[post("/purchases")]
pub async fn create_purchase(
    db: web::Data<Database>,
    req: HttpRequest,
    purchase_req: web::Json<CreatePurchaseRequest>,
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

    let customer_id = claims.sub;
    let products_collection = db.collection::<Product>("products");

    let product_oid = match ObjectId::parse_str(&purchase_req.product_id) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().json("Invalid product ID"),
    };

    let product = match products_collection.find_one(doc! { "_id": product_oid }, None).await {
        Ok(Some(p)) => p,
        _ => return HttpResponse::NotFound().json("Product not found"),
    };

    let purchases_collection = db.collection::<Purchase>("purchases");

    let new_purchase = Purchase {
        id: None,
        customer_id,
        product_id: purchase_req.product_id.clone(),
        payment_method: purchase_req.payment_method.clone(),
        amount: product.price,
        status: "completed".to_string(),
        download_url: product.file_url.clone(),
        created_at: Utc::now(),
    };

    match purchases_collection.insert_one(new_purchase, None).await {
        Ok(_) => {
            // Update download count
            let _ = products_collection
                .update_one(
                    doc! { "_id": product_oid },
                    doc! { "$inc": { "downloads": 1 } },
                    None,
                )
                .await;

            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "message": "Purchase successful",
                "download_url": product.file_url
            }))
        }
        Err(_) => HttpResponse::InternalServerError().json("Failed to create purchase"),
    }
}

#[get("/purchases")]
pub async fn get_user_purchases(
    db: web::Data<Database>,
    req: HttpRequest,
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

    let customer_id = claims.sub;
    let collection = db.collection::<Purchase>("purchases");

    let filter = doc! { "customer_id": customer_id };
    let mut options = mongodb::options::FindOptions::default();
    options.sort = Some(doc! { "created_at": -1 });

    match collection.find(filter, options).await {
        Ok(cursor) => {
            match cursor.try_collect::<Vec<Purchase>>().await {
                Ok(purchases) => HttpResponse::Ok().json(purchases),
                Err(_) => HttpResponse::InternalServerError().json("Failed to fetch purchases"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().json("Failed to fetch purchases"),
    }
}
