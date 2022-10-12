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
        if env::var("JUJU_HOOK_NAME")? == "install".to_string() {
            Ok(Self{})
        } else {
            Err(std::env::VarError::NotPresent)
        }
    }
}


#[derive(Debug)]
pub struct LeaderElectedContext {}

impl ContextFromEnvs for LeaderElectedContext  {
    fn from_env() -> Result<Self, std::env::VarError> {
        if env::var("JUJU_HOOK_NAME")? == "leader-elected".to_string() {
            Ok(Self{})
        } else {
            Err(std::env::VarError::NotPresent)
        }
    }
}


#[derive(Debug)]
pub struct ConfigChangedContext {}

impl ContextFromEnvs for ConfigChangedContext  {
    fn from_env() -> Result<Self, std::env::VarError> {
        if env::var("JUJU_HOOK_NAME")? == "config-changed".to_string() {
            Ok(Self{})
        } else {
            Err(std::env::VarError::NotPresent)
        }
    }
}


#[derive(Debug)]
pub struct StartContext {}

impl ContextFromEnvs for StartContext  {
    fn from_env() -> Result<Self, std::env::VarError> {
        if env::var("JUJU_HOOK_NAME")? == "start".to_string() {
            Ok(Self{})
        } else {
            Err(std::env::VarError::NotPresent)
        }
    }
}


#[derive(Debug)]
pub struct UpdateStatusContext {}

