use actix_web::{HttpServer, App, web, HttpResponse, Responder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::fs;

async fn index() -> impl Responder {
    let html = fs::read_to_string("./html/index.html").expect("Oops an error occured.");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut builder = 
        SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

	HttpServer::new (|| {
		App::new()
			.route("/{filename:.*}", web::get().to(index))
            .service(web::scope("/app").route("/index.html", web::get().to(index)))
		})

	.bind_openssl("191.101.207.193:443", builder)?
	.run()
	.await
}
