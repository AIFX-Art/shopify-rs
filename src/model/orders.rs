use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::interface::{
    client::ShopifyClient,
    order::{Order, OrderCount, OrderList},
};

impl ShopifyClient {
    pub async fn create_order(&self, order: &Order) -> Result<Order, Box<dyn Error>> {
        let url = format!("{}/orders.json", self.base_url);
        let res = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("X-Shopify-Access-Token", &self.access_token)
            .json(&order)
            .send()
            .await?;

        let new_order: Order = res.json().await?;
        Ok(new_order)
    }

    pub async fn get_order(&self, order_id: u64) -> Result<Order, Box<dyn Error>> {
        let url = format!(
            "{}/orders/{}.json?fields=id,line_items,name,total_price",
            self.base_url, order_id
        );
        let res = self
            .client
            .get(&url)
            .header("Content-Type", "application/json")
            .header("X-Shopify-Access-Token", &self.access_token)
            .send()
            .await?;

        let order: Order = res.json().await?;
        Ok(order)
    }

    pub async fn cancel_order(&self, order_id: u64) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/orders/{}/cancel.json", self.base_url, order_id);
        self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("X-Shopify-Access-Token", &self.access_token)
            .send()
            .await?;
        Ok(())
    }

    pub async fn close_order(&self, order_id: u64) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/orders/{}/close.json", self.base_url, order_id);
        self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("X-Shopify-Access-Token", &self.access_token)
            .send()
            .await?;
        Ok(())
    }

    pub async fn reopen_order(&self, order_id: u64) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/orders/{}/open.json", self.base_url, order_id);
        self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .header("X-Shopify-Access-Token", &self.access_token)
            .send()
            .await?;
        Ok(())
    }

    pub async fn get_orders(&self, status: &str) -> Result<OrderList, Box<dyn Error>> {
        let url = format!("{}/orders.json?status={}", self.base_url, status);
        let res = self
            .client
            .get(&url)
            .header("Content-Type", "application/json")
            .header("X-Shopify-Access-Token", &self.access_token)
            .send()
            .await
            .unwrap();

        let response_body = res.text().await.unwrap();
        println!("{}", response_body);
        //

        //let orders: OrderList = res.json().await.unwrap();
        let orders = OrderList {
            orders: vec![]
        };
        //println!("order {:?}", orders);
        Ok(orders)
    }

    pub async fn get_order_count(&self, status: &str) -> Result<OrderCount, Box<dyn Error>> {
        let url = format!("{}/orders/count.json?status={}", self.base_url, status);
        let res = self
            .client
            .get(&url)
            .header("Content-Type", "application/json")
            .header("X-Shopify-Access-Token", &self.access_token)
            .send()
            .await?;

        let count: OrderCount = res.json().await?;
        Ok(count)
    }

    pub async fn update_order(
        &self,
        order_id: u64,
        order: &Order,
    ) -> Result<Order, Box<dyn Error>> {
        let url = format!("{}/orders/{}.json", self.base_url, order_id);
        let res = self
            .client
            .put(&url)
            .header("Content-Type", "application/json")
            .header("X-Shopify-Access-Token", &self.access_token)
            .json(&order)
            .send()
            .await?;

        let updated_order: Order = res.json().await?;
        Ok(updated_order)
    }

    pub async fn delete_order(&self, order_id: u64) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/orders/{}.json", self.base_url, order_id);
        self.client
            .delete(&url)
            .header("Content-Type", "application/json")
            .header("X-Shopify-Access-Token", &self.access_token)
            .send()
            .await?;
        Ok(())
    }
}
