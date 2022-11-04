// Importamos Diesel
extern crate diesel;

// Importamos la conexión con PostgreSQL
use diesel::pg::PgConnection;
use diesel::prelude::*;

// Importamos librerias para leer variables de entorno
use dotenvy::dotenv;
use std::env;
//use std::fmt::format;

//use crate::models::PostSimple;

// pool de conexiones a la base de datos
//use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use diesel::r2d2::{self, ConnectionManager};
use diesel::r2d2::Pool;

// actix framework para lo que es servicios web
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, http::header::ContentType};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

// Importamos esquemas de BBDD y modelo 
pub mod models;
pub mod schema;

use self::models::{Post, NewPostHandler};
use self::schema::posts::dsl::*;

////////////////// fin imports /////////////////

// establish_connection: lee la variable en .env file y crea la conexión a la base de datos
// @param: none
// return Pool<ConnectionManager<PgConnection>>
pub fn establish_connection() -> Pool<ConnectionManager<PgConnection>>{
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(connection).expect("No se puedo construir el pool de conexiones");
    return pool;
}

// hello_world: return a "Hello world" message
// return Responder hay una variante para retornar el valor..
// return HttpResponse::Ok().body("Hello world");
// ó
// HttpResponse::Ok().body("Hello world")
#[get("/")]
async fn index(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = pool.get().expect("Problemas al traer la base de datos");

    // en el treads qn que estamos lo bloquea para no generar condición de carrera
    return match web::block(move || {posts.load::<Post>(&mut conn)}).await {
        Ok(app_data) => { 
           HttpResponse::Ok().content_type(ContentType::json()).body(format!("{:?}", app_data))
        },
        Err(_err) => HttpResponse::Ok().content_type(ContentType::json()).body("Error al recibir la data")
    };
}


#[post("/new_post")]
async fn new_post(pool: web::Data<DbPool>, item: web::Json<NewPostHandler>) -> impl Responder {
    let mut conn = pool.get().expect("Problemas al traer la base de datos");
    
    return match web::block(move || {Post::create_post(&mut conn, &item)}).await {
        Ok(app_data) => { 
            HttpResponse::Ok().content_type(ContentType::json()).body(format!("{:?}", app_data))
        },
        Err(_err) => HttpResponse::Ok().content_type(ContentType::json()).body("Error al recibir la data")
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // dotenv().ok();
    // let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let connection = ConnectionManager::<PgConnection>::new(database_url);
    // let pool = Pool::builder().build(connection).expect("No se puedo construir el pool de conexiones");

    let pool = establish_connection();

    // move indicamos que el ownership a donde lo vamos a necesitar, en este caso es el pool de conexiones
    HttpServer::new(move || {
        App::new()
        .service(index)
        .service(new_post)
        .app_data(web::Data::new(pool.clone()))
    }).bind(("localhost", 9900))?.run().await
}
