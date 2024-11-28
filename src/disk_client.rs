use std::collections::HashMap;

use crate::config::Config;
use base64::prelude::*;
use log;
use rust_i18n::t;
use serde::Deserialize;
use ureq::Error;

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
                        log::info!(
                            "Auth error response body: {}\n description: {}",
                            body.error,
                            body.error_description
                        );

                        let error_codes =
                            HashMap::from([("bad_verification_code", "invalid_code")]);

                        let error_code = error_codes
                            .get(&body.error.as_str())
                            .copied()
                            .unwrap_or(body.error.as_str());
                        let verbose_error = t!(format!("login_form.errors.{error_code}"));

                        Err(format!("{}: {}\n", t!("common.error"), verbose_error))
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
