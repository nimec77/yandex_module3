use actix_web::{App, HttpResponse, HttpServer, Result, web};

async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().body("Hello, world!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
