use actix_web::{post, get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::sync::{Arc, Mutex};

struct Account {
    owner_id: u32,
    balance: f64,
}

struct Exchange {
    fee_percentage: f64,
    buy_orders: Arc<Mutex<Vec<Order>>>,
    sell_orders: Arc<Mutex<Vec<Order>>>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
struct Order {
    is_buy: bool,
    price: u32,
    quantity: u32,
    priority: i32,
    owner_id: u32,
}

trait Matcher {
    fn match_orders(&mut self, buy_orders: Vec<Order>, sell_orders: Vec<Order>);
}

trait Filler {
    fn fill_order(&mut self, buy_order: &Order, sell_order: &Order);
}

struct OrderProcessor {
    exchange: Arc<Exchange>,
    accounts: Arc<Mutex<Vec<Account>>>,
}

impl OrderProcessor {
    fn add_order(&mut self, order: Order) -> Order {
        let list = if order.is_buy {
            &self.exchange.buy_orders
        } else {
            &self.exchange.sell_orders
        };
        let mut orders = list.lock().unwrap();
        orders.push(order);
        println!("added order {}", order);
        return order;
    }
}

impl Clone for OrderProcessor {
    fn clone(&self) -> Self {
        OrderProcessor {
            exchange: self.exchange.clone(),
            accounts: self.accounts.clone(),
        }
    }
}

impl<'a> Matcher for OrderProcessor {
    fn match_orders(&mut self, mut buy_orders: Vec<Order>, mut sell_orders: Vec<Order>) {
        buy_orders.sort_by(|a, b| b.priority.cmp(&a.priority));
        sell_orders.sort_by(|a, b| b.priority.cmp(&a.priority));

        for buy_order in buy_orders.iter() {
            for sell_order in sell_orders.iter() {
                if buy_order.is_buy && !sell_order.is_buy && (buy_order.price >= sell_order.price) {
                    self.fill_order(buy_order, sell_order);
                    // TODO remove orders from the books if they're completed
                }
            }
        }
    }
}

impl Filler for OrderProcessor {
    fn fill_order(&mut self, buy_order: &Order, sell_order: &Order) {
        let total_cost = buy_order.price as f64 * buy_order.quantity as f64;

        let mut accounts = self.accounts.lock().unwrap();

        let buyer_index = accounts
            .iter()
            .position(|acc| acc.owner_id == buy_order.owner_id)
            .expect("Buyer account not found");
        let seller_index = accounts
            .iter()
            .position(|acc| acc.owner_id == sell_order.owner_id)
            .expect("Seller account not found");

        assert_ne!(
            buyer_index, seller_index,
            "Buyer and seller cannot be the same account"
        );

        let buyer_account = &mut accounts[buyer_index];
        buyer_account.balance -= total_cost;
        println!("Buyer's new balance: {}", buyer_account.balance);

        let seller_account = &mut accounts[seller_index];
        let fee = total_cost * self.exchange.fee_percentage / 100.0;
        seller_account.balance += total_cost - fee;
        println!("Seller's new balance: {}", seller_account.balance);
    }
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Order {{ is_buy: {}, price: {}, quantity: {}, priority: {}, owner_id: {} }}",
            self.is_buy, self.price, self.quantity, self.priority, self.owner_id
        )
    }
}

#[post("/orders")]
async fn new_order(
    order: web::Json<Order>,
    processor: web::Data<Arc<Mutex<OrderProcessor>>>,
) -> impl Responder {
    let mut proc = processor.lock().unwrap();
    let o = proc.add_order(order.into_inner());
    HttpResponse::Ok().json(o)
}

#[get("/orders")]
async fn get_orders(
    processor: web::Data<Arc<Mutex<OrderProcessor>>>,
) -> impl Responder {
    #[derive(Serialize)]
    struct OrdersResponse {
        buy_orders: Vec<Order>,
        sell_orders: Vec<Order>,
    }
    
    let proc = processor.lock() .unwrap();
    let buy_orders = proc.exchange.buy_orders.lock().unwrap().clone();
    let sell_orders = proc.exchange.sell_orders.lock().unwrap().clone();
    
    let response = OrdersResponse {
        buy_orders,
        sell_orders
    };

    return HttpResponse::Ok().json(response);
}

async fn run_matcher(processor: Arc<Mutex<OrderProcessor>>) {
    loop {
        println!("running matcher with fee {}", processor.lock().unwrap().exchange.fee_percentage);
        {
            let mut proc = processor.lock().unwrap();
            let buys = proc.exchange.buy_orders.lock().unwrap().clone();
            let sells = proc.exchange.sell_orders.lock().unwrap().clone();
            proc.match_orders(buys, sells)
        }

        // Sleep for a short duration to yield control
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let accounts = vec![
        Account {
            owner_id: 1,
            balance: 1000.0,
        },
        Account {
            owner_id: 2,
            balance: 500.0,
        },
    ];
    
    let exchange = Exchange {
        fee_percentage: 2.0,
        buy_orders: Arc::new(Mutex::new(Vec::new())),
        sell_orders: Arc::new(Mutex::new(Vec::new())),
    };

    let processor = OrderProcessor {
        accounts: Arc::new(Mutex::new(accounts)),
        exchange: Arc::new(exchange),
    };

    tokio::spawn(run_matcher(Arc::new(Mutex::new(processor.clone()))));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(Arc::new(Mutex::new(processor.clone()))))
            .service(new_order)
            .service(get_orders)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
