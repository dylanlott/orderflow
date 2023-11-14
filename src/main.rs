struct Account {
    owner_id: u32,
    balance: f64,
}

struct Exchange {
    fee_percentage: f64,
}

struct Order {
    is_buy: bool,
    price: u32,
    quantity: u32,
    priority: i32,
    owner_id: u32,
}

trait OrderMatcher {
    fn match_orders(&mut self, buy_orders: Vec<Order>, sell_orders: Vec<Order>);
}

trait OrderFiller {
    fn fill_order(&mut self, buy_order: &Order, sell_order: &Order);
}

struct OrderProcessor<'a> {
    exchange: &'a Exchange,
    accounts: &'a mut Vec<Account>,
}

impl<'a> OrderMatcher for OrderProcessor<'a> {
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

impl<'a> OrderFiller for OrderProcessor<'a> {
    fn fill_order(&mut self, buy_order: &Order, sell_order: &Order) {
        let total_cost = buy_order.price as f64 * buy_order.quantity as f64;

        let buyer_index = self.accounts.iter().position(|acc| acc.owner_id == buy_order.owner_id)
                           .expect("Buyer account not found");
        let seller_index = self.accounts.iter().position(|acc| acc.owner_id == sell_order.owner_id)
                            .expect("Seller account not found");

        assert_ne!(buyer_index, seller_index, "Buyer and seller cannot be the same account");

        {
            let buyer_account = &mut self.accounts[buyer_index];
            buyer_account.balance -= total_cost;
            println!("Buyer's new balance: {}", buyer_account.balance);
        }
        
        {
            let seller_account = &mut self.accounts[seller_index];
            let fee = total_cost * self.exchange.fee_percentage / 100.0;
            seller_account.balance += total_cost - fee;
            println!("Seller's new balance: {}", seller_account.balance);
        }
    }
}

fn main() {
    let mut accounts = vec![
        Account { owner_id: 1, balance: 1000.0 },
        Account { owner_id: 2, balance: 500.0 },
    ];
    let exchange = Exchange { fee_percentage: 2.0 };
    
    let mut processor = OrderProcessor {exchange: &exchange, accounts: &mut accounts };

    let buy_orders = vec![
        Order { is_buy: true, price: 100, quantity: 5, priority: 1, owner_id: 1 },
    ];
    let sell_orders = vec![
        Order { is_buy: false, price: 100, quantity: 5, priority: 2, owner_id: 2 },
    ];
    
    processor.match_orders(buy_orders, sell_orders);
}
