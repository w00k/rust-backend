// Indicamos que vamos a utilizar macros
#[macro_use]
// Importamos Diesel
extern crate diesel;

// Importamos la conexión con PostgreSQL
use diesel::pg::PgConnection;
use diesel::prelude::*;

// Importamos librerias para leer variables de entorno
use dotenvy::dotenv;
use std::env;

use crate::models::PostSimple;

// pool de conexiones a la base de datos
use diesel::r2d2::{self, ConnectionManager};
use diesel::r2d2::Pool;

// actix framework para lo que es servicios web
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

// Importamos esquemas de BBDD y modelos
pub mod models;
pub mod schema;

////////////////// fin imports /////////////////

// establish_connection: lee la variable en .env file y crea la conexión a la base de datos
// @param: none
// return PgConnection
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// hello_world: return a "Hello world" message
// return Responder hay una variante para retornar el valor..
// return HttpResponse::Ok().body("Hello world");
// ó
// HttpResponse::Ok().body("Hello world")
#[get("/hello")]
async fn hello_world() -> impl Responder {
    return HttpResponse::Ok().body("Hello world");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(connection).expect("No se puedo construir el pool de conexiones");

    HttpServer::new(move || App::new().service(hello_world).app_data(pool.clone()))
        .bind(("localhost", 9900))?
        .run()
        .await
}
