use dotenv::dotenv;
use std::env;
use actix_web::{get, App, HttpServer, Responder, middleware::Logger, web, HttpRequest, Error};
use actix_cors::Cors;
use sea_orm::{ColumnTrait, Database, DatabaseConnection, EntityTrait, ModelTrait, QueryOrder};
use migration::{Migrator, MigratorTrait};
use entity::{post, post::Entity as Post, post_tag::Entity as PostTag, tag, tag::Entity as Tag};
use sea_orm::LoaderTrait;
use sea_orm::QueryFilter;
use serde::Serialize;

#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[derive(Serialize)]
struct PostResponse {
    post: post::Model,
    tags: Vec<tag::Model>,
}

#[derive(Serialize)]
struct TagResponse {
    tag: tag::Model,
    posts: Vec<PostResponse>,
}

#[get("/")]
async fn index(_req: HttpRequest, data: web::Data<AppState>) -> Result<impl Responder, Error> {
    let conn = &data.conn;
    let posts = Post::find()
        .order_by_asc(post::Column::Id)
        .all(conn)
        .await
        .unwrap();
    let tags = posts.load_many_to_many(Tag, PostTag, conn).await.unwrap();
    let mut response: Vec<PostResponse>  = Vec::new();
    for (post, tags) in posts.into_iter().zip(tags.into_iter()) {
        response.push(PostResponse {
            post,
            tags,
        });
    }
    Ok(web::Json(response))
}


#[get("/posts/{id}")]
async fn get_post(req: HttpRequest, data: web::Data<AppState>) -> Result<impl Responder, Error> {
    let conn = &data.conn;
    let id = req.match_info().get("id").unwrap().parse::<i32>().unwrap();
    let post: post::Model = Post::find_by_id(id)
        .one(conn)
        .await
        .unwrap()
        .unwrap();
    let tags = post.find_related(Tag)
        .all(conn)
        .await
        .unwrap();
    let response = PostResponse {
        post,
        tags,
    };
    Ok(web::Json(response))
}

#[get("/tags/{slug}")]
async fn get_tag(req: HttpRequest, data: web::Data<AppState>) -> Result<impl Responder, Error> {
    let conn = &data.conn;
    let slug = req.match_info().get("slug").unwrap().to_string();
    let tag: tag::Model = Tag::find()
        .filter(tag::Column::Slug.eq(slug))
        .one(conn)
        .await
        .unwrap()
        .unwrap();
    let posts = tag.find_related(Post)
        .all(conn)
        .await
        .unwrap();
    let mut response: Vec<PostResponse>  = Vec::new();
    let tags = posts.load_many_to_many(Tag, PostTag, conn).await.unwrap();
    for (post, tags) in posts.into_iter().zip(tags.into_iter()) {
        response.push(PostResponse {
            post,
            tags,
        });
    }
    let response = TagResponse {
        tag,
        posts: response,
    };
    Ok(web::Json(response))
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
            .service(index)
            .service(get_post)
            .service(get_tag)
    })
        .bind(("0.0.0.0", 8000))?
        .run()
        .await
}
