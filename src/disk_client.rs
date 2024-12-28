use std::{collections::HashMap, fmt};

use crate::{config::Config, meta_db::Meta};
use base64::prelude::*;
use log;
use rust_i18n::t;
use serde::{de::DeserializeOwned, Deserialize};
use ureq::Error as HTTPError;

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

#[derive(Deserialize)]
pub struct User {
    pub display_name: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct DiskMetaResponse {
    pub used_space: u64,
    pub total_space: u64,
    pub user: User,
}

#[derive(Deserialize)]
pub struct DirItem {
    pub name: String,
    pub resource_id: String,
    pub path: String,
    pub r#type: String,
    pub created: String,
    pub modified: String,
    pub revision: u64,
}

#[derive(Deserialize)]
pub struct DirItems {
    pub limit: u32,
    pub offset: u32,
    pub total: u32,
    pub items: Vec<DirItem>,
}

#[derive(Deserialize)]
pub struct ItemResponse {
    pub name: String,
    pub resource_id: String,
    pub path: String,
    pub r#type: String,
    pub created: String,
    pub modified: String,
    pub revision: u64,
    pub _embedded: DirItems,
}

#[derive(Debug)]
pub enum DiskError {
    UnknownServer(String),
    Unauthorized(String),
    Forbidden(String),
    InvalidResponseBody(String),
    EmptyToken,
}

impl DiskError {
    pub fn unknown_default() -> Self {
        Self::UnknownServer(t!("disk.errors.unknown").to_string())
    }

    pub fn unauthorized_default() -> Self {
        Self::Unauthorized(t!("disk.errors.unauthorized").to_string())
    }

    pub fn forbidden_default() -> Self {
        Self::Forbidden(t!("disk.errors.forbidden").to_string())
    }

    pub fn invalid_response_body_default() -> Self {
        Self::InvalidResponseBody(t!("disk.errors.invalid_body").to_string())
    }
}

impl fmt::Display for DiskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DiskError::UnknownServer(msg) => write!(f, "{}", msg),
            DiskError::Unauthorized(msg) => write!(f, "{}", msg),
            DiskError::Forbidden(msg) => write!(f, "{}", msg),
            DiskError::InvalidResponseBody(msg) => write!(f, "{}", msg),
            _ => write!(f, "Unknwon disk error"),
        }
    }
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
    pub fn from_app_conf(conf: &Config, meta: &Meta) -> Self {
        DiskClient {
            api_url: conf.api.api_url.clone(),
            oauth_url: conf.api.oauth_url.clone(),
            client_id: conf.api.client_id.clone(),
            client_secret: conf.api.client_secret.clone(),
            token: meta.api_token.clone(),
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
            Err(HTTPError::Status(code, response)) => {
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

    pub fn disk_meta(&self) -> Result<DiskMetaResponse, DiskError> {
        let token = self.token.clone().ok_or(DiskError::EmptyToken)?;

        let response = ureq::get(&self.api_url)
            .set("Authorization", &format!("OAuth {token}", token = token))
            .call();

        self.match_response::<DiskMetaResponse>(response)
    }

    pub fn item(
        &self,
        path: &str,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> Result<ItemResponse, DiskError> {
        let token = self.token.clone().ok_or(DiskError::EmptyToken)?;

        let offset_q = offset.map(|o| format!("&offset={}", o)).unwrap_or("".to_string());
        let limit_q = limit.map(|l| format!("&limit={}", l)).unwrap_or("".to_string());
        let optional_query_param = offset_q + &limit_q;

        let response = ureq::get(
            format!(
                "{}/resources?path={}{}",
                &self.api_url, path, optional_query_param
            )
            .as_str(),
        )
        .set("Authorization", &format!("OAuth {token}", token = token))
        .call();

        self.match_response::<ItemResponse>(response)
    }

    fn match_response<T: DeserializeOwned>(
        &self,
        response: Result<ureq::Response, HTTPError>,
    ) -> Result<T, DiskError> {
        match response {
            Ok(response_body) => Ok(response_body.into_json::<T>().unwrap()),
            Err(HTTPError::Status(401, response_err)) => {
                log::error!(
                    "Unauthorized response exception: {:?}",
                    response_err.into_string()
                );
                Err(DiskError::unauthorized_default())
            }
            Err(HTTPError::Status(403, response_err)) => {
                log::error!("Forbidden error: {:?}", response_err.into_string());
                Err(DiskError::forbidden_default())
            }
            Err(HTTPError::Status(code, response_err)) => {
                log::error!(
                    "Unknown API error, code: {} {:?}",
                    code,
                    response_err.into_string()
                );
                Err(DiskError::unknown_default())
            }
            Err(response_error) => {
                log::error!("Unexpected response error: {:?}", response_error);
                Err(DiskError::unknown_default())
            }
        }
    }
}
