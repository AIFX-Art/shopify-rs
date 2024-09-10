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
    orders: Vec<Order>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderCount {
    count: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTaxLine {
    title: String,
    price: f64,
    rate: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateLineItem {
    title: String,
    price: f64,
    grams: Option<i32>,
    quantity: Option<i32>,
    tax_lines: Vec<CreateTaxLine>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    kind: String,
    status: String,
    amount: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateOrderInput {
    line_items: Vec<CreateLineItem>,
    transactions: Vec<Transaction>,
    total_tax: f64,
    currency: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Money {
    amount: String,
    currency_code: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Address {
    address1: Option<String>,
    address2: Option<String>,
    city: Option<String>,
    company: Option<String>,
    country: String,
    first_name: Option<String>,
    last_name: Option<String>,
    phone: Option<String>,
    province: Option<String>,
    zip: Option<String>,
    name: Option<String>,
    province_code: Option<String>,
    country_code: Option<String>,
    latitude: Option<f64>,
    longitude: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientDetails {
    accept_language: String,
    browser_height: Option<i32>,
    browser_ip: String,
    browser_width: Option<i32>,
    session_hash: Option<String>,
    user_agent: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Company {
    id: u64,
    location_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderSet {
    shop_money: Money,
    presentment_money: Money,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscountApplication {
    #[serde(rename = "type")]
    type_: String,
    title: Option<String>,
    description: String,
    value: String,
    value_type: String,
    allocation_method: String,
    target_selection: String,
    target_type: String,
    code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscountCode {
    code: String,
    amount: String,
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Fulfillment {
    created_at: String,
    id: u64,
    order_id: u64,
    status: String,
    tracking_company: Option<String>,
    tracking_number: Option<String>,
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttributedStaff {
    id: String,
    quantity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Properties {
    name: Option<String>,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaxLine {
    title: String,
    price: String,
    price_set: OrderSet,
    channel_liable: Option<bool>,
    rate: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscountAllocation {
    amount: String,
    discount_application_index: i32,
    amount_set: OrderSet,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OriginLocation {
    id: u64,
    country_code: String,
    province_code: String,
    name: Option<String>,
    address1: Option<String>,
    address2: Option<String>,
    city: String,
    zip: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DutyTaxLine {
    title: String,
    price: String,
    rate: f32,
    price_set: Option<OrderSet>,
    channel_liable: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Duty {
    id: String,
    harmonized_system_code: String,
    country_code_of_origin: String,
    shop_money: Money,
    presentment_money: Money,
    tax_lines: Vec<DutyTaxLine>,
    admin_graphql_api_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LineItem {
    attributed_staffs: Option<Vec<AttributedStaff>>,
    fulfillable_quantity: i32,
    fulfillment_service: String,
    fulfillment_status: Option<String>,
    grams: i32,
    id: u64,
    price: String,
    product_id: u64,
    quantity: i32,
    current_quantity: i32,
    requires_shipping: bool,
    sku: String,
    title: String,
    variant_id: u64,
    variant_title: Option<String>,
    vendor: String,
    name: Option<String>,
    gift_card: bool,
    price_set: OrderSet,
    properties: Option<Vec<Properties>>,
    taxable: bool,
    tax_lines: Option<Vec<TaxLine>>,
    total_discount: String,
    total_discount_set: OrderSet,
    discount_allocations: Vec<DiscountAllocation>,
    origin_location: Option<OriginLocation>,
    duties: Option<Vec<Duty>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteAttribute {
    name: Option<String>,
    value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentSchedule {
    amount: f32,
    currency: String,
    issued_at: String,
    due_at: String,
    completed_at: Option<String>,
    expected_payment_method: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentTerm {
    amount: f32,
    currency: String,
    payment_terms_name: Option<String>,
    payment_terms_type: String,
    due_in_days: i32,
    payment_schedules: Vec<PaymentSchedule>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShippingLine {
    code: String,
    price: String,
    price_set: OrderSet,
    discounted_price: String,
    discounted_price_set: OrderSet,
    source: String,
    title: String,
    tax_lines: Vec<TaxLine>,
    carrier_identifier: Option<String>,
    requested_fulfillment_service_id: Option<String>,
    is_removed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Refund {
    id: u64,
    order_id: u64,
    created_at: String,
    note: Option<String>,
    user_id: Option<String>,
    processed_at: String,
    refund_line_items: Vec<String>,
    transactions: Vec<String>,
    order_adjustments: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Customer {
    id: u64,
    email: Option<String>,
    accepts_marketing: Option<bool>,
    created_at: String,
    updated_at: String,
    first_name: Option<String>,
    last_name: Option<String>,
    state: String,
    note: Option<String>,
    verified_email: bool,
    multipass_identifier: Option<String>,
    tax_exempt: bool,
    tax_exemptions: Option<Vec<String>>,
    phone: Option<String>,
    tags: String,
    currency: String,
    addresses: Option<Vec<Address>>,
    admin_graphql_api_id: String,
    default_address: Option<Address>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    id: u64,
    admin_graphql_api_id: String,
    app_id: Option<u64>,
    browser_ip: Option<String>,
    buyer_accepts_marketing: bool,
    cancel_reason: Option<String>,
    cancelled_at: Option<String>,
    cart_token: Option<String>,
    checkout_id: Option<u64>,
    checkout_token: Option<String>,
    client_details: Option<ClientDetails>,
    closed_at: Option<String>,
    confirmation_number: Option<String>,
    confirmed: bool,
    created_at: Option<String>,
    currency: String,
    current_total_additional_fees_set: Option<OrderSet>,
    current_total_discounts: Option<String>,
    current_total_discounts_set: Option<OrderSet>,
    current_total_duties_set: Option<OrderSet>,
    current_total_price: Option<String>,
    current_total_price_set: Option<OrderSet>,
    current_subtotal_price: Option<String>,
    current_subtotal_price_set: Option<OrderSet>,
    current_total_tax: Option<String>,
    current_total_tax_set: Option<OrderSet>,
    customer: Option<Customer>,
    customer_locale: Option<String>,
    discount_applications: Option<Vec<DiscountApplication>>,
    discount_codes: Option<Vec<DiscountCode>>,
    email: Option<String>,
    estimated_taxes: Option<bool>,
    financial_status: Option<String>,
    fulfillments: Option<Vec<Fulfillment>>,
    fulfillment_status: Option<String>,
    gateway: Option<String>,
    landing_site: Option<String>,
    line_items: Option<Vec<LineItem>>,
    location_id: Option<u64>,
    merchant_of_record_app_id: Option<u64>,
    name: Option<String>,
    note: Option<String>,
    note_attributes: Option<Vec<NoteAttribute>>,
    number: Option<i32>,
    order_number: Option<i32>,
    original_total_additional_fees_set: Option<OrderSet>,
    original_total_duties_set: Option<OrderSet>,
    payment_terms: Option<PaymentTerm>,
    payment_gateway_names: Option<Vec<String>>,
    phone: Option<String>,
    po_number: Option<String>,
    presentment_currency: Option<String>,
    processed_at: Option<String>,
    referring_site: Option<String>,
    refunds: Option<Vec<Refund>>,
    shipping_address: Option<Address>,
    shipping_lines: Option<Vec<ShippingLine>>,
    source_name: Option<String>,
    source_identifier: Option<String>,
    source_url: Option<String>,
    subtotal_price: Option<String>,
    subtotal_price_set: Option<OrderSet>,
    tags: Option<String>,
    tax_lines: Option<Vec<TaxLine>>,
    taxes_included: Option<bool>,
    test: Option<bool>,
    token: Option<String>,
    total_discounts: Option<String>,
    total_discounts_set: Option<OrderSet>,
    total_line_items_price: Option<String>,
    total_line_items_price_set: Option<OrderSet>,
    total_outstanding: Option<String>,
    total_price: Option<String>,
    total_price_set: Option<OrderSet>,
    total_shipping_price_set: Option<OrderSet>,
    total_tax: Option<String>,
    total_tax_set: Option<OrderSet>,
    total_tip_received: Option<String>,
    total_weight: Option<i32>,
    updated_at: Option<String>,
    user_id: Option<u64>,
    order_status_url: Option<String>,
}