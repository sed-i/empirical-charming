use std::env;

fn main() {
    let hook_name = env::var("JUJU_HOOK_NAME").unwrap_or("Unknown".to_string());
    println!("Hello Juju: {}", hook_name);
}
