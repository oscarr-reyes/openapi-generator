/*
 * OpenAPI Petstore
 *
 * This is a sample server Petstore server. For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActionContainer {
    #[serde(rename = "action")]
    pub action: Box<models::Baz>,
}

impl ActionContainer {
    pub fn new(action: models::Baz) -> ActionContainer {
        ActionContainer {
            action: Box::new(action),
        }
    }
}


