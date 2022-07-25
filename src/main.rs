mod graphql;
mod nodes;

fn main() -> Result<(), anyhow::Error> {
    let country = String::from("Poland");

    let client = graphql::GraphqlClient::new("https://graphql.grid.tf/graphql")?;

    let nodes = client.get_nodes_by_country(country)?;

    println!("nodes: {:?}", nodes);

    Ok(())
}
