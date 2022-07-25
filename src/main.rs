mod graphql;
mod nodes;

fn main() -> Result<(), anyhow::Error> {
    let country = String::from("Poland");

    let client = graphql::GraphqlClient::new("https://graphql.grid.tf/graphql")?;

    let nodes_by_country = client.clone().get_nodes_by_country(country)?;
    println!("nodes by country: {:?}", nodes_by_country);

    let city = String::from("Lochristi");
    let nodes_by_city = client.get_nodes_by_city(city)?;
    println!("nodes by city: {:?}", nodes_by_city);

    Ok(())
}
