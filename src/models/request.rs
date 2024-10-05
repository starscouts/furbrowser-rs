use std::time::Duration;

use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use ureq::Response;

use crate::error::FurbrowserResult;
use crate::models::config::Secrets;
use crate::VERSION;

pub(crate) fn post(uri: &str, user_agent: &str, secrets: &Secrets) -> FurbrowserResult<Response> {
    Ok(ureq::post(uri)
        .timeout(Duration::from_millis(5000))
        .set("User-Agent", &user_agent.replace("VERSION", VERSION))
        .set(
            "Authorization",
            &format!(
                "Basic {}",
                BASE64_STANDARD.encode(format!("{}:{}", secrets.username, secrets.api_key))
            ),
        )
        .call()?)
}

pub(crate) fn delete(uri: &str, user_agent: &str, secrets: &Secrets) -> FurbrowserResult<Response> {
    Ok(ureq::delete(uri)
        .timeout(Duration::from_millis(5000))
        .set("User-Agent", &user_agent.replace("VERSION", VERSION))
        .set(
            "Authorization",
            &format!(
                "Basic {}",
                BASE64_STANDARD.encode(format!("{}:{}", secrets.username, secrets.api_key))
            ),
        )
        .call()?)
}
