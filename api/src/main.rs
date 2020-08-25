use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse, Error};
use serde::Serialize;
use futures::future::{ready, Ready};

#[derive(Serialize)]
struct User {
    id: String,
}

// Responder
impl Responder for User {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();

        // Create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

async fn show_user(path: web::Path<u32>) -> User {
	User { id: path.to_string() }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	let host = "127.0.0.1";
	let port = "8088";
	let ip_address = format!("{}:{}", host, port);
	
    HttpServer::new(|| {
        App::new()
			.service(
				web::scope("/users")
					.route("/{userid}", web::get().to(show_user)))
	})
    .bind(ip_address)?
    .run()
    .await
}