use chrono::{Datelike, Utc};
use serde_json::Result;
use serde::{Deserialize, Serialize};
use fake::{Dummy, Fake, Faker, faker};
use fake::faker::chrono::en::DateTimeBetween;
use fake::faker::internet::en::FreeEmail;
use fake::uuid::UUIDv5;

#[derive(Serialize,Deserialize,Clone)]
pub struct Order {
    pub id: String,
    customer_email: String,
    order_date: String,
    status: String,
    total_amount: f64
}

#[derive(Serialize,Deserialize,Clone)]
pub struct OrdersPage {
    pub orders: Vec<Order>,
    count: i32,
    next: String,
}

impl Order {
    pub fn fake() -> Order {
        let uuid = fake::uuid::UUIDv5{};
        let id = uuid.fake();
        let customer_email = FreeEmail().fake();
        let to = Utc::now();
        let from = Utc::now().with_year(1996).unwrap();
        let order_date = DateTimeBetween(from, to).fake();
        let status = String::from("open");
        let total_amount = (10.0..3000.0).fake::<f64>();
        Order { id, customer_email, order_date, status, total_amount}
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(self)
    }
}

impl OrdersPage {
    pub fn fake() -> OrdersPage {
        let count = (10..50).fake();
        let orders: Vec<Order> = (0..count).map(|_| Order::fake()).collect();
        let next = (8..20).fake::<String>();
        OrdersPage { count, orders, next }
    }

    pub fn to_json(&self) -> Result<String> {
        serde_json::to_string(self)
    }
}