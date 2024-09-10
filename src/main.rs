use dotenvy::dotenv;
use shopify_rs::{
    interface::order::{CreateOrderInput, Order},
    ShopifyClient,
};
use std::env;

#[tokio::main]
async fn main() {
    // load environment variables from .env file
    dotenv().expect(".env file not found");
    let store_name = env::var("SHOPIFY_STORE_NAME").expect("ENV SHOPIFY_STORE_NAME must be set");
    let access_token =
        env::var("SHOPIFY_ACCESS_TOKEN").expect("ENV SHOPIFY_ACCESS_TOKEN must be set");

    let client = ShopifyClient::new(&store_name, &access_token);

    // Test getting an order
    match client.get_orders("any").await {
        // Replace with a valid order_id
        Ok(orders) => println!("Fetched Orders: {:?}", orders),
        Err(e) => eprintln!("Error while fetching order: {}", e),
    }

    // Test creating an order
    /* let order = CreateOrderInput {
           line_items: todo!(),
           transactions: todo!(),
           total_tax: todo!(),
           currency: todo!(),
       };

       match client.create_order(&order).await {
           Ok(created_order) => println!("Created Order: {:?}", created_order),
           Err(e) => eprintln!("Error while creating order: {}", e),
       }
    */
}
