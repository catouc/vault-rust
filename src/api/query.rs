use crate::api::client::Client;

pub trait Query<T, C>
where
    C: Client,
{
    fn query(&self, client: &C) -> Result<T, ureq::Error>;
}
