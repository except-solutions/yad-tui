use base64::{alphabet::URL_SAFE, prelude::*};
use log;
use serde::Deserialize;

use crate::config::Config;
use ureq::{Error, Error::Status, Response};

#[derive(Deserialize)]
enum AuthResponse {
    SuccessAuth,
    AuthError,
    UnknwonError,
}

#[derive(Deserialize)]
pub struct SuccessAuth {
    pub token_type: String,
    pub access_token: String,
    pub expires_in: i32,
    pub refresh_token: String,
}

#[derive(Deserialize)]
struct AuthError {
    error: String,
    error_description: String,
}

#[derive(Debug, Clone)]
pub struct DiskClient {
    pub api_url: String,
    pub oauth_url: String,
    pub client_id: String,
    pub client_secret: String,
    pub token: Option<String>,
}

impl DiskClient {
    pub fn from_app_conf(conf: &Config) -> Self {
        DiskClient {
            api_url: conf.api.api_url.clone(),
            oauth_url: conf.api.oauth_url.clone(),
            client_id: conf.api.client_id.clone(),
            client_secret: conf.api.client_secret.clone(),
            token: None,
        }
    }

    pub fn auth(&self, code: String) -> Result<SuccessAuth, String> {
        let url = &format!("{}/token", self.oauth_url);
        let token = BASE64_STANDARD.encode(format!("{}:{}", &self.client_id, &self.client_secret));
        log::info!("Try fetch api token, url: {}, code: {}", url, code);
        let response = ureq::post(url)
            .set("Authorization", &format!("Basic {}", token))
            .send_form(&[("grant_type", "authorization_code"), ("code", &code)])
            .map(|r| r.into_json::<SuccessAuth>());

        match response {
            Ok(response) => Ok(response.unwrap()),
            Err(Error::Status(code, response)) => {
                log::info!("Auth error response with code {}", code);

                match response.into_json::<AuthError>() {
                    Ok(body) => {
                        log::info!("Auth error response body: {}", body.error);
                        Err(body.error)
                    }
                    Err(response_error) => {
                        log::error!("Unexpected response error: {}", response_error);
                        Err(format!("Unexpected error: {}", response_error))
                    }
                }
            }
            Err(other_error) => {
                log::error!("{}", other_error);
                Err("Unknown error".to_string())
            }
        }
    }
}
