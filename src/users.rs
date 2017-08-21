use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::result::Error;
use models::{User, NewUser};
use schema::users;
use schema::users::dsl::*;
use bcrypt::{DEFAULT_COST, hash};

pub fn create_user<'a>(conn: &PgConnection, u_username: String, u_email: String, u_name: String, u_password: String) -> Result<User, Error> {
    let new_user = NewUser {
        username: u_username,
        email: u_email,
        name: u_name,
        password: hash(&u_password[..], DEFAULT_COST).unwrap()
    };

    diesel::insert(&new_user)
        .into(users::table)
        .get_result(conn)
}

pub fn get_user(conn: &PgConnection, user_id: i32) -> Option<User> {
    let db_user = users.filter(id.eq(user_id))
        .limit(1)
        .load::<User>(conn)
        .expect("Error loading user");

    let db_user = db_user.get(0);

    match db_user {
        Some(user) => Some(user.to_owned()),
        None => None
    }
}

pub fn get_by_username(conn: &PgConnection, u_username: &String) -> Option<User> {
    let db_user = users.filter(username.eq(u_username))
        .limit(1)
        .load::<User>(conn)
        .expect("Error loading user");

    let db_user = db_user.get(0);

    match db_user {
        Some(user) => Some(user.to_owned()),
        None => None
    }
}
