#[macro_use]
extern crate serde_derive;

mod blockchain_info;
mod blockchain_status;
mod blockchain_transaction;
mod blockchain_address;

use blockchain_status::BlockchainStatus;

fn main() {
    let blockchain_status:BlockchainStatus = blockchain_info::blockchain_status_request();

    println!("\n\nQuerying {} - chain: {}\n\n", &blockchain_status.blockbook.coin, &blockchain_status.backend.chain);
}
