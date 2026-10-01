use anyhow::{Context, Result};
use serde::Serialize;
use spin_sdk::{
    http::{body::IncomingBodyExt, IntoResponse, Json, Request},
    http_service, variables,
};

#[derive(Serialize)]
struct ResponseModel {
    authentication: String,
}
#[http_service]
async fn handle_openbao(req: Request) -> Result<impl IntoResponse> {
    let body_bytes = req.into_body().bytes().await?;
    let actual = std::str::from_utf8(&body_bytes).unwrap();
    let expected = variables::get("token")
        .await
        .context("could not get variable")?;
    let outcome = match actual == expected {
        true => "accepted",
        false => "denied",
    };
    Ok(Json(ResponseModel {
        authentication: outcome.into(),
    }))
}
