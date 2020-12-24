use actix_cors::Cors;
use actix_web::{http::{header, Method}, middleware, web, App, Error, Route, HttpResponse, HttpServer};
use juniper::{EmptyMutation, EmptySubscription, RootNode};
use juniper_actix::{
    graphiql_handler as gqli_handler, graphql_handler, playground_handler as play_handler,
};
use sqlx::postgres::PgPoolOptions;
use std::env;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};
use sushii_model::model::juniper::Context;

mod model;
mod relay;

use model::Query;

type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Context>::new(),
        EmptySubscription::<Context>::new(),
    )
}

async fn graphiql_handler() -> Result<HttpResponse, Error> {
    gqli_handler("/", None).await
}
async fn playground_handler() -> Result<HttpResponse, Error> {
    play_handler("/", None).await
}

async fn graphql(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<Schema>,
    pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, Error> {
    let ctx = Context::new((*pool).clone());

    graphql_handler(&schema, &ctx, req, payload).await
}

async fn cors_event() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "PUT, GET, OPTIONS")
        .header("Access-Control-Allow-Headers", "Content-Type, Authorization, Accept")
        .body(""))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into()))
        .init();

    let db_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL in environment");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");

    let server = HttpServer::new(move || {
        App::new()
            .data(schema())
            .data(pool.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::permissive()
                /*
                Cors::default()
                    .send_wildcard()
                    .allowed_methods(vec!["POST", "GET", "OPTIONS"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
                    */
            )
            .service(
                web::resource("/")
                    .route(web::post().to(graphql))
                    .route(web::get().to(graphql))
                    .route(Route::new().method(Method::OPTIONS).to(cors_event)),
            )
            .service(web::resource("/playground").route(web::get().to(playground_handler)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql_handler)))
    });
    server.bind("127.0.0.1:8080").unwrap().run().await
}
