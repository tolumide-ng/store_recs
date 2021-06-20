// use super::schema::user_type;
use crate::schema::user_type;
use crate::schema::user_info;
use diesel::pg::PgConnection;
// use diesel::prelude::{Insertable, Queryable};
use serde::Deserialize;
use diesel::prelude::*;
// use crate::schema::user_type;

#[derive(Queryable)]
pub struct UserType {
    pub id: i32,
    pub auth_type: String
}

// #[derive(Insertable)]
// #[table_name="user_info"]
// pub struct NewUserType<'a> {
//     // pub auth_type: &'a str,
//     pub body: &'a str,
// }


#[derive(Insertable)]
#[table_name="user_info"]
pub struct NewUserInfo<'a> {
    first_name: &'a str,
    last_name: &'a str,
    email: &'a str,
    phone: Option<&'a str>,
    phone_code: Option<i32>,
    password: &'a str,
    birth_country: &'a str,
    residing_country: &'a str
}



#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}


// #[derive(Insertable)]
// #[table_name="posts"]
// pub struct NewPost<'a> {
//     pub title: &'a str,
//     pub body: &'a str,
// }