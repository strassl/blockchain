#[macro_use]
extern crate log;
extern crate env_logger;

mod chain;

fn main() {
    env_logger::init().unwrap();

    let mut ch = chain::init();
    chain::push(&mut ch, 123);
    chain::print(&ch);
    chain::push(&mut ch, 456);
    chain::print(&ch);
    chain::push(&mut ch, 789);
    chain::print(&ch);
    verify_chain(&ch);

    println!("Changing block 1");
    ch.blocks[1].data = 7;
    verify_chain(&ch);
}

fn verify_chain(chain: &chain::Chain) {
    match chain::verify(chain) {
        Ok(_) => println!("Chain ok"),
        Err(err) => println!("Chain error: {}", err),
    }
}
