use reqwest::Client;

pub struct ShopifyClient {
    pub base_url: String,
    pub access_token: String,
    pub client: Client,
}