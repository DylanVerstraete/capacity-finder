use ::reqwest::blocking::Client;
use anyhow::*;
// use geoutils::{Distance, Location};
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "src/nodes_query.graphql",
    response_derives = "Debug"
)]
struct NodesQuery;

fn main() -> Result<(), anyhow::Error> {
    let country = String::from("Belgium");

    let variables = nodes_query::Variables {
        country: country.to_string(),
    };

    let client = Client::builder()
        .user_agent("graphql-rust/0.10.0")
        .build()?;

    let response_body =
        post_graphql::<NodesQuery, _>(&client, "https://graphql.dev.grid.tf/graphql", variables)
            .unwrap();

    // info!("{:?}", response_body);
    println!("{:?}", response_body);

    Ok(())
}
