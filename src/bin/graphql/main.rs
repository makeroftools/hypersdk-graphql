pub mod schema;

use std::error::Error;

use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema };
use async_graphql_axum::GraphQL;
use axum::{
    Router,
    response::{self, IntoResponse},
    routing::get
};

use tokio::net::TcpListener;

use schema::Query;


async fn graphiql() -> impl IntoResponse {
    // Html(GraphiQLSource::build().finish())
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // create the schema
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    // start the http server
    let app = Router::new().route("/", get(graphiql).post_service(GraphQL::new(schema)));
    println!("GraphiQL: http://localhost:8000");
    axum::serve(TcpListener::bind("127.0.0.1:8000").await.unwrap(), app)
        .await
        .unwrap();
    Ok(())
}