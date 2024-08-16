use vault_rust::client::Vault;
use vault_rust::api::query::Query;
use vault_rust::api::auth_method::approle::EndpointRole;
use serde::Deserialize;

#[test]
fn approle() {
    let client = Vault::new("http://127.0.0.1:8200".into(), "<token>".into()).unwrap();
    let approle_endpoint = EndpointRole::new("approle".into(), "my-role".into());

    #[derive(Deserialize, Debug)]
    struct Approle {
        data: ApproleData, 
    }

    #[derive(Deserialize, Debug)]
    struct ApproleData {
        token_ttl: u32,
    }

    let _: Approle = approle_endpoint.query(&client).expect("we want this to work");
}
