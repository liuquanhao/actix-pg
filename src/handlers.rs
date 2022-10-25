use crate::errors::app_error::AppError;
use crate::models::message::Message;
use crate::models::pagination::Pagination;
use crate::models::todo::{CreateTodo, UpdateTodo};
use crate::models::todo_repo::TodoRepo;

use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Result};
use uuid::Uuid;

#[get("/helloworld/")]
pub async fn hello_world() -> impl Responder {
    Message {
        message: "helloworld",
    }
}

#[post("/todos/")]
pub async fn create_todo(
    web::Json(create_todo): web::Json<CreateTodo>,
    todo_repo: web::Data<TodoRepo>,
) -> Result<impl Responder, AppError> {
    let todo = todo_repo.create_todo(create_todo).await?;
    Ok(HttpResponse::Ok().json(todo))
}

#[delete("/todos/{id}")]
pub async fn delete_todo(
    id: web::Path<(Uuid,)>,
    todo_repo: web::Data<TodoRepo>,
) -> Result<impl Responder, AppError> {
    let _ = todo_repo.delete_todo(id.0).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[put("/todos/{id}")]
pub async fn update_todo(
    id: web::Path<(Uuid,)>,
    web::Json(update_todo): web::Json<UpdateTodo>,
    todo_repo: web::Data<TodoRepo>,
) -> Result<impl Responder, AppError> {
    let _ = todo_repo.update_todo(id.0, update_todo).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[get("/todos/{id}")]
pub async fn get_todo(
    id: web::Path<(Uuid,)>,
    todo_repo: web::Data<TodoRepo>,
) -> Result<impl Responder, AppError> {
    let todo = todo_repo.get_todo(id.0).await?;
    Ok(HttpResponse::Ok().json(todo))
}

#[get("/todos/")]
pub async fn list_todo(
    web::Query(pagination): web::Query<Pagination>,
    todo_repo: web::Data<TodoRepo>,
) -> Result<impl Responder, AppError> {
    let todos = todo_repo.list_todo(pagination).await?;
    Ok(HttpResponse::Ok().json(todos))
}
