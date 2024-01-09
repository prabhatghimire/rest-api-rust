// src/api/handlers.rs
use crate::database::mongodb_connector::get_database;
use crate::utils::auth::generate_token;
use crate::utils::auth::hash_password;
use crate::Response;
use actix_web::{post, web, HttpResponse, Responder};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    // #[serde(rename = "_id")]
    username: String,
    pub _id: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone_no: Option<String>,
    pub password: String,
    pub image: Option<String>,
    pub user_role: UserRole,
    pub agree_terms_condition: Option<bool>,
    pub added_on: Option<String>,
    pub deleted: Option<String>,
    pub is_verified: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserRole {
    Superuser,
    Owner,
    Manager,
    KitchenStaff,
    WaitStaff,
    Admin,
    Customer,
}

#[post("/login")]
pub async fn login(data: web::Json<LoginRequest>) -> impl Responder {
    let db = get_database();

    let query = doc! {
        "email":  &data.username,
    };

    let projection = doc! {
        "_id": true,
        "role":1,
        "business_name": 1,
        "business_email": 1,
        "business_phone": 1,
        "location": 1,
        // "image": "$image.filename"
        "email":1, "password":1
    };

    let options = mongodb::options::FindOneOptions::builder()
        .projection(Some(projection))
        .build();

    // Use match to handle the Result<Option<User>, Error>
    match db
        .collection::<User>("users")
        .find_one(query, Some(options))
        .await
    {
        Ok(Some(user)) if user.password == data.password => {
            // Generate a JWT token
            let token: String = generate_token(user.username, 3600); // Set expiration time as needed

            HttpResponse::Ok().json(token)
        }
        Ok(_) => {
            println!("The value of y is");
            let response = Response {
                message: "user not found".to_string(),
                status: 404,
            };
            HttpResponse::NotFound().json(response) // Changed from Ok() to NotFound()
        }
        Err(_) => {
            let response = Response {
                message: "Database error".to_string(), // Changed message
                status: 500, // Set an appropriate status code for internal server error
            };
            HttpResponse::InternalServerError().json(response) // Changed from Ok() to InternalServerError()
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct SignupRequest {
    // Define your User struct fields here
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    phone_no: Option<String>,
    // Add other fields as needed
}

#[post("/register")]
async fn register(data: web::Json<SignupRequest>) -> impl Responder {
    // Hash the password before storing it in the database
    let signup_request = data.into_inner();

    let hashed_password = hash_password(signup_request.password);
    println!("{hashed_password}");
    let query = doc! {
        "email":  signup_request.email,
    };

    let projection = doc! {
        "_id": true,
        "role":1,
        "business_name": 1,
        "business_email": 1,
        "business_phone": 1,
        "location": 1,
        // "image": "$image.filename"
        "email":1, "password":1
    };

    let options = mongodb::options::FindOneOptions::builder()
        .projection(Some(projection))
        .build();

    // Connect to the MongoDB database
    let databse = get_database();

    // Check if the user already exists
    if let Ok(Some(_)) = databse
        .collection::<User>("users")
        .find_one(query, Some(options))
        .await
    {
        return HttpResponse::Conflict().json(json!({
            "status_code": 409,
            "status": "duplicate user",
            "message": "error"
        }));
    }

    let doc = User {
        _id: None,
        added_on: None,
        agree_terms_condition: None,
        username: String::new(),
        first_name: signup_request.first_name,
        last_name: signup_request.last_name,
        email: String::new(),
        phone_no: signup_request.phone_no,
        password: String::new(),
        image: None,
        user_role: UserRole::Owner,
        deleted: None,
        is_verified: None,
    };

    let result = databse
        .collection::<User>("users")
        .insert_one(doc, None)
        .await;

    if result.is_ok() {
        return HttpResponse::Ok().json(json!({
            "status_code": 200,
            "status": "done",
            "message": "Registration successful"
        }));
    } else {
        return HttpResponse::InternalServerError().json(json!({
            "status_code": 500,
            "status": "unknown error",
            "message": format!("Database error: {:?}", result.err())
        }));
    }
}
