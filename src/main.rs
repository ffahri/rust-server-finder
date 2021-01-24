use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Hello, world!");
    let client = reqwest::Client::new();
    let resp = client.get("https://api.battlemetrics.com/servers")
        .query(&[
            ("filter[game]", "rust"),
            ("filter[status]", "online"),
            ("filter[countries][]", "TR"),
            ("filter[players][min]", "50"),
        ]).send().await?.text().await?;

    println!("{}", resp);
    Ok(())
}
