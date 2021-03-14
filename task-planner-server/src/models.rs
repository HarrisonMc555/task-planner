use super::schema::users;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub display_name: String,
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub display_name: &'a str,
}
