use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Error};

pub mod models;
pub mod schema;
pub mod action;


pub mod db;


use self::action::{get_post, create_post};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    println!("heelllo ");
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// fn get_post(db: &mut PgConnection) {
//     posts
//         .filter(published.eq(true))
//         .limit(5)
//         .load::<Post>(db)
//         .expect("Error loading posts");

// }

#[get("/count-post")]
async fn count_post(pool: web::Data<db::DbPool>) -> Result<HttpResponse, Error> {
    // use web::block to offload blocking Diesel code without blocking server thread
    let results = web::block(move || {
        let mut conn = pool.get()?;
        get_post(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    println!("Displaying {} posts", results.len());
    let temp =  results.len();
    Ok(HttpResponse::Ok().body(temp))
}

#[post("/add-post")]
async fn add_post(pool: web::Data<db::DbPool>, form: web::Json<models::NewPostPayload>) -> Result<HttpResponse, Error> {
    // use web::block to offload blocking Diesel code without blocking server thread
    let results = web::block(move || {
        let mut conn = pool.get()?;
        create_post(&mut conn, &form.title, &form.body)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(results))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let pool = db::get_connection_pool();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
            .service(echo)
            .service(show_post)
            .service(add_post)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
