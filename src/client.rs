use crate::api::client::Client;
use url::Url;

pub struct Vault {
    http_client: ureq::Agent,
    base_url: Url,
    token: String,
}

impl Vault {
    pub fn new(base_url_string: String, token: String) -> Result<Vault, url::ParseError> {
        let base_url = url::Url::parse(&base_url_string)?;
        let http_client = ureq::Agent::new();
        Ok(Vault{http_client, base_url, token})
    }
}

impl Client for Vault {
    fn call(&self, method: &http::Method, endpoint: &str) -> Result<ureq::Response, ureq::Error> {
        let request_url = &self.base_url.join(endpoint)?;
        self.http_client.request(method.as_str(), request_url.as_str())
            .set("X-Vault-Token", &self.token)
            .call()
    }
}
