use jsonapi::api::*;
use jsonapi::model::*;
use jsonapi::array::JsonApiArray;
use jsonapi::jsonapi_model;

use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiProject {
    pub id: u64,
    pub name: String,
    pub project_users: Vec<ApiProjectUser>
}
jsonapi_model!(ApiProject; "project"; has many project_users);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiProjectUser {
    pub id: u64,
    pub user_id: u64,
    pub view_role: bool,
    pub modify_role: bool,
    pub admin_role: bool,
}
jsonapi_model!(ApiProjectUser; "project_user");

// #[macro_use]
// extern crate json_api;

// struct APIProject {
//     id: u64,
//     name: String,
//     project_users: Vec<APIProjectUser>,
// }

// struct APIProjectUser {
//     id: u64,
//     user_id: u64,
//     project: Option<APIProject>,
//     view_role: bool,
//     modify_role: bool,
//     admin_role: bool,
// }

// resource!(APIProject, |&self| {
//     kind "projects",
//     id self.id,

//     attrs id, name;

//     has_many "project_users", {
//         data self.project_users.iter()
//     }
// })
// // use crate::models::{ProjectUsers};
// // use crate::database;
// // use serde::{Serialize, Deserialize};


// // #[derive(Serialize, Deserialize)]
// // pub struct ProjectAttributes {
// //     pub name: String,
// // }


// // #[derive(Serialize, Deserialize)]
// // pub struct APIProjectWrapper {
// //     pub r#type: String,
// //     pub id: u64,
// //     pub attributes: ProjectAttributes,
// //     pub relationships: Vec<ProjectUser>
// // }



// // #[derive(Serialize, Deserialize)]
// // pub struct JsonAPIResponse {
// //     pub data: Vec<APIProjectWrapper>,
// // }

