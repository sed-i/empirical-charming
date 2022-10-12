use std::env;
use strum_macros::Display;


trait ContextFromEnvs {
    fn from_env() -> Result<Self, std::env::VarError> where Self: Sized;
}

#[derive(Debug)]
pub struct GenericContext {
    model: String,
    model_uuid: String,
    app_name: String,
    unit: u32,
    juju_version: String,
}

impl ContextFromEnvs for GenericContext {
    fn from_env() -> Result<Self, std::env::VarError> {

        // Unit name looks like this: "unit/0".
        // First part is app name, second part is unit number.
        let unit = env::var("JUJU_UNIT_NAME")?;
        let mut split = unit.split("/");
        let app_name = split.next().unwrap().to_string();
        let unit_num = split.next().unwrap().parse::<u32>().unwrap();

        Ok(Self {
            model: env::var("JUJU_MODEL_NAME")?,
            model_uuid: env::var("JUJU_MODEL_UUID")?,
            app_name,
            unit: unit_num,
            juju_version: env::var("JUJU_VERSION")?,
        })
    }
}


#[derive(Debug)]
pub struct InstallContext {}

impl ContextFromEnvs for InstallContext  {
    fn from_env() -> Result<Self, std::env::VarError> {
        let hook_name = env::var("JUJU_HOOK_NAME")?;

        if hook_name == "install".to_string() {
            Ok(Self{})
        } else {
            Err(std::env::VarError::NotPresent)
        }
    }
}


#[derive(Debug)]
pub struct StopContext {}

impl ContextFromEnvs for StopContext {
    fn from_env() -> Result<Self, std::env::VarError> {
        let hook_name = env::var("JUJU_HOOK_NAME")?;

        if hook_name == "stop".to_string() {
            Ok(Self{})
        } else {
            Err(std::env::VarError::NotPresent)
        }
    }
}


#[derive(Debug)]
pub struct RemoveContext {}

impl ContextFromEnvs for RemoveContext {
    fn from_env() -> Result<Self, std::env::VarError> {
        let hook_name = env::var("JUJU_HOOK_NAME")?;

        if hook_name == "remove".to_string() {
            Ok(Self{})
        } else {
            Err(std::env::VarError::NotPresent)
        }
    }
}


#[derive(Debug)]
pub struct PebbleReadyContext {
    workload: String,
}

impl ContextFromEnvs for PebbleReadyContext {
    fn from_env() -> Result<Self, std::env::VarError> {
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
pub enum HookContext {
    Install(InstallContext, GenericContext),
    PebbleReady(PebbleReadyContext, GenericContext),
    Stop(StopContext, GenericContext),
    Remove(RemoveContext, GenericContext),
    InvalidContext(String),
}

pub fn parse_hook_context() -> HookContext {
    if let Ok(generic_ctx) = GenericContext::from_env() {
        if let Ok(ctx) = InstallContext::from_env() {
            HookContext::Install(ctx, generic_ctx)
        } else if let Ok(ctx) = PebbleReadyContext::from_env() {
            HookContext::PebbleReady(ctx, generic_ctx)
        } else if let Ok(ctx) = StopContext::from_env() {
            HookContext::Stop(ctx, generic_ctx)
        } else if let Ok(ctx) = RemoveContext::from_env() {
            HookContext::Remove(ctx, generic_ctx)
        } else {
            HookContext::InvalidContext("Juju context present but hook context absent".to_string())
        }
    } else {
        HookContext::InvalidContext("Invalid juju context (not even bothering with hook context)".to_string())
    }
}
