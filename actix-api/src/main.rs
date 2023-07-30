use actix_web::{delete, get, patch, post, web, App, Error, HttpResponse, HttpServer, Responder};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use std::env;

pub mod action;
pub mod models;
pub mod schema;

pub mod db;

use self::action::{create_post, delete_post, get_all_published_post, get_post, publish_post};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn run_migration(connection: &mut impl MigrationHarness<diesel::pg::Pg>) {
    match connection.run_pending_migrations(MIGRATIONS) {
        Ok(_) => {
            println!("Migrations successfully completed");
        },
        Err(e) => {
            panic!("error running pending migrations {}", e)
        },
    };
}


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/post/{post_id}")]
async fn get_api_post(
    pool: web::Data<db::DbPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let post_id: i32 = path.into_inner();
    let result = web::block(move || {
        let mut conn = pool.get()?;
        get_post(&mut conn, post_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}

#[get("/count-post")]
async fn count_api_post(pool: web::Data<db::DbPool>) -> Result<HttpResponse, Error> {
    let results = web::block(move || {
        let mut conn = pool.get()?;
        get_all_published_post(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let temp = format!("Have {} posts", results.len());
    Ok(HttpResponse::Ok().body(temp))
}

#[post("/post")]
async fn add_api_post(
    pool: web::Data<db::DbPool>,
    form: web::Json<models::NewPostPayload>,
) -> Result<HttpResponse, Error> {
    let results = web::block(move || {
        let mut conn = pool.get()?;
        match form.published {
            None => create_post(&mut conn, &form.title, &form.body, false),
            Some(i) => create_post(&mut conn, &form.title, &form.body, i),
        }
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(results))
}

#[delete("/post/{post_id}")]
async fn delete_api_post(
    pool: web::Data<db::DbPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let post_id: i32 = path.into_inner();
    // use web::block to offload blocking Diesel code without blocking server thread
    web::block(move || {
        let mut conn = pool.get()?;
        delete_post(&mut conn, post_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    let temp = format!("Delete succeed post with id {}", post_id);
    Ok(HttpResponse::Ok().body(temp))
}

#[patch("/post/{post_id}")]
async fn publish_api_post(
    pool: web::Data<db::DbPool>,
    path: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let post_id: i32 = path.into_inner();
    // use web::block to offload blocking Diesel code without blocking server thread
    let result = web::block(move || {
        let mut conn = pool.get()?;
        publish_post(&mut conn, post_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(result))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let actix_port: String = env::var("ACTIX_PORT").unwrap();
    let max_worker: String = env::var("MAX_WORKERS").unwrap();
    HttpServer::new(|| {
        let pool = db::get_connection_pool();
        let mut connection = pool.clone().get().unwrap();
        run_migration(&mut connection);

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
            .service(get_api_post)
            .service(add_api_post)
            .service(delete_api_post)
            .service(count_api_post)
            .service(publish_api_post)
            .route("/hey", web::get().to(manual_hello))
    })
    .workers(max_worker.parse::<usize>().unwrap())
    .bind(("0.0.0.0", actix_port.parse::<u16>().unwrap()))?
    .run()
    .await
}
