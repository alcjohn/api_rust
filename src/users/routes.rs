use crate::error_handler::CustomError;
use crate::users::{User, Users};
use actix_web::{post, web, HttpResponse};
use argon2::{self, Config};
use chrono::prelude::*;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::json;

const JWT_SECRET: &[u8] = b"secret";

fn hash(password: &[u8]) -> String {
    let salt = rand::thread_rng().gen::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

fn verify(hash: &str, password: &[u8]) -> bool {
    argon2::verify_encoded(hash, password).unwrap_or(false)
}

#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: i32,
    exp: usize,
}

fn create_jwt(id: &i32) -> String {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::seconds(60))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: id.to_owned(),
        exp: expiration as usize,
    };
    let header = Header::new(Algorithm::HS512);
    encode(&header, &claims, &EncodingKey::from_secret(JWT_SECRET)).unwrap()
}

#[post("/signin")]
async fn signin(user: web::Json<User>) -> Result<HttpResponse, CustomError> {
    let user = Users::create(User {
        email: user.email.clone(),
        password: hash(&user.password.as_bytes()),
    })?;

    Ok(HttpResponse::Ok().json(user))
}

#[post("/login")]
async fn login(user: web::Json<User>) -> Result<HttpResponse, CustomError> {
    let u = Users::find_by_email(user.email.to_string())?;
    if verify(&u.password.clone(), &user.password.as_bytes()) == false {
        return Ok(HttpResponse::Unauthorized().json(json!({
            "message": "Bad Password",
        })));
    }
    Ok(HttpResponse::Ok().json(json!({
        "token": create_jwt(&u.id)
    })))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(signin);
    comfig.service(login);
}
