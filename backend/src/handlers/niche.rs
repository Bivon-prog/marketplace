use actix_web::{get, web, HttpResponse, Responder};
use mongodb::{Database, bson::doc};
use futures::stream::TryStreamExt;
use crate::models::Product;

#[get("/niche/{niche_type}")]
pub async fn get_niche_products(
    db: web::Data<Database>,
    niche_type: web::Path<String>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> impl Responder {
    let niche_categories = match niche_type.as_str() {
        "resume" => vec!["career", "resume", "cv"],
        "business" => vec!["business", "invoice", "contract"],
        "student" => vec!["education", "student", "notes"],
        "creator" => vec!["creative", "design", "graphics"],
        "developer" => vec!["development", "code", "api"],
        _ => return HttpResponse::BadRequest().json("Invalid niche type"),
    };

    let collection = db.collection::<Product>("products");
    
    let mut filter = doc! {
        "category": { "$in": niche_categories }
    };
    
    if let Some(search) = query.get("search") {
        filter.insert("$or", vec![
            doc! { "title": { "$regex": search, "$options": "i" } },
            doc! { "description": { "$regex": search, "$options": "i" } },
        ]);
    }

    let mut options = mongodb::options::FindOptions::default();
    
    if let Some(sort) = query.get("sort") {
        let sort_doc = match sort.as_str() {
            "popular" => doc! { "downloads": -1 },
            "recent" => doc! { "created_at": -1 },
            "price-low" => doc! { "price": 1 },
            "price-high" => doc! { "price": -1 },
            _ => doc! { "created_at": -1 },
        };
        options.sort = Some(sort_doc);
    } else {
        options.sort = Some(doc! { "created_at": -1 });
    }

    match collection.find(filter, options).await {
        Ok(cursor) => {
            match cursor.try_collect::<Vec<Product>>().await {
                Ok(products) => HttpResponse::Ok().json(products),
                Err(_) => HttpResponse::InternalServerError().json("Failed to fetch niche products"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().json("Failed to fetch niche products"),
    }
}
