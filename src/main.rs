use actix_web::{HttpServer, App, web, HttpResponse, Responder};
use std::fs;

async fn index() -> impl Responder {
    let html = fs::read_to_string("./html/index.html").expect("Oops an error occured.");
    HttpResponse::Ok()
        .content_type("text/html")
        .body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new (|| {
		App::new()
			.route("/{filename:.*}", web::get().to(index))
            .service(web::scope("/app").route("/index.html", web::get().to(index)))
		})

	.bind("191.101.207.193:80")?
	.run()
	.await
}
