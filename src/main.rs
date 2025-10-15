use actix_web::{get, App, HttpServer, HttpResponse, Responder};
use actix_cors::Cors;
use serde::Serialize;
use rand::Rng;

#[derive(Serialize, Clone)]
struct Quote {
    text: String,
    author: String,
}

#[get("/quote")]
async fn random_quote() -> impl Responder {
    let quotes = vec![
        Quote { text: "Stay hungry, stay foolish.".into(), author: "Steve Jobs".into() },
        Quote { text: "Simplicity is the ultimate sophistication.".into(), author: "Leonardo da Vinci".into() },
        Quote { text: "Do one thing every day that scares you.".into(), author: "Eleanor Roosevelt".into() },
        Quote { text: "In the middle of difficulty lies opportunity.".into(), author: "Albert Einstein".into() },
        Quote { text: "Happiness depends upon ourselves.".into(), author: "Aristotle".into() },
    ];

    let mut rng = rand::thread_rng();
    let quote = quotes[rng.gen_range(0..quotes.len())].clone();

    HttpResponse::Ok().json(quote)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Rust backend at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive()) // allow CORS for development
            .service(random_quote)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
