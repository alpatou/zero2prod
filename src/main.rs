use actix_web::{http::StatusCode, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name: &str = req.match_info().get("name").unwrap_or("default");

    format!("Hello {}!", &name)
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    return HttpServer::new(|| App::new().route("/{health_check}", web::get().to(health_check)))
        .bind("127.0.0.1:8000")?
        .run()
        .await;
}
