use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use mongodb::{Database, bson::doc};
use chrono::Utc;
use futures::stream::TryStreamExt;
use crate::models::{Review, CreateReviewRequest, Service, Product};
use crate::auth::verify_jwt;

#[post("/reviews")]
pub async fn create_review(
    db: web::Data<Database>,
    req: HttpRequest,
    review_req: web::Json<CreateReviewRequest>,
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

    let user_id = claims.sub;
    let collection = db.collection::<Review>("reviews");

    let new_review = Review {
        id: None,
        user_id,
        item_id: review_req.item_id.clone(),
        item_type: review_req.item_type.clone(),
        rating: review_req.rating,
        comment: review_req.comment.clone(),
        created_at: Utc::now(),
    };

    match collection.insert_one(new_review, None).await {
        Ok(_) => {
            // Update average rating
            let filter = doc! { "item_id": &review_req.item_id, "item_type": &review_req.item_type };
            
            if let Ok(cursor) = collection.find(filter, None).await {
                if let Ok(reviews) = cursor.try_collect::<Vec<Review>>().await {
                    let avg_rating: f64 = reviews.iter().map(|r| r.rating as f64).sum::<f64>() / reviews.len() as f64;
                    
                    if review_req.item_type == "service" {
                        if let Ok(oid) = mongodb::bson::oid::ObjectId::parse_str(&review_req.item_id) {
                            let services = db.collection::<Service>("services");
                            let _ = services.update_one(
                                doc! { "_id": oid },
                                doc! { "$set": { "rating": avg_rating } },
                                None,
                            ).await;
                        }
                    } else if review_req.item_type == "product" {
                        if let Ok(oid) = mongodb::bson::oid::ObjectId::parse_str(&review_req.item_id) {
                            let products = db.collection::<Product>("products");
                            let _ = products.update_one(
                                doc! { "_id": oid },
                                doc! { "$set": { "rating": avg_rating } },
                                None,
                            ).await;
                        }
                    }
                }
            }

            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "message": "Review created successfully"
            }))
        }
        Err(_) => HttpResponse::InternalServerError().json("Failed to create review"),
    }
}

#[get("/reviews/{item_type}/{item_id}")]
pub async fn get_reviews(
    db: web::Data<Database>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (item_type, item_id) = path.into_inner();
    let collection = db.collection::<Review>("reviews");

    let filter = doc! { "item_type": item_type, "item_id": item_id };
    let mut options = mongodb::options::FindOptions::default();
    options.sort = Some(doc! { "created_at": -1 });

    match collection.find(filter, options).await {
        Ok(cursor) => {
            match cursor.try_collect::<Vec<Review>>().await {
                Ok(reviews) => HttpResponse::Ok().json(reviews),
                Err(_) => HttpResponse::InternalServerError().json("Failed to fetch reviews"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().json("Failed to fetch reviews"),
    }
}
