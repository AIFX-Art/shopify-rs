use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::interface::client::ShopifyClient;



impl ShopifyClient {
    pub fn new(store_name: &str, access_token: &str) -> Self {
        ShopifyClient {
            base_url: format!("https://{}.myshopify.com/admin/api/2024-07", store_name),
            access_token: access_token.to_string(),
            client: Client::new(),
        }
    }
}




