use std::env;
use strum_macros::Display;


#[derive(Display, Debug)]
enum HookContext {
    // #[strum(to_string = "context")]
    Context(String),

    // #[strum(to_string = "invalid_context")]
    InvalidContext,
}

fn parse_hook_context() -> HookContext {
    if let Ok(hook_name) = env::var("JUJU_HOOK_NAME") {
        HookContext::Context(hook_name)
    } else {
        HookContext::InvalidContext
    }
}

fn main() {
    println!("Hello Juju: {}", parse_hook_context());
}
