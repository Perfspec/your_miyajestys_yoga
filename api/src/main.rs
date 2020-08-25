use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn show_users()  -> impl Responder {
	HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	let host = "127.0.0.1";
	let port = "8088";
	let ip = format!("{}:{}", host, port);
	
    HttpServer::new(|| {
        App::new()
			.service(
				web::scope("/users")
					.route("/show", web::get().to(show_users)))
	})
    .bind(ip)?
    .run()
    .await
}