# Shopify API Rust Interface
This is a work in progress and will change as we build up the api.


```rs
    // load environment variables from .env file
    dotenv().expect(".env file not found");
    let store_name = env::var("SHOPIFY_STORE_NAME").expect("ENV SHOPIFY_STORE_NAME must be set");
    let access_token =
        env::var("SHOPIFY_ACCESS_TOKEN").expect("ENV SHOPIFY_ACCESS_TOKEN must be set");

    let client = ShopifyClient::new(&store_name, &access_token);

    // Getting an order
    match client.get_orders("any").await {        
        Ok(orders) => println!("Fetched Orders: {:?}", orders),
        Err(e) => eprintln!("Error while fetching order: {}", e),
    }
```
Some Examples in the main.rs file
