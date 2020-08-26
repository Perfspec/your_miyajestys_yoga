use actix_web::{web, App, HttpServer, Responder, HttpRequest, HttpResponse, Error};
use serde::{Serialize, Deserialize};
use futures::future::{ready, Ready};

#[derive(Serialize, Deserialize)]
struct User {
    user_id: u32
}

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

#[derive(Deserialize)]
struct UriParams {
    user_id: u32
}

#[derive(Serialize)]
struct UserId {
    user_id: u32
}

impl Responder for UserId {
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

// Begin Handlers

async fn get_user(path: web::Path<UriParams>) -> User {
	User { user_id: path.user_id }
}

async fn create_user(body: web::Json<User>) -> UserId {
	UserId { user_id: body.user_id }
}

// End Handlers

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	let ip_address = "127.0.0.1:8088";
	
    HttpServer::new(|| {
        App::new()
			.service(
				web::scope("/api")
					.route("/users", web::post().to(create_user))
					.route("/users/{user_id}", web::get().to(get_user)))
	})
    .bind(ip_address)?
    .run()
    .await
}