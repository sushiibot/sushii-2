use actix_cors::Cors;
use actix_web::{middleware, web, App, Error, HttpResponse, HttpServer};
use juniper::{EmptyMutation, EmptySubscription, RootNode};
use juniper_actix::{
    graphiql_handler as gqli_handler, graphql_handler, playground_handler as play_handler,
};
use sqlx::postgres::PgPoolOptions;
use std::env;
use sushii_model::model::juniper::Context;
use tracing_subscriber::filter::{EnvFilter, LevelFilter};

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

    let allowed_origin =
        env::var("ALLOWED_ORIGIN").unwrap_or_else(|_| "http://localhost:3000".into());

    let server = HttpServer::new(move || {
        App::new()
            .data(schema())
            .data(pool.clone())
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .wrap(
                Cors::default()
                    .allowed_origin(&allowed_origin)
                    .allowed_origin_fn(|origin, head| {
                        dbg!(origin, head);

                        true
                    })
                    .allow_any_header()
                    .allowed_methods(vec!["GET", "HEAD", "POST", "OPTIONS"])
                    // Local testing origins
                    .allowed_origin("http://127.0.0.1:8080")
                    .allowed_origin("http://localhost:3000")
                    .allowed_origin("http://localhost:3000/leaderboard")
                    .max_age(3600),
            )
            .service(
                web::resource("/")
                    .route(web::post().to(graphql))
                    .route(web::get().to(graphql)),
            )
            .service(web::resource("/playground").route(web::get().to(playground_handler)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql_handler)))
    });
    server.bind("127.0.0.1:8080").unwrap().run().await
}
