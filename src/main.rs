mod orderbook;
mod binance_data;

use orderbook::orderbook::process_l3orderbook;
use binance_data::binance_data::main as async_main;

fn main() {
    // Call the function from the orderbook module
    //process_l3orderbook();
    let stream = async_main();
    println!("{:?}", stream);
}
