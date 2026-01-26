use base64::prelude::*;
use log;
use rust_i18n::t;
use serde::{de::DeserializeOwned, Deserialize};
use std::io::Read;
use std::{collections::HashMap, fmt};
use ureq::{Error as HTTPError, Request};

use crate::{config::Config, meta_db::Meta};

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

#[derive(Deserialize, Clone, Hash, Eq, PartialEq, Debug)]
pub struct DirItem {
    pub name: String,
    pub resource_id: String,
    pub path: String,
    pub r#type: String,
    pub created: String,
    pub modified: String,
    pub revision: u64,
}

impl DirItem {
    pub fn is_dir(&self) -> bool {
        &self.r#type == "dir"
    }
}

#[derive(Deserialize, Clone)]
pub struct DirItems {
    pub limit: u32,
    pub offset: u32,
    pub total: u32,
    pub items: Vec<DirItem>,
}

#[derive(Deserialize, Clone)]
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

impl ItemResponse {
    pub fn is_dir(&self) -> bool {
        &self.r#type == "dir"
    }

    pub fn as_tuple(self) -> (DirItem, DirItems) {
        (
            DirItem {
                name: self.name,
                resource_id: self.resource_id,
                path: self.path,
                r#type: self.r#type,
                created: self.created,
                modified: self.modified,
                revision: self.revision,
            },
            self._embedded,
        )
    }
}

#[derive(Debug)]
pub enum DiskError {
    UnknownServer(String),
    Unauthorized(String),
    Forbidden(String),
    InvalidResponseBody(String),
    EmptyToken,
}

#[derive(Deserialize)]
pub struct DownloadHref {
    pub href: String,
    pub method: String,
    pub templated: bool,
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

pub trait DiskClientT: Sync + Send + Clone + 'static {
    fn auth(&self, code: String) -> Result<SuccessAuth, String>;

    fn disk_meta(&self) -> Result<DiskMetaResponse, DiskError>;

    fn item(
        &self,
        path: &str,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> Result<ItemResponse, DiskError>;

    fn prepare_request(&self, request_f: impl Fn() -> Request) -> Result<Request, DiskError>;

    fn set_api_token(&self, request: Request) -> Result<Request, DiskError>;

    fn file_reader(&self, cloud_path: String) -> Result<Box<dyn Read + Send + Sync>, DiskError>;
    fn update_token(&self, new_token: String) -> Self;
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

    fn match_response<T>(
        &self,
        response: Result<ureq::Response, HTTPError>,
        response_handler: impl Fn(ureq::Response) -> T,
    ) -> Result<T, DiskError> {
        match response {
            Ok(response_body) => Ok(response_handler(response_body)),
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

impl DiskClientT for DiskClient {
    fn update_token(&self, new_token: String) -> Self {
        DiskClient {
            token: Some(new_token),
            ..self.clone()
        }
    }

    fn auth(&self, code: String) -> Result<SuccessAuth, String> {
        // TODO: rewrite with new api match_respose, set_token etc
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

    fn disk_meta(&self) -> Result<DiskMetaResponse, DiskError> {
        let response = self.prepare_request(|| ureq::get(&self.api_url))?.call();
        self.match_response::<DiskMetaResponse>(response, into_json)
    }

    fn item(
        &self,
        path: &str,
        offset: Option<u32>,
        limit: Option<u32>,
    ) -> Result<ItemResponse, DiskError> {
        let offset_q = offset
            .map(|o| format!("&offset={}", o))
            .unwrap_or("".to_string());
        let limit_q = limit
            .map(|l| format!("&limit={}", l))
            .unwrap_or("".to_string());
        let optional_query_param = offset_q + &limit_q;
        let response = self
            .prepare_request(|| {
                ureq::get(
                    format!(
                        "{}/resources?path={}{}",
                        &self.api_url, path, optional_query_param
                    )
                    .as_str(),
                )
            })?
            .call();
        self.match_response::<ItemResponse>(response, into_json)
    }

    fn prepare_request(&self, request_f: impl Fn() -> Request) -> Result<Request, DiskError> {
        self.set_api_token(request_f())
    }

    fn set_api_token(&self, request: Request) -> Result<Request, DiskError> {
        let token = self.token.clone().ok_or(DiskError::EmptyToken)?;
        Ok(request.set("Authorization", &format!("OAuth {token}", token = token)))
    }

    fn file_reader(&self, cloud_path: String) -> Result<Box<dyn Read + Send + Sync>, DiskError> {
        let fetch_download_link_response = self
            .prepare_request(&|| {
                ureq::get(
                    format!("{}/resources/download?path={}", self.api_url, cloud_path).as_str(),
                )
            })?
            .call();

        let download_link =
            self.match_response::<DownloadHref>(fetch_download_link_response, into_json)?;
        let remote_file_response = self
            .prepare_request(|| ureq::get(&download_link.href))?
            .call();

        let remote_file_reader = self.match_response(remote_file_response, |r| r.into_reader())?;

        Ok(remote_file_reader)
    }
}

fn into_json<T: DeserializeOwned>(response: ureq::Response) -> T {
    response.into_json::<T>().unwrap()
}
