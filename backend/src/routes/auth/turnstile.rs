use serde::Deserialize;

const VERIFY_URL: &str = "https://challenges.cloudflare.com/turnstile/v0/siteverify";

#[derive(Deserialize)]
struct VerifyResponse {
    success: bool,
}

pub async fn verify(secret: &str, token: &str) -> bool {
    let response = match reqwest::Client::new()
        .post(VERIFY_URL)
        .form(&[("secret", secret), ("response", token)])
        .send()
        .await
    {
        Ok(response) => response,
        Err(_) => return false,
    };

    response
        .json::<VerifyResponse>()
        .await
        .map(|body| body.success)
        .unwrap_or(false)
}
