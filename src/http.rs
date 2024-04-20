//! A module for the http client that makes requests to the Revolt REST API.

use {
    crate::models::User, ahash::AHashMap, governor::{
        clock::DefaultClock,
        state::{InMemoryState, NotKeyed},
        Jitter, Quota, RateLimiter,
    }, lazy_static::lazy_static, reqwest::{
        header::{HeaderMap, HeaderValue},
        Client,
    }, serde::{de::DeserializeOwned, ser::Serialize}, std::{num::NonZeroU32, sync::Arc, time::Duration}
};

use crate::error::APIError;
use anyhow::Result;

/// The url of the [Revolt REST API](https://developers.revolt.chat/api/).
const DELTA_API: &str = "https://api.revolt.chat";

lazy_static!{
    /// Rate limits for certain API paths
    static ref RATE_LIMITS: AHashMap<&'static str, u8> = AHashMap::from_iter([
        ("bots", 10),
        ("channels", 15),
        ("servers", 5),
        ("auth", 3),
        ("swagger", 100),
    ]);
}

/// A struct to execute requests to the [Revolt REST API](https://developers.revolt.chat/api/).
#[derive(Debug, Clone)]
pub struct HttpClient {
    client: Client,
    jitter: Jitter,
    rate_limiter: Arc<RateLimiter<NotKeyed, InMemoryState, DefaultClock>>,
    pub(crate) bot: User,
}

impl HttpClient {
    pub(crate) async fn new(token: &str) -> Self {
        let mut bot_token = HeaderValue::from_str(token).unwrap();
        bot_token.set_sensitive(true);
        let mut headers = HeaderMap::new();
        headers.insert("X-BOT-TOKEN", HeaderValue::from_str(token).unwrap());

        let client = Client::builder()
            .default_headers(headers)
            .https_only(true)
            .build()
            .unwrap();

        let quota = Quota::per_second(NonZeroU32::new(2).unwrap());
        let rate_limiter = RateLimiter::direct(quota);
        let jitter = Jitter::up_to(Duration::from_millis(100));
        let bot: User = client
            .get(format!("{}/users/@me", DELTA_API))
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap(); // use expect with message (?)

        Self {
            client,
            jitter,
            bot,
            rate_limiter: Arc::new(rate_limiter),
        }
    }

    fn make_url(path: impl AsRef<str>) -> String {
        format!("{}/{}", DELTA_API, path.as_ref())
    }

    /// Make a `GET` request to the API and convert the response body to JSON.
    #[allow(clippy::missing_panics_doc)]
    pub async fn get<T: DeserializeOwned>(&self, path: impl AsRef<str>) -> Result<T> {
        self.obtain_lease(path.as_ref(), "GET").await;

        let response = self.client.get(Self::make_url(path)).send().await?;

        if !response.status().is_success() {
            let api_error: APIError = response.json().await.unwrap();
            return Err(anyhow::Error::new(api_error));
        };

        let body = response.json().await?;

        Ok(body)
    }

    /// Make a `POST` request to the API with a json body and convert the response body to JSON.
    #[allow(clippy::missing_panics_doc)]
    pub async fn post<T: DeserializeOwned, U: Serialize>(
        &self,
        path: impl AsRef<str>,
        body: U,
    ) -> Result<T> {
        self.obtain_lease(path.as_ref(), "POST").await;

        let response = self
            .client
            .post(Self::make_url(path))
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let api_error: APIError = response.json().await.unwrap();
            return Err(anyhow::Error::new(api_error));
        };

        let body = response.json().await?;

        Ok(body)
    }

    /// Make a `PUT` request to the API with a JSON body.
    #[allow(clippy::missing_panics_doc)]
    pub async fn put<T: Serialize>(&self, path: impl AsRef<str>, body: T) -> Result<()> {
        self.obtain_lease(path.as_ref(), "PUT").await;

        let response = self
            .client
            .put(Self::make_url(path))
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let api_error: APIError = response.json().await.unwrap();
            return Err(anyhow::Error::new(api_error));
        };

        Ok(())
    }

    /// Make a `PATCH` request to the API with a JSON body.
    #[allow(clippy::missing_panics_doc)]
    pub async fn patch<T: Serialize>(&self, path: impl AsRef<str>, body: T) -> Result<()> {
        self.obtain_lease(path.as_ref(), "PATCH").await;

        let response = self
            .client
            .patch(Self::make_url(path))
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let api_error: APIError = response.json().await.unwrap();
            return Err(anyhow::Error::new(api_error));
        };

        Ok(())
    }

    /// Make a `DELETE` request to the API.
    #[allow(clippy::missing_panics_doc)]
    pub async fn delete(&self, path: impl AsRef<str>) -> Result<()> {
        self.obtain_lease(path.as_ref(), "DELETE").await;

        println!("Sending DELETE request");
        let response = self.client.delete(Self::make_url(path)).send().await?;

        if !response.status().is_success() {
            let api_error: APIError = response.json().await.unwrap();
            return Err(anyhow::Error::new(api_error));
        };

        Ok(())
    }

    async fn obtain_lease(
        &self,
        path: &str,
        method: &str,
    ) {
        match method {
            "POST" => {
                if path.starts_with("/channels") && path.ends_with("/messages"){
                    let quota = Quota::per_second(NonZeroU32::new(1).unwrap());
                    RateLimiter::direct(quota).until_ready_with_jitter(self.jitter).await;
                } else {
                    self.rate_limiter.until_ready_with_jitter(self.jitter).await;
                }
            },
            "DELETE" => {
                if path.starts_with("/auth") {
                    let quota = Quota::per_second(NonZeroU32::new(25).unwrap());
                    RateLimiter::direct(quota).until_ready_with_jitter(self.jitter).await;
                } else {
                    self.rate_limiter.until_ready_with_jitter(self.jitter).await;
                }
            },

            _ => if let Some(rate_limit) = RATE_LIMITS.get(path.split('/').next().unwrap()){
                let quota = Quota::per_second(NonZeroU32::new((rate_limit / 10) as u32).unwrap());
                RateLimiter::direct(quota).until_ready_with_jitter(self.jitter).await;
            } else {
                self.rate_limiter.until_ready_with_jitter(self.jitter).await;
            }
        }
    }
}
