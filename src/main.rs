#[macro_use]
extern crate log;
extern crate env_logger;

mod chain;

use chain::Blockchain;

fn main() {
    env_logger::init().unwrap();

    let mut ch: chain::Chain = Blockchain::init();
    ch.push(vec![0,0,0,0]);
    println!("{:?}", ch);
    ch.push(vec![0,0,0,1]);
    println!("{:?}", ch);
    ch.push(vec![0,0,0,2]);
    println!("{:?}", ch);
    verify_chain(&ch);

    println!("Changing block 1");
    ch.blocks[1].data = vec![0,0,0,0];
    verify_chain(&ch);
}

fn verify_chain(chain: &chain::Chain) {
    match chain.verify() {
        Ok(_) => println!("Chain ok"),
        Err(err) => println!("Chain error: {}", err),
    }
}
