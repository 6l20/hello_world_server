use actix_web::{Responder, HttpResponse, web};
use rusqlite::{Connection, params};

// Import the User struct from the user_model module
use crate::models::user_model::User;

// Endpoint to create a new user
pub(crate) async fn create_user(user: web::Json<User>) -> impl Responder {
    log::info!("Handling request for /users (POST)");
    let conn = Connection::open("users.db").unwrap();
    conn.execute(
        "INSERT INTO users (name, email) VALUES (?1, ?2)",
        params![user.name, user.email],
    )
    .unwrap();

    HttpResponse::Ok().body("User created")
}

// Endpoint to retrieve all users
pub(crate) async fn get_users() -> impl Responder {
    log::info!("Handling request for /users (GET)");
    let conn = Connection::open("users.db").unwrap();
    let mut stmt = conn.prepare("SELECT id, name, email FROM users").unwrap();
    let user_iter = stmt
        .query_map(params![], |row| {
            Ok(User {
                name: row.get(1)?,
                email: row.get(2)?,
            })
        })
        .unwrap();

    let users: Vec<User> = user_iter.map(|user| user.unwrap()).collect();
    HttpResponse::Ok().json(users)
}