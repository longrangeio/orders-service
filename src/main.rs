mod data;

use std::collections::HashMap;
use std::os::raw::c_float;
use std::sync::Mutex;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result, http};
use serde::{Deserialize, Serialize};
use crate::data::{Order, OrdersPage};

struct AppState {
    orders: Mutex<HashMap<String,Order>>
}

#[get("/orders")]
async fn orders(data: web::Data<AppState>) -> impl Responder {
    let response = OrdersPage::fake();
    let mut orders = data.orders.lock().unwrap();
    for ord in response.clone().orders {
        println!("INSERTING ORDER...");
        (*orders).insert(ord.id.clone(), ord);
    }
    HttpResponse::Ok()
        .content_type("application/json")
        .body(response.to_json().unwrap())
}

#[get("/orders/{uuid}")]
async fn orderById(data: web::Data<AppState>,id: web::Path<String>) -> impl Responder {
    let other_orders = data.orders.lock().unwrap();
    println!("FOUND {} ORDERS", (*other_orders).len());

    match (*other_orders).get(id.as_str())  {
        Some(found) =>
            HttpResponse::Ok()
                .content_type("application/json")
                .body(found.clone().to_json().unwrap()),
        None =>
            HttpResponse::NotFound()
                .body("Not found")
    }
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server...");
    let state = web::Data::new(AppState{ orders: Mutex::new(HashMap::<String,Order>::new()) });
    // run server
    HttpServer::new(move || {

        App::new()
            .app_data(state.clone())
            .service(orders)
            .service(orderById)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}