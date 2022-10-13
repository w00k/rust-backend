#[derive(Queryable)]
pub struct Post {
    pub id: i32, 
    pub title: String,
    pub slug: String, 
    pub body: String,
}

use super::schema::posts;

#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost<'a> {
    pub title: &'a str, 
    pub body: &'a str, 
    pub slug: &'a str,
}