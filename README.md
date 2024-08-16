# vault_rust

Small, currently read-only, client for Hashicorp Vault.

# Usage

```
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
```

