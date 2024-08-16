pub trait Client {
    fn call(&self, method: &http::Method, endpoint: &str) -> Result<ureq::Response, ureq::Error>;
}
