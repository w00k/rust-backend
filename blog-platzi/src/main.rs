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

use crate::models::PostSimple;

//use crate::models::NewPost;

// Importamos esquemas de BBDD y modelos
pub mod schema;
pub mod models;

////////////////// fin imports /////////////////

// establish_connection: lee la variable en .env file y crea la conexión a la base de datos
// @param: none
// return PgConnection
pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// add_new_post: agrega datos a la tabla posts
// @param: conexión a la base de datos
// void 
fn add_new_post(conn: &mut PgConnection) {
    use self::models::{Post, NewPost};
    use self::schema::posts::dsl::*;

    let new_post = NewPost { 
        title: "Mi septimo blogpost", 
        body: "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book. It has survived not only five centuries, but also the leap into electronic typesetting, remaining essentially unchanged. It was popularised in the 1960s with the release of Letraset sheets containing Lorem Ipsum passages, and more recently with desktop publishing software like Aldus PageMaker including versions of Lorem Ipsum. ", 
        slug: "septimo-post",
    };

    diesel::insert_into(posts).values(&new_post).get_result::<Post>(conn).expect("Error en insertar dato");
}

// select_limit: hace un select a la tabla post con un límite de elementos a mostrar
// @params: conexión y limite de rows
// void
fn select_limit(conn: &mut PgConnection, limit: i64) {
    use self::models::Post;
    use self::schema::posts::dsl::*;

    // let limit = 10; // limitar los resultados
    let posts_result = posts.limit(limit).load::<Post>(conn).expect("Error en consulta SQL");

    // Iteramos los resultados de la consulta
    for post in posts_result {
        println!("{:?} \n--------------", post);
    }
}


// select_all: hace un select a la tabla post y ordenado por id de forma descendente
// @params: conexión 
// void
fn select_all(conn: &mut PgConnection) {
    use self::models::Post;
    use self::schema::posts::dsl::*;

    let posts_result = posts.order(id.desc()).load::<Post>(conn).expect("Error en consulta SQL");


    // Iteramos los resultados de la consulta
    for post in posts_result {
        println!("{:?} \n--------------", post);
    }
}

// select_limit: hace un select a la tabla post con un límite de elementos a mostrar
// @params: conexión y limite de rows
// void
fn select_title_body_limit(conn: &mut PgConnection, limit: i64) {
    use self::schema::posts::dsl::*;

    // limit limitar los resultados
    let posts_result = posts.select((title, body)).limit(limit).load::<PostSimple>(conn).expect("Error en consulta SQL");


    // Iteramos los resultados de la consulta
    for post in posts_result {
        println!("{:?} \n--------------", post);
    }
}

// select_with_where_slug: busca con el slug que se ingresa
// @param: conexión y slug a buscar
// void
fn select_with_where_slug(conn: &mut PgConnection, input_slug: String) {
    // use self::models::Post;
    use self::schema::posts::dsl::*;

    // limit limitar los resultados
    let posts_result = posts.filter(slug.eq(input_slug)).select((title, body)).load::<PostSimple>(conn).expect("Error en consulta SQL");


    // Iteramos los resultados de la consulta
    for post in posts_result {
        println!("{:?} \n--------------", post);
    }
}

// update_title_slug: hace un select a la tabla post y ordenado por id de forma descendente
// @params: conexión, id para comparar, title y slug 
// void
fn update_slug(conn: &mut PgConnection, input_id: i32, input_slug: String) {
    use self::schema::posts::dsl::*;
    use self::models::Post;

    let post_update = diesel::update(posts.filter(id.eq(input_id))).set(slug.eq(input_slug)).get_result::<Post>(conn).expect("Error en el update");

    println!("update --> {:?} \n--------------\n\n", post_update);
}

// update_title_slug: hace un select a la tabla post y ordenado por id de forma descendente
// @params: conexión, id para comparar, title y slug 
// void
fn update_title_slug(conn: &mut PgConnection, input_id: i32, input_title: String, input_slug: String) {
    use self::schema::posts::dsl::*;
    use self::models::Post;

    let post_update = diesel::update(
        posts.filter(
            id.eq(input_id)
        )).set(
        (
            slug.eq(input_slug), 
            title.eq(input_title))
        ).get_result::<Post>(conn).expect("Error en el update");

    println!("update --> {:?} \n--------------\n\n", post_update);
}

// delete_post: elimina post por id
// @params: conexión y id para el filtro
// void 
fn delete_post(conn: &mut PgConnection, input_id: i32) {
    use self::schema::posts::dsl::*;

    diesel::delete(posts.filter(id.eq(input_id))).execute(conn).expect("Error al eliminar registro");
}


// delete_post_by_slug: elimina post por slug
// @params: conexión y slug para el filtro
// void 
// Nota: a modo de ejemplo borra todo con el filtro "%-post%"
fn delete_post_by_slug(conn: &mut PgConnection, input_slug: String) {
    use self::schema::posts::dsl::*;

    // input_slug = "%-post%"
    diesel::delete(posts.filter(slug.like(input_slug))).execute(conn).expect("Error al eliminar registro");
}

fn main() {

    // obtiene las conexiones 
    let conn  = &mut establish_connection();

    // función que permite agregar datos a la base de datos
    // add_new_post(conn);

    // SELECT * FROM posts
    let limit = 1; // limitador de resultados
    let id: i32 = 7;
    let title: String = String::from("Mi segundo blogpost");
    let slug: String = String::from("segundo-post");
    let delete_slug: String = String::from("%-post%");
    
    println!("limit: {} \nid: {}\n title: {}\nslug:{}\ndelete_slug {}  
    \n--------------\n\n", limit, id, title, slug, delete_slug);

    // DELETE TABLE posts WHERE id = ?
    // delete_post(conn, id);
    // delete_post_by_slug(conn, delete_slug);

    // UPDATE TABLE posts SET slug = '?' WHERE id = ?
    // update_slug(conn, id, slug);
    // update_title_slug(conn, id, title, slug);
    
    // select_limit(conn, limit); // limita los resultados
    select_all(conn); // obtiene todos los resultados
    // select_title_body_limit(conn, limit); // obtiene resultado/s simplificados y limitados
    // select_with_where_slug(conn, slug);

}
