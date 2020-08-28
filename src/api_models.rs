use jsonapi::api::*;
use jsonapi::model::*;
use jsonapi::array::JsonApiArray;
use jsonapi::jsonapi_model;

use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiProject {
    pub id: u64,
    pub name: String,
    pub project_users: Option<Vec<ApiProjectUser>>
}
jsonapi_model!(ApiProject; "project"; has many project_users);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiProjectUser {
    pub id: u64,
    pub project_id: u64,
    pub user_id: u64,
    pub view_role: bool,
    pub modify_role: bool,
    pub admin_role: bool,
}
jsonapi_model!(ApiProjectUser; "project_user");
