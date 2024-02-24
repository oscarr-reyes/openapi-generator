/*
 * oneOf test
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "realtype")]
pub enum CustomOneOfSchema {
    #[serde(rename="a-type")]
    AType(Box<models::ObjA>),
    #[serde(rename="b-type")]
    BType(Box<models::ObjB>),
}

impl Default for CustomOneOfSchema {
    fn default() -> Self {
        Self::AType(Box::default())
    }
}


