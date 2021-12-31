use crate::error_handler::CustomError;
use crate::posts::{Post, Posts};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

#[get("/posts")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let posts = Posts::find_all()?;
    Ok(HttpResponse::Ok().json(posts))
}

#[get("/posts/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let post = Posts::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(post))
}

#[post("/posts")]
async fn create(post: web::Json<Post>) -> Result<HttpResponse, CustomError> {
    let post = Posts::create(post.into_inner())?;
    Ok(HttpResponse::Ok().json(post))
}

#[put("/posts/{id}")]
async fn update(id: web::Path<i32>, post: web::Json<Post>) -> Result<HttpResponse, CustomError> {
    let post = Posts::update(id.into_inner(), post.into_inner())?;
    Ok(HttpResponse::Ok().json(post))
}

#[delete("/posts/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_post = Posts::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_post })))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(find_all);
    comfig.service(find);
    comfig.service(create);
    comfig.service(update);
    comfig.service(delete);
}
