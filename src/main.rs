use actix_web::web::Data;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use ed25519_dalek::Digest;
use ed25519_dalek::Signature;
use ed25519_dalek::SignatureError;
use ed25519_dalek::SigningKey;

use rand::rngs::OsRng;
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use sha2::Sha512;
use std::env;

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

// Generate a new keypair eddsa based on the ed25519 curve
fn generate_keypair() -> SigningKey {
    let mut csprng = OsRng {};
    let signing_key: SigningKey = SigningKey::generate(&mut csprng);
    signing_key
}



// Define a structure for the version response
#[derive(Serialize)]
struct VersionInfo {
    version: String,
    date: String,
    signature: String,
}

#[derive(Serialize)]
struct RandomNumber {
    number: u32,
    hex_string: String,
}



// Handler for the version endpoint
async fn version(data: web::Data<SigningKey>) -> impl Responder {
    log::info!("Handling request for /version");

    let context: &[u8] = b"Ed25519DalekSignPrehashedDoctest";

    // Create a hash digest object which we'll feed the message into:
    let mut prehashed: Sha512 = Sha512::new();

    prehashed.update(context);

    let sig: Result<Signature, SignatureError> = data.sign_prehashed(prehashed, Some(context));

    HttpResponse::Ok().json(VersionInfo {
        version: "0.1.0".to_string(),
        date: "06/01/2022".to_string(),
        signature: "0x".to_string() + &hex::encode(sig.unwrap().to_bytes()),
    })
}

mod models;



mod config;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info"); // Set default log level to info if not specified
    env_logger::init(); // Initialize the logger

    let signing_key = generate_keypair();

    config::print_config();

    init_db().unwrap();

    log::info!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(signing_key.clone()))
            .route("/", web::get().to(routes::greet_route::greet))
            .route("/version", web::get().to(version))
            .route("/random_number", web::get().to(routes::random_route::random_number))
            .route("/users", web::get().to(routes::user_route::get_users))
            .route("/users", web::post().to(routes::user_route::create_user))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
