/*
 * Example
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use hyper;

pub struct Configuration<C: hyper::client::connect::Connect>
    where C: Clone + std::marker::Send + Sync + 'static {
    pub base_path: String,
    pub user_agent: Option<String>,
    pub client: hyper::client::Client<C>,
    pub basic_auth: Option<BasicAuth>,
    pub oauth_access_token: Option<String>,
    pub api_key: Option<ApiKey>,
    // TODO: take an oauth2 token source, similar to the go one
}

pub type BasicAuth = (String, Option<String>);

pub struct ApiKey {
    pub prefix: Option<String>,
    pub key: String,
}

impl<C: hyper::client::connect::Connect> Configuration<C>
    where C: Clone + std::marker::Send + Sync {
    pub fn new(client: hyper::client::Client<C>) -> Configuration<C> {
        Configuration {
            base_path: "http://api.example.xyz/v1".to_owned(),
            user_agent: Some("OpenAPI-Generator/1.0.0/rust".to_owned()),
            client,
            basic_auth: None,
            oauth_access_token: None,
            api_key: None,
        }
    }
}
