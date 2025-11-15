use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use mongodb::{Database, bson::doc};
use chrono::Utc;
use futures::stream::TryStreamExt;
use crate::models::{Booking, CreateBookingRequest};
use crate::auth::verify_jwt;

#[post("/bookings")]
pub async fn create_booking(
    db: web::Data<Database>,
    req: HttpRequest,
    booking_req: web::Json<CreateBookingRequest>,
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
    let collection = db.collection::<Booking>("bookings");

    let new_booking = Booking {
        id: None,
        customer_id,
        service_id: booking_req.service_id.clone(),
        booking_date: booking_req.booking_date.clone(),
        booking_time: booking_req.booking_time.clone(),
        notes: booking_req.notes.clone(),
        status: "pending".to_string(),
        created_at: Utc::now(),
    };

    match collection.insert_one(new_booking, None).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "Booking created successfully"
        })),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create booking"),
    }
}

#[get("/bookings")]
pub async fn get_user_bookings(
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
    let collection = db.collection::<Booking>("bookings");

    let filter = doc! { "customer_id": customer_id };
    let mut options = mongodb::options::FindOptions::default();
    options.sort = Some(doc! { "created_at": -1 });

    match collection.find(filter, options).await {
        Ok(cursor) => {
            match cursor.try_collect::<Vec<Booking>>().await {
                Ok(bookings) => HttpResponse::Ok().json(bookings),
                Err(_) => HttpResponse::InternalServerError().json("Failed to fetch bookings"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().json("Failed to fetch bookings"),
    }
}
