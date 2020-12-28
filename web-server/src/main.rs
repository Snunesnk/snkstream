use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::fs;

async fn index() -> impl Responder {
    let html = fs::read_to_string("../front/Vues/index.html").expect("Oops an error occured.");

    HttpResponse::Ok().body(html)
}

async fn indexjs() -> impl Responder {
    let html = fs::read_to_string("../front/Scripts/index.js").expect("Oops an error occured.");

    HttpResponse::Ok().body(html)
}

async fn btnstylesheet() -> impl Responder {
    let html = fs::read_to_string("../front/Stylesheets/button.css").expect("Oops an error occured.");

    HttpResponse::Ok().body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // This is with openssl, to deploy on teh server
    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // builder
    //     .set_private_key_file("key.pem", SslFiletype::PEM)
    //     .unwrap();
    // builder.set_certificate_chain_file("cert.pem").unwrap();

    // HttpServer::new(|| {
    //     App::new()
    //         .route("/", web::get().to(index))
    //         .route("/{name}", web::get().to(index))
    // })
    // .bind_openssl("127.0.0.1:4242", builder)?
    // .run()
    // .await

    //This is without openssl, for test purposes
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/index.js", web::get().to(indexjs))
            .route("/button.css", web::get().to(btnstylesheet))
    })
    .bind("127.0.0.1:4242")?
    .run()
    .await
}
