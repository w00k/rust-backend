// @generated automatically by Diesel CLI.
// macro 
diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        slug -> Varchar,
        body -> Text,
    }
}
