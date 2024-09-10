use serde::{Deserialize, Serialize};
use chrono::{DateTime, FixedOffset};

#[derive(Serialize, Deserialize, Debug)]
pub struct Refund {
    pub created_at: String,
    pub duties: Vec<Duty>,
    pub id: i64,
    pub note: Option<String>,
    pub order_adjustments: Vec<OrderAdjustment>,
    pub processed_at: String,
    pub refund_duties: Option<Vec<RefundDuty>>,
    pub refund_line_items: Vec<RefundLineItem>,
    pub restock: bool,
    pub transactions: Vec<Transaction>,
    pub user_id: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Duties {
    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Duty {
    pub duty_id: i64,
    pub amount_set: AmountSet,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AmountSet {
    pub shop_money: Money,
    pub presentment_money: Money,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Money {
    pub amount: String,
    pub currency_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderAdjustment {
    pub id: i64,
    pub order_id: i64,
    pub refund_id: i64,
    pub amount: String,
    pub tax_amount: String,
    pub kind: String,
    pub reason: String,
    pub amount_set: AmountSet,
    pub tax_amount_set: AmountSet,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RefundDuty {
    pub duty_id: i64,
    pub refund_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RefundLineItem {
    pub id: i64,
    pub line_item: serde_json::Value,  // Since the line_item object is empty
    pub line_item_id: i64,
    pub quantity: i64,
    pub location_id: Option<i64>,
    pub restock_type: String,
    pub subtotal: f64,
    pub total_tax: f64,
    pub subtotal_set: AmountSet,
    pub total_tax_set: AmountSet,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    pub id: i64,
    pub order_id: i64,
    pub amount: String,
    pub kind: String,
    pub gateway: String,
    pub status: String,
    pub message: Option<String>,
    pub created_at: String,
    pub test: bool,
    pub authorization: String,
    pub currency: String,
    pub location_id: Option<i64>,
    pub user_id: Option<i64>,
    pub parent_id: Option<i64>,
    pub device_id: Option<i64>,
    pub receipt: serde_json::Value,  // Assuming receipt can have dynamic content
    pub error_code: Option<String>,
    pub source_name: String,
}