impl ContextFromEnvs for UpdateStatusContext  {
    fn from_env() -> Result<Self, std::env::VarError> {
        if env::var("JUJU_HOOK_NAME")? == "update-status".to_string() {
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
        if env::var("JUJU_HOOK_NAME")? == "stop".to_string() {
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
        if env::var("JUJU_HOOK_NAME")? == "remove".to_string() {
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


#[derive(Debug)]
pub struct RelationCreatedContext {
    relation_name: String,
    // relation_id: u32,
    relation_id: String,
    remote_app: String,
}

impl ContextFromEnvs for RelationCreatedContext {
    fn from_env() -> Result<Self, std::env::VarError> {
        let hook_name = env::var("JUJU_HOOK_NAME")?;
        let relation_name = env::var("JUJU_RELATION")?;
        // let relation_id = env::var("JUJU_RELATION_ID")?.parse::<u32>().unwrap();
        let relation_id = env::var("JUJU_RELATION_ID")?;
        let remote_app = env::var("JUJU_REMOTE_APP")?;

        if hook_name == format!("{}-relation-created", relation_name) {
            Ok(Self{relation_name, relation_id, remote_app})
        } else {
            Err(std::env::VarError::NotPresent)
        }
    }
}


#[derive(Debug)]
pub struct RelationBrokenContext {
    relation_name: String,
    relation_id: u32,
    remote_app: String,
}

impl ContextFromEnvs for RelationBrokenContext {
    fn from_env() -> Result<Self, std::env::VarError> {
        let hook_name = env::var("JUJU_HOOK_NAME")?;
        let relation_name = env::var("JUJU_RELATION")?;
        let relation_id = env::var("JUJU_RELATION_ID")?.parse::<u32>().unwrap();
        let remote_app = env::var("JUJU_REMOTE_APP")?;

        if hook_name == format!("{}-relation-broken", relation_name) {
            Ok(Self{relation_name, relation_id, remote_app})
        } else {
            Err(std::env::VarError::NotPresent)
        }
    }
}


#[derive(Debug)]
pub struct RelationJoinedContext {
    relation_name: String,
    relation_id: u32,
    remote_app: String,
    remote_unit: String,
}

impl ContextFromEnvs for RelationJoinedContext {
    fn from_env() -> Result<Self, std::env::VarError> {
        let hook_name = env::var("JUJU_HOOK_NAME")?;
        let relation_name = env::var("JUJU_RELATION")?;
        let relation_id = env::var("JUJU_RELATION_ID")?.parse::<u32>().unwrap();
        let remote_app = env::var("JUJU_REMOTE_APP")?;
        let remote_unit = env::var("JUJU_REMOTE_UNIT")?;

        if hook_name == format!("{}-relation-joined", relation_name) {
            Ok(Self{relation_name, relation_id, remote_app, remote_unit})
        } else {
            Err(std::env::VarError::NotPresent)
        }
    }
}


#[derive(Debug)]
pub struct RelationDepartedContext {
    relation_name: String,
    relation_id: u32,
    remote_app: String,
    remote_unit: String,
    departing_unit: String,
}

impl ContextFromEnvs for RelationDepartedContext {
    fn from_env() -> Result<Self, std::env::VarError> {
        let hook_name = env::var("JUJU_HOOK_NAME")?;
        let relation_name = env::var("JUJU_RELATION")?;
        let relation_id = env::var("JUJU_RELATION_ID")?.parse::<u32>().unwrap();
        let remote_app = env::var("JUJU_REMOTE_APP")?;
        let remote_unit = env::var("JUJU_REMOTE_UNIT")?;
        let departing_unit = env::var("JUJU_DEPARTING_UNIT")?;

        if hook_name == format!("{}-relation-departed", relation_name) {
            Ok(Self{relation_name, relation_id, remote_app, remote_unit, departing_unit})
        } else {
            Err(std::env::VarError::NotPresent)
        }
    }
}


#[derive(Debug)]
pub struct RelationChangedContext {
    relation_name: String,
    relation_id: u32,
    remote_app: String,
    remote_unit: String,
}

impl ContextFromEnvs for RelationChangedContext {
    fn from_env() -> Result<Self, std::env::VarError> {
        let hook_name = env::var("JUJU_HOOK_NAME")?;
        let relation_name = env::var("JUJU_RELATION")?;
        let relation_id = env::var("JUJU_RELATION_ID")?.parse::<u32>().unwrap();
        let remote_app = env::var("JUJU_REMOTE_APP")?;
        let remote_unit = env::var("JUJU_REMOTE_UNIT")?;

        if hook_name == format!("{}-relation-changed", relation_name) {
            Ok(Self{relation_name, relation_id, remote_app, remote_unit})
        } else {
            Err(std::env::VarError::NotPresent)
        }
    }
}


#[derive(Display, Debug)]
pub enum HookContext {
    Install(InstallContext, GenericContext),
    Start(StartContext, GenericContext),
    LeaderElected(LeaderElectedContext, GenericContext),
    ConfigChanged(ConfigChangedContext, GenericContext),
    PebbleReady(PebbleReadyContext, GenericContext),
    UpdateStatus(UpdateStatusContext, GenericContext),
    Stop(StopContext, GenericContext),
    Remove(RemoveContext, GenericContext),

    RelationCreated(RelationCreatedContext, GenericContext),
    RelationBroken(RelationBrokenContext, GenericContext),
    RelationJoined(RelationJoinedContext, GenericContext),
    RelationDeparted(RelationDepartedContext, GenericContext),
    RelationChanged(RelationChangedContext, GenericContext),

    InvalidContext(String),
}

pub fn parse_hook_context() -> HookContext {
    if let Ok(generic_ctx) = GenericContext::from_env() {
        if let Ok(ctx) = InstallContext::from_env() {
            HookContext::Install(ctx, generic_ctx)
        } else if let Ok(ctx) = StartContext::from_env() {
            HookContext::Start(ctx, generic_ctx)
        } else if let Ok(ctx) = LeaderElectedContext::from_env() {
            HookContext::LeaderElected(ctx, generic_ctx)
        } else if let Ok(ctx) = ConfigChangedContext::from_env() {
            HookContext::ConfigChanged(ctx, generic_ctx)
        } else if let Ok(ctx) = PebbleReadyContext::from_env() {
            HookContext::PebbleReady(ctx, generic_ctx)
        } else if let Ok(ctx) = UpdateStatusContext::from_env() {
            HookContext::UpdateStatus(ctx, generic_ctx)
        } else if let Ok(ctx) = StopContext::from_env() {
            HookContext::Stop(ctx, generic_ctx)
        } else if let Ok(ctx) = RemoveContext::from_env() {
            HookContext::Remove(ctx, generic_ctx)
        } else if let Ok(ctx) = RelationCreatedContext::from_env() {
            HookContext::RelationCreated(ctx, generic_ctx)
        } else if let Ok(ctx) = RelationBrokenContext::from_env() {
            HookContext::RelationBroken(ctx, generic_ctx)
        } else if let Ok(ctx) = RelationJoinedContext::from_env() {
            HookContext::RelationJoined(ctx, generic_ctx)
        } else if let Ok(ctx) = RelationDepartedContext::from_env() {
            HookContext::RelationDeparted(ctx, generic_ctx)
        } else if let Ok(ctx) = RelationChangedContext::from_env() {
            HookContext::RelationChanged(ctx, generic_ctx)
        } else {
            HookContext::InvalidContext("Juju context present but hook context absent".to_string())
        }
    } else {
        HookContext::InvalidContext("Invalid juju context (not even bothering with hook context)".to_string())
    }
}
