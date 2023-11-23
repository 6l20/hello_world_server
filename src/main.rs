use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use rand::Rng; // To generate random numbers
use serde::{Deserialize, Serialize};
use std::env;
use log::{info, trace, warn};
use rusqlite::{params, Connection, Result};

fn init_db() -> Result<()> {
    let conn = Connection::open("users.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
                  id INTEGER PRIMARY KEY,
                  name TEXT NOT NULL,
                  email TEXT NOT NULL UNIQUE
         )",
        params![],
    )?;
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    email: String,
}

// Define a structure for the version response
#[derive(Serialize)]
struct VersionInfo {
    version: String,
    date: String,
}

#[derive(Serialize)]
struct RandomNumber {
    number: u32,
    hex_string: String,
}

async fn greet() -> impl Responder {
    log::info!("Handling request for /");
    HttpResponse::Ok().body("Hello world!")
}

// Handler for the version endpoint
async fn version() -> impl Responder {
    log::info!("Handling request for /version");
    HttpResponse::Ok().json(VersionInfo {
        version: "0.1.0".to_string(),
        date: "06/01/2022".to_string(),
    })
}

// Handler for the random number endpoint
async fn random_number() -> impl Responder {
    log::info!("Handling request for /random_number");
    let mut rng = rand::thread_rng();
    let num: u32 = rng.gen_range(1001..=u32::MAX);

    HttpResponse::Ok().json(RandomNumber {
        number: num,
        hex_string: format!("{:X}", num),
    })
}

// Endpoint to create a new user
async fn create_user(user: web::Json<User>) -> impl Responder {
    log::info!("Handling request for /users (POST)");
    let conn = Connection::open("users.db").unwrap();
    conn.execute(
        "INSERT INTO users (name, email) VALUES (?1, ?2)",
        params![user.name, user.email],
    ).unwrap();

    HttpResponse::Ok().body("User created")
}

// Endpoint to retrieve all users
async fn get_users() -> impl Responder {
    log::info!("Handling request for /users (GET)");
    let conn = Connection::open("users.db").unwrap();
    let mut stmt = conn.prepare("SELECT id, name, email FROM users").unwrap();
    let user_iter = stmt.query_map(params![], |row| {
        Ok(User {
            name: row.get(1)?,
            email: row.get(2)?,
        })
    }).unwrap();

    let users: Vec<User> = user_iter.map(|user| user.unwrap()).collect();
    HttpResponse::Ok().json(users)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info"); // Set default log level to info if not specified
    env_logger::init(); // Initialize the logger

    init_db().unwrap();

    log::info!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/version", web::get().to(version))
            .route("/random_number", web::get().to(random_number))
            .route("/users", web::get().to(get_users))
            .route("/users", web::post().to(create_user))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
