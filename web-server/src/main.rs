use actix_web::{web, App, HttpResponse, HttpServer, HttpRequest, Responder, guard};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::fs;
use actix_files as act_fs;
use actix_web::error::Result as act_Result;

async fn not_found(req: HttpRequest) -> impl Responder {
    println!("Cannot access given URL for request: {:?}.\n", req);
    HttpResponse::Ok().content_type("text/html").body("URL not found")
}

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


//Services loading several favicons
async fn android_192() -> act_Result<act_fs::NamedFile> {
    Ok(act_fs::NamedFile::open("../favicons/android-chrome-192x192.png")?)
}
async fn android_512() -> act_Result<act_fs::NamedFile> {
    Ok(act_fs::NamedFile::open("../favicons/android-chrome-512x512.png")?)
}
async fn apple() -> act_Result<act_fs::NamedFile> {
    Ok(act_fs::NamedFile::open("../favicons/apple-touch-icon.png")?)
}
async fn fav_16() -> act_Result<act_fs::NamedFile> {
    Ok(act_fs::NamedFile::open("../favicons/favicon-16x16.png")?)
}
async fn fav_32() -> act_Result<act_fs::NamedFile> {
    Ok(act_fs::NamedFile::open("../favicons/favicon-32x32.png")?)
}
async fn fav() -> act_Result<act_fs::NamedFile> {
    Ok(act_fs::NamedFile::open("../favicons/favicon.ico")?)
}
async fn webmanifest() -> act_Result<act_fs::NamedFile> {
    Ok(act_fs::NamedFile::open("../favicons/site.webmanifest")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("privkey.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("fullchain.pem").unwrap();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/index.js", web::get().to(index_js))
            .route("/button.css", web::get().to(btn_stylesheet))
            .route("/background.css", web::get().to(background_stylesheet))
            .route("/title.css", web::get().to(title_stylesheet))
            .route("/android-chrome-192x192.png", web::get().to(android_192))
            .route("/android-chrome-512x512.png", web::get().to(android_512))
            .route("/apple-touch-icon.png", web::get().to(apple))
            .route("/favicon-16x16.png", web::get().to(fav_16))
            .route("/favicon-32x32.png", web::get().to(fav_32))
            .route("/favicon.ico", web::get().to(fav))
            .route("/site.webmanifest", web::get().to(webmanifest))
            .default_service(
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .to(not_found),
            )
    })
    .bind_openssl("127.0.0.1:4242", builder)?
    .run()
    .await
}
