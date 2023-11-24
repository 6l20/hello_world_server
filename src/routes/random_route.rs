use actix_web::{Responder, HttpResponse};
use rand::Rng;

use crate::RandomNumber;

// Handler for the random number endpoint
pub async fn random_number() -> impl Responder {
    log::info!("Handling request for /random_number");
    let mut rng = rand::thread_rng();
    let num: u32 = rng.gen_range(1001..=u32::MAX);

    HttpResponse::Ok().json(RandomNumber {
        number: num,
        hex_string: format!("{:X}", num),
    })
}