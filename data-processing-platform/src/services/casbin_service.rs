use casbin::{CoreApi, Enforcer, MgmtApi};

pub async fn initialize_enforcer(model_path: &str) -> Result<casbin::Enforcer, crate::errors::AppError> {
    // Create an enforcer with the provided model and policy
    let enforcer = Enforcer::new(model_path, "").await?;
    Ok(enforcer)
}

pub async fn add_policy(
    enforcer: &mut casbin::Enforcer,
    subject: &str,
    resource: &str,
    action: &str,
) -> Result<bool, crate::errors::AppError> {
    let result = enforcer.add_policy(vec![subject.to_string(), resource.to_string(), action.to_string()]).await?;
    Ok(result)
}

pub async fn remove_policy(
    enforcer: &mut casbin::Enforcer,
    subject: &str,
    resource: &str,
    action: &str,
) -> Result<bool, crate::errors::AppError> {
    let result = enforcer.remove_policy(vec![subject.to_string(), resource.to_string(), action.to_string()]).await?;
    Ok(result)
}

pub async fn enforce(
    enforcer: &casbin::Enforcer,
    subject: &str,
    resource: &str,
    action: &str,
) -> Result<bool, crate::errors::AppError> {
    let result = enforcer.enforce((subject, resource, action)).await?;
    Ok(result)
}