use crate::graphql::GraphqlClient;
use anyhow::*;
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "src/nodes/queries/nodes_by_country_query.graphql",
    response_derives = "Debug"
)]
struct NodesByCountryQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "src/nodes/queries/nodes_by_city_query.graphql",
    response_derives = "Debug"
)]
struct NodesByCityQuery;

impl GraphqlClient {
    pub fn get_nodes_by_country(self, country: String) -> Result<Vec<i64>, anyhow::Error> {
        let variables = nodes_by_country_query::Variables {
            country: country.to_string(),
        };

        let response_body =
            post_graphql::<NodesByCountryQuery, _>(&self.client, self.url, variables)?;

        let response_data: nodes_by_country_query::ResponseData =
            response_body.data.expect("missing response data");

        let ids: Vec<i64> = response_data
            .nodes
            .into_iter()
            .map(|node| node.node_id)
            .collect();

        Ok(ids)
    }

    pub fn get_nodes_by_city(self, city: String) -> Result<Vec<i64>, anyhow::Error> {
        let variables = nodes_by_city_query::Variables {
            city: city.to_string(),
        };

        let response_body = post_graphql::<NodesByCityQuery, _>(&self.client, self.url, variables)?;

        let response_data: nodes_by_city_query::ResponseData =
            response_body.data.expect("missing response data");

        let ids: Vec<i64> = response_data
            .nodes
            .into_iter()
            .map(|node| node.node_id)
            .collect();

        Ok(ids)
    }
}
