use serde::{Serialize,Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum OrderStatus {
    open,     // Show only open orders.
    closed,   // Show only closed orders.
    cancelled,// Show only canceled orders.
    any,      // Show orders of any status, including archived orders.
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderList {
    pub orders: Vec<Order>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderCount {
    pub count: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaxLine {
    pub title: String,
    pub price: f64,
    pub rate: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLineItem {
    pub title: String,
    pub price: f64,
    pub grams: Option<i32>,
    pub quantity: Option<i32>,
    pub tax_lines: Vec<CreateTaxLine>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub kind: String,
    pub status: String,
    pub amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderInput {
    pub line_items: Vec<CreateLineItem>,
    pub transactions: Vec<Transaction>,
    pub total_tax: f64,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Money {
    pub amount: String,
    pub currency_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub city: Option<String>,
    pub company: Option<String>,
    pub country: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub province: Option<String>,
    pub zip: Option<String>,
    pub name: Option<String>,
    pub province_code: Option<String>,
    pub country_code: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientDetails {
    pub accept_language: String,
    pub browser_height: Option<i32>,
    pub browser_ip: String,
    pub browser_width: Option<i32>,
    pub session_hash: Option<String>,
    pub user_agent: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Company {
    pub id: u64,
    pub location_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderSet {
    pub shop_money: Money,
    pub presentment_money: Money,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscountApplication {
    #[serde(rename = "type")]
    pub type_: String,
    pub title: Option<String>,
    pub description: String,
    pub value: String,
    pub value_type: String,
    pub allocation_method: String,
    pub target_selection: String,
    pub target_type: String,
    pub code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscountCode {
    pub code: String,
    pub amount: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fulfillment {
    pub created_at: String,
    pub id: u64,
    pub order_id: u64,
    pub status: String,
    pub tracking_company: Option<String>,
    pub tracking_number: Option<String>,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributedStaff {
    pub id: String,
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Properties {
    pub name: Option<String>,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxLine {
    pub title: String,
    pub price: String,
    pub price_set: OrderSet,
    pub channel_liable: Option<bool>,
    pub rate: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscountAllocation {
    pub amount: String,
    pub discount_application_index: i32,
    pub amount_set: OrderSet,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginLocation {
    pub id: u64,
    pub country_code: String,
    pub province_code: String,
    pub name: Option<String>,
    pub address1: Option<String>,
    pub address2: Option<String>,
    pub city: String,
    pub zip: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DutyTaxLine {
    pub title: String,
    pub price: String,
    pub rate: f32,
    pub price_set: Option<OrderSet>,
    pub channel_liable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Duty {
    pub id: String,
    pub harmonized_system_code: String,
    pub country_code_of_origin: String,
    pub shop_money: Money,
    pub presentment_money: Money,
    pub tax_lines: Vec<DutyTaxLine>,
    pub admin_graphql_api_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineItem {
    pub attributed_staffs: Option<Vec<AttributedStaff>>,
    pub fulfillable_quantity: i32,
    pub fulfillment_service: String,
    pub fulfillment_status: Option<String>,
    pub grams: i32,
    pub id: u64,
    pub price: String,
    pub product_id: u64,
    pub quantity: i32,
    pub current_quantity: i32,
    pub requires_shipping: bool,
    pub sku: String,
    pub title: String,
    pub variant_id: u64,
    pub variant_title: Option<String>,
    pub vendor: String,
    pub name: Option<String>,
    pub gift_card: bool,
    pub price_set: OrderSet,
    pub properties: Option<Vec<Properties>>,
    pub taxable: bool,
    pub tax_lines: Option<Vec<TaxLine>>,
    pub total_discount: String,
    pub total_discount_set: OrderSet,
    pub discount_allocations: Vec<DiscountAllocation>,
    pub origin_location: Option<OriginLocation>,
    pub duties: Option<Vec<Duty>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteAttribute {
    pub name: Option<String>,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentSchedule {
    pub amount: f32,
    pub currency: String,
    pub issued_at: String,
    pub due_at: String,
    pub completed_at: Option<String>,
    pub expected_payment_method: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTerm {
    pub amount: f32,
    pub currency: String,
    pub payment_terms_name: Option<String>,
    pub payment_terms_type: String,
    pub due_in_days: i32,
    pub payment_schedules: Vec<PaymentSchedule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShippingLine {
    pub code: String,
    pub price: String,
    pub price_set: OrderSet,
    pub discounted_price: String,
    pub discounted_price_set: OrderSet,
    pub source: String,
    pub title: String,
    pub tax_lines: Vec<TaxLine>,
    pub carrier_identifier: Option<String>,
    pub requested_fulfillment_service_id: Option<String>,
    pub is_removed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Refund {
    pub id: u64,
    pub order_id: u64,
    pub created_at: String,
    pub note: Option<String>,
    pub user_id: Option<String>,
    pub processed_at: String,
    pub refund_line_items: Vec<String>,
    pub transactions: Vec<String>,
    pub order_adjustments: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Customer {
    pub id: u64,
    pub email: Option<String>,
    pub accepts_marketing: Option<bool>,
    pub created_at: String,
    pub updated_at: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub state: String,
    pub note: Option<String>,
    pub verified_email: bool,
    pub multipass_identifier: Option<String>,
    pub tax_exempt: bool,
    pub tax_exemptions: Option<Vec<String>>,
    pub phone: Option<String>,
    pub tags: String,
    pub currency: String,
    pub addresses: Option<Vec<Address>>,
    pub admin_graphql_api_id: String,
    pub default_address: Option<Address>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub id: u64,
    pub admin_graphql_api_id: String,
    pub app_id: Option<u64>,
    pub browser_ip: Option<String>,
    pub buyer_accepts_marketing: bool,
    pub cancel_reason: Option<String>,
    pub cancelled_at: Option<String>,
    pub cart_token: Option<String>,
    pub checkout_id: Option<u64>,
    pub checkout_token: Option<String>,
    pub client_details: Option<ClientDetails>,
    pub closed_at: Option<String>,
    pub confirmation_number: Option<String>,
    pub confirmed: bool,
    pub created_at: Option<String>,
    pub currency: String,
    pub current_total_additional_fees_set: Option<OrderSet>,
    pub current_total_discounts: Option<String>,
    pub current_total_discounts_set: Option<OrderSet>,
    pub current_total_duties_set: Option<OrderSet>,
    pub current_total_price: Option<String>,
    pub current_total_price_set: Option<OrderSet>,
    pub current_subtotal_price: Option<String>,
    pub current_subtotal_price_set: Option<OrderSet>,
    pub current_total_tax: Option<String>,
    pub current_total_tax_set: Option<OrderSet>,
    pub customer: Option<Customer>,
    pub customer_locale: Option<String>,
    pub discount_applications: Option<Vec<DiscountApplication>>,
    pub discount_codes: Option<Vec<DiscountCode>>,
    pub email: Option<String>,
    pub estimated_taxes: Option<bool>,
    pub financial_status: Option<String>,
    pub fulfillments: Option<Vec<Fulfillment>>,
    pub fulfillment_status: Option<String>,
    pub gateway: Option<String>,
    pub landing_site: Option<String>,
    pub line_items: Option<Vec<LineItem>>,
    pub location_id: Option<u64>,
    pub merchant_of_record_app_id: Option<u64>,
    pub name: Option<String>,
    pub note: Option<String>,
    pub note_attributes: Option<Vec<NoteAttribute>>,
    pub number: Option<i32>,
    pub order_number: Option<i32>,
    pub original_total_additional_fees_set: Option<OrderSet>,
    pub original_total_duties_set: Option<OrderSet>,
    pub payment_terms: Option<PaymentTerm>,
    pub payment_gateway_names: Option<Vec<String>>,
    pub phone: Option<String>,
    pub po_number: Option<String>,
    pub presentment_currency: Option<String>,
    pub processed_at: Option<String>,
    pub referring_site: Option<String>,
    pub refunds: Option<Vec<Refund>>,
    pub shipping_address: Option<Address>,
    pub shipping_lines: Option<Vec<ShippingLine>>,
    pub source_name: Option<String>,
    pub source_identifier: Option<String>,
    pub source_url: Option<String>,
    pub subtotal_price: Option<String>,
    pub subtotal_price_set: Option<OrderSet>,
    pub tags: Option<String>,
    pub tax_lines: Option<Vec<TaxLine>>,
    pub taxes_included: Option<bool>,
    pub test: Option<bool>,
    pub token: Option<String>,
    pub total_discounts: Option<String>,
    pub total_discounts_set: Option<OrderSet>,
    pub total_line_items_price: Option<String>,
    pub total_line_items_price_set: Option<OrderSet>,
    pub total_outstanding: Option<String>,
    pub total_price: Option<String>,
    pub total_price_set: Option<OrderSet>,
    pub total_shipping_price_set: Option<OrderSet>,
    pub total_tax: Option<String>,
    pub total_tax_set: Option<OrderSet>,
    pub total_tip_received: Option<String>,
    pub total_weight: Option<i32>,
    pub updated_at: Option<String>,
    pub user_id: Option<u64>,
    pub order_status_url: Option<String>,
}