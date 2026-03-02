use crate::error::ApiError;
use crate::pagination::PaginatedResponse;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue, USER_AGENT};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::time::Duration;
use tracing::{debug, warn};
use uuid::Uuid;

const DEFAULT_BASE_URL: &str = "https://api.todoist.com/api/v1";
const DEFAULT_TIMEOUT_SECS: u64 = 30;
const DEFAULT_MAX_RETRIES: u32 = 3;

#[derive(Debug, Clone)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub base_delay: Duration,
    pub max_delay: Duration,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: DEFAULT_MAX_RETRIES,
            base_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(60),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TodoistClient {
    http: reqwest::Client,
    base_url: String,
    token: String,
    retry_config: RetryConfig,
}

impl TodoistClient {
    pub fn new(token: impl Into<String>) -> Result<Self, ApiError> {
        Self::with_base_url(token, DEFAULT_BASE_URL)
    }

    pub fn with_base_url(
        token: impl Into<String>,
        base_url: impl Into<String>,
    ) -> Result<Self, ApiError> {
        let token = token.into();
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {token}")).map_err(|_| ApiError::Auth {
                message: "Invalid token format".into(),
            })?,
        );
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static(concat!("td-cli/", env!("CARGO_PKG_VERSION"))),
        );

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .build()?;

        Ok(Self {
            http,
            base_url: base_url.into(),
            token,
            retry_config: RetryConfig::default(),
        })
    }

    pub fn with_retry_config(mut self, config: RetryConfig) -> Self {
        self.retry_config = config;
        self
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base_url, path)
    }

    async fn execute(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<reqwest::Response, ApiError> {
        let mut retries = 0;

        loop {
            let req = request.try_clone().ok_or_else(|| {
                ApiError::Network(
                    reqwest::Client::new()
                        .get("http://invalid")
                        .build()
                        .unwrap_err(),
                )
            })?;

            match req.send().await {
                Ok(response) => {
                    let status = response.status();

                    if status.is_success() {
                        return Ok(response);
                    }

                    if status.as_u16() == 401 || status.as_u16() == 403 {
                        return Err(ApiError::Auth {
                            message:
                                "Token invalid or expired. Run `td auth login` to re-authenticate."
                                    .into(),
                        });
                    }

                    if status.as_u16() == 404 {
                        return Err(ApiError::NotFound {
                            resource: "requested resource".into(),
                        });
                    }

                    if status.as_u16() == 429 || status.is_server_error() {
                        if retries >= self.retry_config.max_retries {
                            if status.as_u16() == 429 {
                                return Err(ApiError::RateLimited {
                                    retry_after_secs: 0,
                                });
                            }
                            let body = response.text().await.unwrap_or_default();
                            return Err(ApiError::Server {
                                status: status.as_u16(),
                                message: body,
                            });
                        }

                        let delay = self.calculate_backoff(retries, &response);
                        warn!(
                            status = status.as_u16(),
                            retry = retries + 1,
                            delay_ms = delay.as_millis() as u64,
                            "Retrying request"
                        );
                        tokio::time::sleep(delay).await;
                        retries += 1;
                        continue;
                    }

                    if status.as_u16() == 422 {
                        let body = response.text().await.unwrap_or_default();
                        return Err(ApiError::Validation { message: body });
                    }

                    let body = response.text().await.unwrap_or_default();
                    return Err(ApiError::Server {
                        status: status.as_u16(),
                        message: body,
                    });
                }
                Err(e) if e.is_timeout() => {
                    if retries >= self.retry_config.max_retries {
                        return Err(ApiError::Timeout);
                    }
                    retries += 1;
                    let delay = self.retry_config.base_delay * 2u32.pow(retries - 1);
                    tokio::time::sleep(delay).await;
                }
                Err(e) => return Err(ApiError::Network(e)),
            }
        }
    }

    fn calculate_backoff(&self, retry: u32, response: &reqwest::Response) -> Duration {
        // Check Retry-After header first
        if let Some(retry_after) = response.headers().get("retry-after")
            && let Ok(secs) = retry_after.to_str().unwrap_or("0").parse::<u64>()
        {
            return Duration::from_secs(secs);
        }

        // Exponential backoff with jitter
        let base = self.retry_config.base_delay.as_millis() as u64;
        let exp_delay = base * 2u64.pow(retry);
        let jitter = exp_delay / 4; // 25% jitter
        let delay = exp_delay + (Uuid::new_v4().as_u128() as u64 % jitter.max(1));
        Duration::from_millis(delay).min(self.retry_config.max_delay)
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, ApiError> {
        debug!(path, "GET");
        let response = self.execute(self.http.get(self.url(path))).await?;
        Ok(response.json().await?)
    }

    pub async fn get_with_query<T: DeserializeOwned, Q: Serialize + ?Sized>(
        &self,
        path: &str,
        query: &Q,
    ) -> Result<T, ApiError> {
        debug!(path, "GET with query");
        let response = self
            .execute(self.http.get(self.url(path)).query(query))
            .await?;
        Ok(response.json().await?)
    }

    pub async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, ApiError> {
        debug!(path, "POST");
        let request_id = Uuid::new_v4().to_string();
        let response = self
            .execute(
                self.http
                    .post(self.url(path))
                    .header("X-Request-Id", &request_id)
                    .json(body),
            )
            .await?;
        Ok(response.json().await?)
    }

    pub async fn post_empty_response<B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<(), ApiError> {
        debug!(path, "POST (no response body)");
        let request_id = Uuid::new_v4().to_string();
        self.execute(
            self.http
                .post(self.url(path))
                .header("X-Request-Id", &request_id)
                .json(body),
        )
        .await?;
        Ok(())
    }

    pub async fn post_no_body(&self, path: &str) -> Result<(), ApiError> {
        debug!(path, "POST (no body)");
        let request_id = Uuid::new_v4().to_string();
        self.execute(
            self.http
                .post(self.url(path))
                .header("X-Request-Id", &request_id),
        )
        .await?;
        Ok(())
    }

    pub async fn delete(&self, path: &str) -> Result<(), ApiError> {
        debug!(path, "DELETE");
        let request_id = Uuid::new_v4().to_string();
        self.execute(
            self.http
                .delete(self.url(path))
                .header("X-Request-Id", &request_id),
        )
        .await?;
        Ok(())
    }

    pub async fn get_paginated<T: DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<PaginatedResponse<T>, ApiError> {
        self.get(path).await
    }

    pub async fn get_all_pages<T: DeserializeOwned>(&self, path: &str) -> Result<Vec<T>, ApiError> {
        let mut all_results = Vec::new();
        let mut cursor: Option<String> = None;

        loop {
            let page: PaginatedResponse<T> = match &cursor {
                Some(c) => {
                    let query = [("cursor", c.as_str())];
                    self.get_with_query(path, &query).await?
                }
                None => self.get(path).await?,
            };
            all_results.extend(page.results);

            match page.next_cursor {
                Some(next) => cursor = Some(next),
                None => break,
            }
        }

        Ok(all_results)
    }
}
