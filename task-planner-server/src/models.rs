#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub display_name: String,
}