use actix_web::{post, web, HttpResponse, Responder};
use mongodb::Database;
use mongodb::bson::doc;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use crate::models::{LoginRequest, SignupRequest, AuthResponse, UserResponse, User};
use crate::auth::create_jwt;

#[post("/auth/signup")]
pub async fn signup(
    db: web::Data<Database>,
    req: web::Json<SignupRequest>,
) -> impl Responder {
    let password_hash = match hash(&req.password, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => return HttpResponse::InternalServerError().json("Failed to hash password"),
    };

    let collection = db.collection::<User>("users");

    // Check if email already exists
    if let Ok(Some(_)) = collection.find_one(doc! { "email": &req.email }, None).await {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "success": false,
            "message": "Email already exists"
        }));
    }

    let new_user = User {
        id: None,
        name: req.name.clone(),
        email: req.email.clone(),
        password_hash,
        user_type: req.user_type.clone(),
        created_at: Utc::now(),
    };

    match collection.insert_one(new_user, None).await {
        Ok(_) => HttpResponse::Ok().json(serde_json::json!({
            "success": true,
            "message": "User created successfully"
        })),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "success": false,
            "message": "Failed to create user"
        })),
    }
}

#[post("/auth/login")]
pub async fn login(
    db: web::Data<Database>,
    req: web::Json<LoginRequest>,
) -> impl Responder {
    let collection = db.collection::<User>("users");

    let user = collection
        .find_one(doc! { "email": &req.email }, None)
        .await;

    match user {
        Ok(Some(user)) => {
            if verify(&req.password, &user.password_hash).unwrap_or(false) {
                let user_id = user.id.unwrap().to_hex();
                let token = create_jwt(&user_id).unwrap();
                HttpResponse::Ok().json(AuthResponse {
                    token,
                    user: UserResponse {
                        id: user_id,
                        name: user.name,
                        email: user.email,
                        user_type: user.user_type,
                    },
                })
            } else {
                HttpResponse::Unauthorized().json(serde_json::json!({
                    "success": false,
                    "message": "Invalid credentials"
                }))
            }
        }
        _ => HttpResponse::Unauthorized().json(serde_json::json!({
            "success": false,
            "message": "Invalid credentials"
        })),
    }
}
