// Importamos Diesel
extern crate diesel;

// Importamos la conexión con PostgreSQL
use diesel::pg::PgConnection;
use diesel::prelude::*;

// Importamos librerias para leer variables de entorno
use dotenvy::dotenv;
use std::env;
//use std::fmt::format;

// tera framework
use tera::Tera;

// pool de conexiones a la base de datos
//use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use diesel::r2d2::Pool;
use diesel::r2d2::{self, ConnectionManager};

// actix framework para lo que es servicios web
use actix_web::{
    get, http::header::ContentType, post, web, App, HttpResponse, HttpServer, Responder,
};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

// Importamos esquemas de BBDD y modelo
pub mod models;
pub mod schema;

use self::models::{NewPostHandler, Post};
use self::schema::posts::dsl::*;

////////////////// fin imports /////////////////

// establish_connection: lee la variable en .env file y crea la conexión a la base de datos
// @param: none
// return Pool<ConnectionManager<PgConnection>>
pub fn establish_connection() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder()
        .build(connection)
        .expect("No se puedo construir el pool de conexiones");
    return pool;
}

// hello_world: return a "Hello world" message
// return Responder hay una variante para retornar el valor..
// return HttpResponse::Ok().body("Hello world");
// ó
// HttpResponse::Ok().body("Hello world")
#[get("/")]
async fn index(pool: web::Data<DbPool>, template_manager: web::Data<tera::Tera>) -> impl Responder {
    let mut conn = pool.get().expect("Problemas al traer la base de datos");

    // en el treads qn que estamos lo bloquea para no generar condición de carrera
    return match web::block(move || posts.load::<Post>(&mut conn)).await {
        Ok(app_data) => {
            let mut ctx = tera::Context::new();
            let data = app_data.unwrap();
            ctx.insert("posts", &data);

            HttpResponse::Ok()
            .content_type(ContentType::html())
            .body(template_manager.render("index_tera.html", &ctx).unwrap())


            // HttpResponse::Ok()
            //    .content_type(ContentType::json())
            //    .body(format!("{:?}", app_data))
        }
        Err(_err) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body("Error al recibir la data"),
    };
}

#[post("/new_post")]
async fn new_post(pool: web::Data<DbPool>, item: web::Json<NewPostHandler>) -> impl Responder {
    let mut conn = pool.get().expect("Problemas al traer la base de datos");

    return match web::block(move || Post::create_post(&mut conn, &item)).await {
        Ok(app_data) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(format!("{:?}", app_data)),
        Err(_err) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body("Error al recibir la data"),
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // dotenv().ok();
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let connection = ConnectionManager::<PgConnection>::new(database_url);
    // let pool = Pool::builder().build(connection).expect("No se puedo construir el pool de conexiones");

    println!("server init"); 
    let pool = establish_connection();

    // move indicamos que el ownership a donde lo vamos a necesitar, en este caso es el pool de conexiones
    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*.html")).unwrap();

        App::new()
            .service(index)
            .service(new_post)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(tera.clone()))
    })
    .bind(("0.0.0.0", 9900))?
    .run()
    .await
}
