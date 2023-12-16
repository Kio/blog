use dotenv::dotenv;
use std::env;
use actix_web::{get, App, HttpServer, Responder, middleware::Logger, web, HttpRequest, Error};
use actix_cors::Cors;
use sea_orm::{Database, DatabaseConnection, EntityTrait, QueryOrder};
use migration::{Migrator, MigratorTrait};
use entity::{post, post::Entity as Post};

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[get("/")]
async fn hello(_req: HttpRequest, data: web::Data<AppState>) -> Result<impl Responder, Error> {
    let conn = &data.conn;
    let posts = Post::find()
        .order_by_asc(post::Column::Id)
        .all(conn)
        .await
        .unwrap();
    Ok(web::Json(posts))
}
#[get("/posts/{id}")]
async fn get_post(req: HttpRequest, data: web::Data<AppState>) -> Result<impl Responder, Error> {
    let conn = &data.conn;
    let id = req.match_info().get("id").unwrap().parse::<i32>().unwrap();
    let post = Post::find_by_id(id).one(conn).await.unwrap();
    Ok(web::Json(post))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DB_URL must be set");
    let conn= Database::connect(db_url).await.unwrap();
    Migrator::up(&conn, None).await.unwrap();

    let state = AppState { conn };

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin();
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(web::Data::new(state.clone()))
            .service(hello)
            .service(get_post)
    })
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}
