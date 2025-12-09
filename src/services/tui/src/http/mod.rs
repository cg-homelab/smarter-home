use once_cell::sync::Lazy;
use reqwest::blocking::{Client, ClientBuilder};

pub static HTTP: Lazy<HttpClient> = Lazy::new(|| {
    let base_url = match std::env::var("backend_url") {
        Ok(val) => val,
        Err(_) => {
            panic!("backend_url not set");
            // tracing::warn!(" not set, using default secret. This is not recommended for production!");
            // "some_default_secret".to_string()
        }
    };

    HttpClient {
        base_url,
        client: HttpClient::create_client(None),
    }
});

#[derive(Debug, Default)]
pub struct HttpClient {
    base_url: String,
    client: Client,
}
impl HttpClient {
    pub fn create_client(token: Option<String>) -> Client {
        let mut builder = ClientBuilder::new().https_only(true);
        if let Some(t) = token {
            builder = builder.default_headers({
                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(
                    reqwest::header::AUTHORIZATION,
                    format!("Bearer {}", t).parse().unwrap(),
                );
                headers
            });
        }
        builder.build().unwrap()
    }

    pub fn set_token(&mut self, token: String) {
        self.client = HttpClient::create_client(Some(token));
    }

    pub fn get(&self, endpoint: &str) -> reqwest::blocking::Response {
        let url = format!("{}/{}", self.base_url, endpoint);
        self.client.get(&url).send().unwrap()
    }

    pub fn post<T>(&self, endpoint: &str, body: &T) -> reqwest::blocking::Response
    where
        T: serde::Serialize,
    {
        let url = format!("{}/{}", self.base_url, endpoint);
        self.client.post(&url).json(body).send().unwrap()
    }

    pub fn put<T>(&self, endpoint: &str, body: &T) -> reqwest::blocking::Response
    where
        T: serde::Serialize,
    {
        let url = format!("{}/{}", self.base_url, endpoint);
        self.client.put(&url).json(body).send().unwrap()
    }

    pub fn patch<T>(&self, endpoint: &str, body: &T) -> reqwest::blocking::Response
    where
        T: serde::Serialize,
    {
        let url = format!("{}/{}", self.base_url, endpoint);
        self.client.patch(&url).json(body).send().unwrap()
    }

    pub fn delete(&self, endpoint: &str) -> reqwest::blocking::Response {
        let url = format!("{}/{}", self.base_url, endpoint);
        self.client.delete(&url).send().unwrap()
    }
}
