use crate::api::query::Query;
use crate::api::client::Client;
use serde::de::DeserializeOwned;
use std::borrow::Cow;

pub trait Endpoint {
    fn method(&self) -> http::Method; 
    fn endpoint(&self) -> Cow<'static, str>;
}

impl<E, T, C> Query<T, C> for E
where
    E: Endpoint,
    T: DeserializeOwned,
    C: Client,
{
    fn query(&self, client: &C) -> Result<T, ureq::Error> {
        let resp: T = client.call(&self.method(), &self.endpoint())?.into_json()?;
        Ok(resp)
    }
}
