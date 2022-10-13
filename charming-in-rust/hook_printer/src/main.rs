// use jujuops::calc;

mod jujuenv;

use jujuenv::parse_hook_context;

#[derive(Default)]
struct Charm {}

impl Charm {
    fn on_install(&self) {
        println!("Custom on_install hook");
    }

    fn on_replica_created(&self) {
        println!("Custom replicas-created hook");
    }

    fn dispatch(&self) {
        let ctx = parse_hook_context();
        println!("Hello Juju: {:?}", ctx);

        match ctx {
            jujuenv::HookContext::Install(_, _) => self.on_install(),
            jujuenv::HookContext::RelationCreated(ctx, _) => {
                if ctx.relation_name == "relicas" {
                    self.on_replica_created();
                }
            }
            _ => {},
        }
    }
}

fn main() {
    let charm = Charm::default();
    charm.dispatch();
}
