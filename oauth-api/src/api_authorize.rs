use crate::db::*;
use r2d2::Pool;
use redis::Client;
use rocket::{response::Redirect, State};
use sqlx::PgPool;
use tracing::debug;

#[allow(unused_variables)]
#[get(
    "/authorize?<client_id>&<response_type>&<scope>&<state>&<code_challenge>&<code_challenge_method>&<redirect_uri>"
)]
pub async fn authorize(
    client_id: String,
    response_type: String,
    scope: String,
    state: String,
    code_challenge: String,
    code_challenge_method: String,
    redirect_uri: String,
    redis_pool: &State<Pool<Client>>,
    pg_pool: &State<PgPool>,
) -> Result<Redirect, String> {
    let _: () = {
        // input validation…
        if code_challenge_method != String::from("S256") {
            Err("code_challenge_method must be S256".to_string())
        } else if response_type != String::from("code") {
            Err("response type must be 'code'".to_string())
        } else {
            Ok(())
        }
    }?;

    debug!("authorizing: {}", client_id);

    let mut connection = redis_pool.get().map_err(|e| e.to_string())?;
    let code_verifier = sha256::digest(code_challenge);
    let auth_code = connection
        .get_auth_code(&client_id, &code_verifier)
        .await
        .map_err(|e| e.to_string())?;

    if let Some(auth_code) = auth_code {
        Ok(Redirect::to(format!(
            "{redirect_uri}?code={auth_code}&state={state}"
        )))
    } else {
        debug!("not authenticated.");
        Ok(Redirect::to(format!("/{}/login", client_id.clone())))
    }
}

#[cfg(all(test, feature = "it"))]
mod integration_tests {
    use super::*;
    use reqwest::{redirect::Policy, StatusCode};

    #[tokio::test]
    async fn authorize() -> anyhow::Result<()> {
        let client = reqwest::ClientBuilder::new()
            .redirect(Policy::none())
            .build()?;
        let uri = uri!(
            "http://localhost:8000",
            authorize(
                "client",
                "code",
                "scope",
                "state",
                "challenge",
                "S256",
                "redirect"
            )
        );
        println!("uri: {}", uri);
        let response = client.get(uri.to_string()).send().await?;
        println!("response: {:?}", response);

        // status is redirect
        assert_eq!(response.status(), StatusCode::SEE_OTHER);
        assert!(response.headers().contains_key("location"));

        // no content in body
        let body = response.text().await?;
        println!("body: {}", body);
        assert!(body.is_empty());
        Ok(())
    }
}
