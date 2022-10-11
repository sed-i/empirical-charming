use std::env;
use strum_macros::Display;


#[derive(Debug)]
struct GenericContext {
    model: String,
    model_uuid: String,
    unit: String,
    juju_version: String,
}

impl GenericContext {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        Ok(Self {
            model: env::var("JUJU_MODEL_NAME")?,
            model_uuid: env::var("JUJU_MODEL_UUID")?,
            unit: env::var("JUJU_UNIT_NAME")?,
            juju_version: env::var("JUJU_VERSION")?,
        })
    }
}


#[derive(Debug)]
struct InstallContext {}

impl InstallContext {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        let hook_name = env::var("JUJU_HOOK_NAME")?;

        if hook_name == "install".to_string() {
            Ok(Self{})
        } else {
            Err(std::env::VarError::NotPresent)
        }
    }
}


#[derive(Debug)]
struct PebbleReadyContext {
    workload: String,
}

impl PebbleReadyContext {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        let hook_name = env::var("JUJU_HOOK_NAME")?;
        let workload_name = env::var("JUJU_WORKLOAD_NAME")?;

        if hook_name == format!("{}-pebble-ready", workload_name) {
            Ok(Self{workload: workload_name})
        } else {
            Err(std::env::VarError::NotPresent)
        }
    }
}


#[derive(Display, Debug)]
enum HookContext {
    Install(InstallContext, GenericContext),
    PebbleReady(PebbleReadyContext, GenericContext),
    InvalidContext(String),
}

fn parse_hook_context() -> HookContext {

    if let Ok(generic_ctx) = GenericContext::from_env() {
        if let Ok(ctx) = InstallContext::from_env() {
            HookContext::Install(ctx, generic_ctx)
        }
        else if let Ok(ctx) = PebbleReadyContext::from_env() {
            HookContext::PebbleReady(ctx, generic_ctx)
        } else {
            HookContext::InvalidContext("Juju context present but hook context absent".to_string())
        }
    } else {
        HookContext::InvalidContext("Invalid juju context (not even bothering with hook context)".to_string())
    }
}


fn main() {
    println!("Hello Juju: {:?}", parse_hook_context());
}
