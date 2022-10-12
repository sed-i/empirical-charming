// use jujuops::calc;

mod jujuenv;

use jujuenv::parse_hook_context;


fn on_install() {
    println!("Custom on_install hook");
}

fn main() {
    let ctx = parse_hook_context();
    match ctx {
        jujuenv::HookContext::Install(_, _) => on_install(),
        _ => {},
    }
    println!("Hello Juju: {:?}", ctx);
}
