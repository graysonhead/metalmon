use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use diesel::Connection;

use crypto::scrypt::{scrypt_simple, scrypt_check, ScryptParams};
use crate::models::{User, NewUser, Project, NewProject, ProjectUser, NewProjectUser};

pub fn hash_password(password: &str) -> String {
    let params = ScryptParams::new(1, 8, 1);
    
    scrypt_simple(&password, &params).unwrap()
}

pub fn check_password(conn: &MysqlConnection, target_username: &str, password: &str) -> bool {
    use crate::schema::users::dsl::{users, username};

    let user: User = users
        .filter(username.like(target_username))
        .first(conn)
        .unwrap_or_else(|_| panic!("Unable to find user {}", target_username));
    
    let do_hashes_match = match scrypt_check(password, &user.pw_hash) {
        Ok(bool_result) => bool_result,
        Err(_e) => false
    };
    do_hashes_match
}

pub fn create_user<'a>(conn: &MysqlConnection, username: &'a str, password: &'a str) -> User {
    use crate::schema::users;

    let hash = hash_password(password);

    let new_user = NewUser {
        username: username,
        pw_hash: &hash
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error creating new user");
    
    users::table.order(users::id.desc()).first(conn).unwrap()
}

pub fn delete_user(conn: &MysqlConnection, target_username: &str) -> usize {
    use crate::schema::users::dsl::*;
    let num_deleted = diesel::delete(users.filter(username.like(target_username)))
        .execute(conn)
        .expect("Error deleting user");

    num_deleted
}

pub fn change_user_password<'a>(conn: &MysqlConnection, target_username: &'a str, password: &'a str) {
    use crate::schema::users::dsl::{users, username, pw_hash};
    
    let new_hash = hash_password(&password);
    diesel::update(users.filter(username.like(target_username)))
        .set(pw_hash.eq(&new_hash))
        .execute(conn)
        .unwrap();
}

pub fn list_users(conn: &MysqlConnection) -> Vec<User> {
    use crate::schema::users::dsl::*;

    let results = users
        .load::<User>(conn)
        .expect("Error loading users");

    results
}

pub fn get_user_by_name(conn: &MysqlConnection, target_username: &str) -> User {
    use crate::schema::users::dsl::*;

    let user: User = users
        .filter(username.like(target_username))
        .first(conn)
        .unwrap_or_else(|_| panic!("Unable to find user {}", target_username));
    user
}

pub fn get_project_by_name(conn: &MysqlConnection, target_project: &str) -> Project {
    use crate::schema::projects::dsl::*;

    let project: Project = projects
        .filter(name.like(target_project))
        .first(conn)
        .unwrap_or_else(|_| panic!("Unable to find project {}", target_project));
    project
}

pub fn add_user_to_project(conn: &MysqlConnection, user: User, project: Project) -> ProjectUser {
    use crate::schema::project_users;

    let new_project_user = NewProjectUser {
        user_id: user.id,
        project_id: project.id
    };
    diesel::insert_into(project_users::table)
        .values(&new_project_user)
        .execute(conn)
        .expect("Error creating project user role");
    project_users::table.order(project_users::id.desc()).first(conn).unwrap()
}

pub fn create_project(conn: &MysqlConnection, project_name: &str) -> Project {
    use crate::schema::projects;

    let new_project = NewProject{
        name: project_name
    };

    diesel::insert_into(projects::table)
        .values(&new_project)
        .execute(conn)
        .expect("Error creating new project");
    projects::table.order(projects::id.desc()).first(conn).unwrap()

}

pub fn delete_project(conn: &MysqlConnection, project_name: &str) -> usize {
    use crate::schema::projects::dsl::*;
    let num_deleted = diesel::delete(projects.filter(name.like(project_name)))
        .execute(conn)
        .expect("Error deleting project");
    num_deleted
}

pub fn get_projects(conn: &MysqlConnection) -> Vec<Project> {
    use crate::schema::projects::dsl::*;

    let results = projects
        .load::<Project>(conn)
        .expect("Error loading projects");
    results
}

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL not found");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hash() {
        let password = String::from("password");
        let hash = hash_password(&password);
        let do_hashes_match = match scrypt_check(&password, &hash) {
            Ok(bool_result) => bool_result,
            Err(_e) => false
        };
        assert_eq!(do_hashes_match, true);
    }
}
