mod errors;
mod handlers;
mod models;
mod pg_database;

use models::todo_repo::TodoRepo;
use pg_database::PgDatabase;

use actix_http::KeepAlive;
use actix_web::{web, App, HttpServer};
use handlers::{create_todo, delete_todo, get_todo, hello_world, list_todo, update_todo};
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pg_database = PgDatabase::new(&PgDatabase::get_pg_url("POSTGRESQL_URL")).await;
    let todo_repo = TodoRepo::new(pg_database).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(todo_repo.clone()))
            .service(hello_world)
            .service(create_todo)
            .service(delete_todo)
            .service(update_todo)
            .service(get_todo)
            .service(list_todo)
    })
    .keep_alive(KeepAlive::Os)
    .client_request_timeout(Duration::from_secs(0))
    .backlog(102400)
    .workers(num_cpus::get())
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
