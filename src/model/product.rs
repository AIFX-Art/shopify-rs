use reqwest::Client;

use crate::{interface::product::{Product, ProductCount, ProductCreateUpdate, ProductList}, ShopifyClient};



impl ShopifyClient {

    pub async fn create_product(&self, product: &ProductCreateUpdate) -> Result<Product, Box<dyn std::error::Error>> {
        let url = format!("{}/products.json", self.base_url);
        let res = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("X-Shopify-Access-Token", &self.access_token)
            .json(&product)
            .send()
            .await?;
        let new_product: Product = res.json().await?;
        Ok(new_product)
    }

    pub async fn get_product(&self, product_id: u64) -> Result<Product, Box<dyn std::error::Error>> {
        let url = format!("{}/products/{}.json", self.base_url, product_id);
        let res = self.client
            .get(&url)
            .header("Content-Type", "application/json")
            .header("X-Shopify-Access-Token", &self.access_token)
            .send()
            .await?;
        let product: Product = res.json().await?;
        Ok(product)
    }

    pub async fn get_products(&self, ids: Option<&str>, status: Option<&str>) -> Result<ProductList, Box<dyn std::error::Error>> {
        let query = format!(
            "{}{}",
            ids.map(|i| format!("ids={}", i)).unwrap_or_default(),
            status.map(|s| format!("&status={}", s)).unwrap_or_default()
        );
        let url = if query.is_empty() {
            format!("{}/products.json", self.base_url)
        } else {
            format!("{}/products.json?{}", self.base_url, query)
        };
        let res = self.client
            .get(&url)
            .header("Content-Type", "application/json")
            .header("X-Shopify-Access-Token", &self.access_token)
            .send()
            .await?;
        let products: ProductList = res.json().await?;
        Ok(products)
    }

    pub async fn get_product_count(&self) -> Result<ProductCount, Box<dyn std::error::Error>> {
        let url = format!("{}/products/count.json", self.base_url);
        let res = self.client
            .get(&url)
            .header("Content-Type", "application/json")
            .header("X-Shopify-Access-Token", &self.access_token)
            .send()
            .await?;
        let count: ProductCount = res.json().await?;
        Ok(count)
    }

    pub async fn update_product(&self, product_id: u64, product: &ProductCreateUpdate) -> Result<Product, Box<dyn std::error::Error>> {
        let url = format!("{}/products/{}.json", self.base_url, product_id);
        let res = self.client
            .put(&url)
            .header("Content-Type", "application/json")
            .header("X-Shopify-Access-Token", &self.access_token)
            .json(&product)
            .send()
            .await?;
        let updated_product: Product = res.json().await?;
        Ok(updated_product)
    }

    pub async fn delete_product(&self, product_id: u64) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/products/{}.json", self.base_url, product_id);
        self.client
            .delete(&url)
            .header("Content-Type", "application/json")
            .header("X-Shopify-Access-Token", &self.access_token)
            .send()
            .await?;
        Ok(())
    }
}