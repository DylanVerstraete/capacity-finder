mod graphql;
mod nodes;

fn main() -> Result<(), anyhow::Error> {
    let country = String::from("Belgium");

    let client = graphql::GraphqlClient::new("https://graphql.dev.grid.tf/graphql")?;

    let nodes = client.get_nodes_by_country(country)?;

    println!("nodes: {:?}", nodes);

    Ok(())
}
