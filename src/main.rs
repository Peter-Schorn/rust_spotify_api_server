#![feature(proc_macro_hygiene, decl_macro)]

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

#[macro_use] extern crate rocket;

use serde::{Serialize, Deserialize};
use rocket::request::Form;

fn main() {
    // https://bit.ly/3CcqYym
    rocket::ignite()
        .mount(
            "/",
            routes![
                index,
                client_credentials_flow_retrieve_tokens
            ]
        )
        .launch();
}

// #[get(Endpoint::Root)]
#[get("/")]
fn index() -> &'static str {
    "success"
}

// #[post(Endpoint::CLIENT_CREDENTIALS_FLOW_RETRIEVE_TOKENS)]
#[post(
    "/client-credentials-flow/retrieve-tokens",
    data="<credentials>"
)]
fn client_credentials_flow_retrieve_tokens(
    credentials: Form<ClientCredentialsTokensRequest>
) -> String {

    println!(
        "in endpoint: {}",
        Endpoint::CLIENT_CREDENTIALS_FLOW_RETRIEVE_TOKENS
    );

    // let body = ClientCredentialsTokensRequest::new();
    let body = format!("{:?}", credentials);

    return body
}

enum Endpoint { }
impl Endpoint {

    const ROOT: &str = "/";

    const CLIENT_CREDENTIALS_FLOW_RETRIEVE_TOKENS: &str = "/client-credentials-flow/retrieve-tokens";

    const AUTHORIZATION_CODE_FLOW_RETRIEVE_TOKENS: &str = "authorization-code-flow/retrieve-tokens";
    const AUTHORIZATION_CODE_FLOW_REFRESH_TOKENS: &str = "authorization-code-flow/refresh-tokens";

    const AUTHORIZATION_CODE_FLOW_PKCE_RETRIEVE_TOKENS: &str = "authorization-code-flow-pkce/retrieve-tokens";
    const AUTHORIZATION_CODE_FLOW_PKCE_REFRESH_TOKENS: &str = "authorization-code-flow-pkce/refresh-tokens";

}

#[derive(FromForm, Serialize, Deserialize, Debug)]
pub struct ClientCredentialsTokensRequest {
    /// Should always be "client_credentials"
    pub grant_type: String,
}

impl ClientCredentialsTokensRequest {

    pub fn new() -> Self {
        Self {
            grant_type: "client_credentials".to_string()
        }
    }

}
