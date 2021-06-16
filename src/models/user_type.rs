use super::schema::user_type;

#[derive(Queryable)]
pub struct UserType {
    pub id: i32,
    pub type: String
}

#[derive(Insertable)]
#[table_name="user_type"]
pub struct NewUserType<'a> {
    pub type: &'a str,
    pub body: &'a str,
}