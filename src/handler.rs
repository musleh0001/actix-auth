use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use tracing::error;
use validator::Validate;

use crate::connections::postgres::Db;

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(length(
        min = 1,
        max = 100,
        message = "Name must be between 1 and 100 chracters"
    ))]
    pub name: String,

    pub email: String,

    #[validate(length(
        min = 8,
        max = 100,
        message = "Password must be between 8 and 100 chracters"
    ))]
    pub password: String,
}

pub async fn create_user(db: web::Data<Db>, user: web::Json<CreateUserRequest>) -> impl Responder {
    match db
        .insert_user(&user.name, &user.email, &user.password)
        .await
    {
        Ok(user) => HttpResponse::Created().json(user),
        Err(err) => {
            error!("Error creating user: {:?}", err);
            HttpResponse::InternalServerError().json("Failed to create user")
        }
    }
}

pub async fn get_all_users(db: web::Data<Db>) -> impl Responder {
    match db.get_all_users().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => {
            error!("Error fetching users: {:?}", err);
            HttpResponse::InternalServerError().json("Failed to fetch users")
        }
    }
}
