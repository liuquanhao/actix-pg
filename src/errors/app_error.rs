use super::todo_error::*;
use crate::models::message::Message;
use std::fmt;

use actix_web::{
    body::BoxBody,
    error::ResponseError,
    http::{header::ContentType, StatusCode},
    HttpResponse, HttpResponseBuilder,
};

#[derive(Debug)]
pub enum AppError {
    TodoRepo(TodoRepoError),
}

impl From<TodoRepoError> for AppError {
    fn from(inner: TodoRepoError) -> Self {
        AppError::TodoRepo(inner)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::TodoRepo(TodoRepoError::NotFound) => StatusCode::NOT_FOUND,
            AppError::TodoRepo(TodoRepoError::DatabaseError) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let error_msg = match self {
            AppError::TodoRepo(TodoRepoError::NotFound) => "没有找到Todo",
            AppError::TodoRepo(TodoRepoError::DatabaseError) => "数据库错误",
        };
        let message = Message { message: error_msg };
        let body = serde_json::to_string(&message).unwrap();
        HttpResponseBuilder::new(self.status_code())
            .content_type(ContentType::json())
            .body(body)
    }
}
