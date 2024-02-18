use std::io;
use csv::{ReaderBuilder};
use std::collections::{HashMap, BTreeMap};
use std::io::Write;


pub mod orderbook {

    use std::io;
    use csv::{ReaderBuilder};
    use std::collections::{HashMap, BTreeMap};

    // A struct to represent an order.
    #[derive(Debug, Clone)] 
    struct Order {
        order_id : i32, // order identifier  : type = i32 
        side : String,  // side of the order either Bid or Ask : type = String (would of  made this an enum but wasn't sure if it had to be a String)
        price : i32,    // price the order was placed : type = i64 (as prices )
        volume : i32     // volume of the order : type = i64
    }

    #[derive(Debug)]
    struct L3OrderBook {
        aks_orders_by_price: BTreeMap<(i32, i32), Order>,  // Ask, Orders sorted by (price, order_id)
        ask_orders_by_id: HashMap<i32, Order>,             // Ask, Orders mapped by order ID

        bid_orders_by_price: BTreeMap<(i32, i32), Order>,  // Bid, Orders sorted by (price, order_id)
        bid_orders_by_id: HashMap<i32, Order>,             // Bid, Orders mapped by order ID

    }

    impl L3OrderBook {
        fn add_order(&mut self, order: Order) {
            // Checking to see whether the order is bid or ask.
            if order.side == "Bid" {
                // Inserts order_id into the BTreeMap based on (price, order_id)
                self.bid_orders_by_price.insert((order.price, order.order_id), order.clone());
                // Inserts order into the HashMap 
                self.bid_orders_by_id.insert(order.order_id, order);
            }  
            else if order.side == "Ask" {
                // Inserts order_id into the BTreeMap based on (price, order_id)
                self.bid_orders_by_price.insert((order.price, order.order_id), order.clone());
                // Inserts order into the HashMap 
                self.bid_orders_by_id.insert(order.order_id, order);
            }
        }

        fn remove_order_by_id(&mut self, order_id_to_remove: i32) {
            if let Some(order) = self.ask_orders_by_id.remove(&order_id_to_remove) {
                // If the order is found, remove it from the BTreeMap
                self.aks_orders_by_price.remove(&(order.price, order_id_to_remove));
            }
            if let Some(order) = self.bid_orders_by_id.remove(&order_id_to_remove) {
                // If the order is found, remove it from the BTreeMap
                self.bid_orders_by_price.remove(&(order.price, order_id_to_remove));
            }
        }

        fn print_snapshot(&self){
            // prints out the asks
            let asks_size : usize = self.aks_orders_by_price.len();
            let mut rev_vec =  Vec::new(); // vec to take the rev of the asks as they need to be outputted from worst to best
            for (t, order) in self.aks_orders_by_price.iter().take(std::cmp::min(asks_size, 10)) {
                rev_vec.push(order);
            }
            rev_vec.reverse(); // reverse the vec
            if rev_vec.len() > 0{
                for order in rev_vec{
                    let side = &order.side;
                    let price = order.price;
                    let volume = order.volume;
                    let order_id = order.order_id;
                    println!("{}, {}, {}, {}", *side, price, order_id, volume);
                } // prints out the asks as intructed
            }

            // prints out the bids
            let bid_size : usize = self.bid_orders_by_price.len();
            // Ensure bid_size is at least 10 before iterating
            if bid_size >= 10 {
                for (t, order) in self.bid_orders_by_price.iter().rev().skip(bid_size - 10) {
                    let side = &order.side;
                    let price = order.price;
                    let volume = order.volume;
                    let order_id = order.order_id;
                    println!("{}, {}, {}, {}", *side, price, order_id, volume);
                }   // maybe I did this weird haha I am tired rn
            } else {
                for (t, order) in self.bid_orders_by_price.iter().rev() {
                    let side = &order.side;
                    let price = order.price;
                    let volume = order.volume;
                    let order_id = order.order_id;
                    println!("{}, {}, {}, {}", *side, price, order_id, volume);
                } // prints out the bids as intructed
            }
        }
    }

    pub fn process_l3orderbook(){
        // Creating a l3 orderbook object
        let mut aksbtmap: BTreeMap<(i32, i32), Order> = BTreeMap::new();
        let mut bidbtmap: BTreeMap<(i32, i32), Order> = BTreeMap::new();
        let mut askhashmap: HashMap<i32, Order> = HashMap::new();
        let mut bidhashmap: HashMap<i32, Order> = HashMap::new();
        
        let mut l3_orderbook = L3OrderBook{
            aks_orders_by_price : aksbtmap,
            ask_orders_by_id : askhashmap,
            bid_orders_by_price : bidbtmap,
            bid_orders_by_id : bidhashmap,
        };
    
        // Parsing the CSV rows from stdin
        let mut rdr = ReaderBuilder::new().from_reader(io::stdin());
        // variable to keep track of the event_count
        let mut event_count = 0;
    
        // iterating through the rows of stdin
        for row in rdr.records(){
            let record = row.unwrap(); 
            let event = &record[0];
    
            match event {
                // case of Add event
                "Add" => {
                    // parsing the row 
                    let order_id: i32 = record[1].parse().unwrap();
                    let side = record[2].to_string();
                    let price: i32 = record[3].parse().unwrap();
                    let volume: i32 = record[4].parse().unwrap();
                    // constructing an order object
                    let order = Order {order_id, side, price, volume};
                    // adding the order to the order book 
                    l3_orderbook.add_order(order);
                }
                // case of Remove event
                "Remove" => {
                    // parse order id and remove from book
                    let order_id : i32 = record[1].parse().unwrap();
                    l3_orderbook.remove_order_by_id(order_id);
                }
                _ => {} // no other matching case 
            }
            event_count += 1;
            println!("{}", event_count);
            l3_orderbook.print_snapshot();
        }
    }

    // TESTS
    #[cfg(test)]
    mod tests {
        use super::*;
        use std::io::Write;

        #[test]
        fn test_add_order() {
            let mut order_book = L3OrderBook {
                aks_orders_by_price: BTreeMap::new(),
                ask_orders_by_id: HashMap::new(),
                bid_orders_by_price: BTreeMap::new(),
                bid_orders_by_id: HashMap::new(),
            };

            let order = Order {
                order_id: 1,
                side: "Bid".to_string(),
                price: 100,
                volume: 10,
            };

            order_book.add_order(order.clone());

            // Assert that the order exists in the order book after adding
            assert!(order_book.bid_orders_by_id.contains_key(&order.order_id));
        }

        #[test]
        fn test_remove_order() {
            let mut order_book = L3OrderBook {
                aks_orders_by_price: BTreeMap::new(),
                ask_orders_by_id: HashMap::new(),
                bid_orders_by_price: BTreeMap::new(),
                bid_orders_by_id: HashMap::new(),
            };

            let order = Order {
                order_id: 1,
                side: "Bid".to_string(),
                price: 100,
                volume: 10,
            };

            order_book.add_order(order.clone());
            order_book.remove_order_by_id(order.order_id);

            // Assert that the order no longer exists in the order book after removal
            assert!(!order_book.bid_orders_by_id.contains_key(&order.order_id));
        }
    }
    
}



