use crate::graphql::GraphqlClient;
use anyhow::*;
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.json",
    query_path = "src/nodes/queries/nodes_query.graphql",
    response_derives = "Debug"
)]
struct NodesQuery;

impl GraphqlClient {
    pub fn get_nodes_by_country(self, country: String) -> Result<Vec<i64>, anyhow::Error> {
        let variables = nodes_query::Variables {
            country: country.to_string(),
        };

        let response_body = post_graphql::<NodesQuery, _>(&self.client, self.url, variables)?;

        let response_data: nodes_query::ResponseData =
            response_body.data.expect("missing response data");

        let ids: Vec<i64> = response_data
            .nodes
            .into_iter()
            .map(|node| node.node_id)
            .collect();

        Ok(ids)
    }
}
