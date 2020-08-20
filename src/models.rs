use crate::schema::{users, projects, project_users};

#[derive(Identifiable, Queryable, Associations)]
pub struct Project {
    pub id: u64,
    pub name: String
}

#[derive(Insertable)]
#[table_name="projects"]
pub struct NewProject<'a> {
    pub name: &'a str,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Project, foreign_key = "project_id")]
#[belongs_to(User, foreign_key = "user_id")]
pub struct ProjectUser {
    pub id: u64,
    pub user_id: u64,
    pub project_id: u64,
}

#[derive(Insertable)]
#[table_name="project_users"]
pub struct NewProjectUser {
    pub user_id: u64,
    pub project_id: u64,
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