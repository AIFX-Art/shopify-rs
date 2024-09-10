use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    pub id: u64,
    pub product_id: u64,
    pub position: u32,
    pub created_at: String,
    pub updated_at: String,
    pub width: u32,
    pub height: u32,
    pub src: String,
    pub variant_ids: Vec<Option<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductOption {
    pub id: u64,
    pub product_id: u64,
    pub name: String,
    pub position: u32,
    pub values: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Variant {
    pub id: u64,
    pub product_id: u64,
    pub title: String,
    pub price: String,
    pub sku: String,
    pub position: u32,
    pub inventory_policy: String,
    pub compare_at_price: Option<String>,
    pub fulfillment_service: String,
    pub inventory_management: Option<String>,
    pub option1: String,
    pub option2: Option<String>,
    pub option3: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub taxable: bool,
    pub barcode: Option<String>,
    pub grams: u32,
    pub image_id: Option<u64>,
    pub inventory_quantity: i32,
    pub weight: f64,
    pub weight_unit: String,
    pub inventory_item_id: u64,
    pub requires_shipping: bool,
    pub admin_graphql_api_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: u64,
    pub title: String,
    pub body_html: Option<String>,
    pub vendor: String,
    pub product_type: String,
    pub created_at: String,
    pub handle: String,
    pub updated_at: String,
    pub published_at: Option<String>,
    pub template_suffix: Option<String>,
    pub published_scope: String,
    pub tags: String,
    pub admin_graphql_api_id: String,
    pub variants: Vec<Variant>,
    pub options: Vec<ProductOption>,
    pub images: Vec<Image>,
    pub image: Option<Image>, // main image
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductCreateUpdate {
    pub product: Product,
}

// Struct for counting products response
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductCount {
    pub count: u64,
}

// Struct for a list of products response
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductList {
    pub products: Vec<Product>,
}