use actix_web::{post, web, App, HttpServer, Responder};
use rand::Rng;

#[post("/classify")]
async fn classify() -> impl Responder {
    let mut rng = rand::thread_rng();
    let is_simple: i32 = if rng.gen_bool(0.5) { 0 } else { 1 };
    web::Json(is_simple)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Classification service is active and running.");
    HttpServer::new(|| App::new().service(classify))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
