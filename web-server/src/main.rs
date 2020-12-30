use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::fs;

async fn index() -> impl Responder {
    let html = fs::read_to_string("../front/Vues/index.html").expect("Oops an error occured.");

    HttpResponse::Ok().content_type("text/html").body(html)
}

async fn index_js() -> impl Responder {
    let html = fs::read_to_string("../front/Scripts/index.js").expect("Oops an error occured.");

    HttpResponse::Ok()
        .content_type("text/javascript")
        .body(html)
}

async fn btn_stylesheet() -> impl Responder {
    let html =
        fs::read_to_string("../front/Stylesheets/button.css").expect("Oops an error occured.");

    HttpResponse::Ok().content_type("text/css").body(html)
}

async fn background_stylesheet() -> impl Responder {
    let html =
        fs::read_to_string("../front/Stylesheets/background.css").expect("Oops an error occured.");

    HttpResponse::Ok().content_type("text/css").body(html)
}

async fn title_stylesheet() -> impl Responder {
    let html =
        fs::read_to_string("../front/Stylesheets/title.css").expect("Oops an error occured.");

    HttpResponse::Ok().content_type("text/css").body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //This is without openssl, for test purposes
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/index.js", web::get().to(index_js))
            .route("/button.css", web::get().to(btn_stylesheet))
            .route("/background.css", web::get().to(background_stylesheet))
            .route("/title.css", web::get().to(title_stylesheet))
    })
    
    //This is for prod site 
    .bind_openssl("127.0.0.1:4242", builder)?

    //This is for local test
    //.bind("127.0.0.1:4242")?
    
    .run()
    .await
}
