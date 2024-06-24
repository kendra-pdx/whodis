use api_authorize::authorize;
use sqlx::PgPool;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

mod api_authorize;
mod schema;
mod db;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let pg_pool = PgPool::connect("postgres://postgres:postgres@localhost/postgres")
        .await
        .expect("could not connect to db");

    let redis_client = redis::Client::open("redis://localhost").expect("could not create redis client");
    let redis_pool = r2d2::Pool::builder()
        .build(redis_client)
        .expect("could not create redis pool");

    schema::migrate(&pg_pool).await.expect("migrations failed");

    rocket::build()
        .manage(pg_pool)
        .manage(redis_pool)
        .mount("/", routes![index, authorize])
}
