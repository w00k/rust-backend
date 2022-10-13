// Indicamos que vamos a utilizar macros
#[macro_use]
// Importamos Diesel
extern crate diesel;

// Importamos la conexión con PostgreSQL
use diesel::prelude::*;
use diesel::pg::PgConnection;

// Importamos librerias para leer variables de entorno
use dotenvy::dotenv;
use std::env;

//use crate::models::NewPost;

// Importamos esquemas de BBDD y modelos
pub mod schema;
pub mod models;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {

    use self::models::{Post, NewPost};
    // use self::schema::posts;
    use self::schema::posts::dsl::*;

    let conn  = &mut establish_connection();

    let new_post = NewPost { 
        title: "Mi primer blogpost", 
        body: "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum. ", 
        slug: "primer-post",
    };

    diesel::insert_into(posts).values(&new_post).get_result::<Post>(conn).expect("Error en insertar dato");

    //SELECT * FROM posts
    let posts_result = posts.load::<Post>(conn).expect("Error en consulta SQL");

    // Iteramos los resultados de la consulta
    for post in posts_result {
        println!("{}", post.id);
        println!("{}", post.title);
        println!("{}", post.body);
        println!("{}", post.slug);
        println!("----------------");
    }
}
