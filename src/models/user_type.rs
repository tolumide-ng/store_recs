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


#[derive(Insertable)]
#[table_name="user_info"]
pub struct NewUserInfo<'a> {
    first_name: &'a str,
    last_name: &'a str,
    email: &'a str,
    phone: Option<&'a str>,
    phone_code: Option<u32>
}



#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}


#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}