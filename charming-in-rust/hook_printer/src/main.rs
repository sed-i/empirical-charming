// use jujuops::calc;

mod jujuenv;

use jujuenv::parse_hook_context;



fn main() {
    println!("Hello Juju: {:?}", parse_hook_context());
}
