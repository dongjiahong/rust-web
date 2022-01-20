use crate::common::CONTEXT;
use crate::middleware::jwt;
use poem::{Endpoint, Request, Response};

const TOKEN_HEADER: &str = "Token";

pub async fn auth_token<E: Endpoint>(next: E, mut req: Request) -> Response {
    if let Some(value) = req
        .headers()
        .get(TOKEN_HEADER)
        .and_then(|value| value.to_str().ok())
    {
        let token = value.to_string();
        match jwt::JWTToken::verify(&CONTEXT.app_config.server.token_secret, &token) {
            Ok(data) => {} // TODO query database
            Err(err) => {} // TODO
        }
    }
    next.call(req).await.into_response()
}
