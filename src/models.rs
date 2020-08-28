use crate::schema::{users, projects, project_users};
use serde::{Serialize, Deserialize};
use diesel::mysql::MysqlConnection;
use crate::database::{
    get_project_users_by_id,
    get_project_by_id,
};
use crate::api_models::{ApiProject, ApiProjectUser};

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize)]
pub struct Project {
    pub id: u64,
    pub name: String,
}

impl Project {

    pub fn convert_project_users(project_users_vec: Vec<ProjectUser>) -> Vec<ApiProjectUser> {
        let mut api_project_users_vec = Vec::new();
        for project_user in project_users_vec {
            api_project_users_vec.push(project_user.get_api_object());
        }
        api_project_users_vec
    }
    pub fn get_api_object(self, conn: Option<&MysqlConnection>) -> ApiProject {
        use crate::models;
        println!("Getting project {}", self.id);
        let api_project_users_vec = match conn {
            Some(conn) => {
                let project_users_vec = get_project_users_by_id(conn, self.id);
                let converted_pu = models::Project::convert_project_users(project_users_vec);
                Some(converted_pu)
            },
            None       => None,
        };
        
        let project_api = ApiProject {
            id: self.id,
            name: self.name,
            project_users: api_project_users_vec
        };
        project_api
    }
}

#[derive(Serialize, Deserialize)]
pub struct Projects {
    pub projects: Vec<Project>
}



#[derive(Insertable)]
#[table_name="projects"]
pub struct NewProject<'a> {
    pub name: &'a str,
}

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize)]
#[belongs_to(Project, foreign_key = "project_id")]
#[belongs_to(User, foreign_key = "user_id")]
pub struct ProjectUser {
    pub id: u64,
    pub user_id: u64,
    pub project_id: u64,
    pub view_role: bool,
    pub modify_role: bool,
    pub admin_role: bool,
}

impl ProjectUser {
    pub fn get_api_object(self) -> ApiProjectUser {
        let project_user_api = ApiProjectUser {
            id: self.id,
            project_id: self.project_id,
            user_id: self.user_id,
            view_role: self.view_role,
            modify_role: self.modify_role,
            admin_role: self.admin_role,
        };
        project_user_api
    }
}

#[derive(Insertable)]
#[table_name="project_users"]
pub struct NewProjectUser {
    pub user_id: u64,
    pub project_id: u64,
    pub view_role: bool,
    pub modify_role: bool,
    pub admin_role: bool,
}

#[derive(Identifiable, Queryable, Associations)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub pw_hash: String
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub pw_hash: &'a str,
}