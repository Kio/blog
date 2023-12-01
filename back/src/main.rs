use actix_web::{get, App, HttpResponse, HttpServer, Responder, middleware::Logger};


#[get("/")]
async fn hello() -> impl Responder {

    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(hello)
    })
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}
