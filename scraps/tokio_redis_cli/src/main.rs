use core::str;
use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, Redis Cli");

    // open a connection to the mini-redis address
    let mut client = client::connect("127.0.0.1:6379").await?;

    let data = client.get("Foo").await?.unwrap_or_default();

    let txt = str::from_utf8(data.as_ref()).unwrap();

    println!("Got txt [{:?}]", txt);

    let new_data = format!("{}-{}", txt, "Bar");

    // Set a key
    client.set("Foo", new_data.into()).await?;

    // Get the key back
    let result = client.get("Foo").await?;

    println!("Got value from the server: {:?}", result);

    Ok(())
}
